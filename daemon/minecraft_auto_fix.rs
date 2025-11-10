use serde_json::{json, Value};
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
struct DetectedIssue {
    kind: String,
    details: String,
    pack: Option<String>,
    ns: Option<String>,
    line: String,
}

fn latest_log_path(instance_dir: &Path) -> Option<PathBuf> {
    let p = instance_dir.join("logs/latest.log");
    if p.exists() { return Some(p); }
    // Fallback: algunos perfiles guardan logs en .minecraft/logs
    let p2 = instance_dir.join(".minecraft/logs/latest.log");
    if p2.exists() { return Some(p2); }
    None
}

fn parse_pack_from_line(line: &str) -> Option<String> {
    // Busca patrones "in file/<packname>" y devuelve el packname
    if let Some(idx) = line.find("in file/") {
        let rest = &line[idx + 8..];
        let mut end = rest.len();
        for (i, ch) in rest.char_indices() {
            if ch.is_whitespace() || ch == '-' || ch == '>' { end = i; break; }
        }
        let mut name = rest[..end].trim().to_string();
        // limpia sufijos comunes
        if let Some(pos) = name.find(" ->") { name = name[..pos].to_string(); }
        if name.ends_with(':') { name.pop(); }
        if !name.is_empty() { return Some(name); }
    }
    None
}

fn parse_namespace_from_model_line(line: &str) -> Option<String> {
    // Ejemplo: "Failed to load model forbidden_arcanus:models/block/..."
    if let Some(idx) = line.find("Failed to load model ") {
        let rest = &line[idx + "Failed to load model ".len()..];
        if let Some(col) = rest.find(':') {
            let ns = &rest[..col];
            let ns = ns.trim();
            if !ns.is_empty() { return Some(ns.to_string()); }
        }
    }
    None
}

fn scan_log(instance_dir: &Path) -> Vec<DetectedIssue> {
    let mut issues = Vec::new();
    let Some(log) = latest_log_path(instance_dir) else { return issues; };
    if let Ok(mut f) = fs::File::open(log) {
        let mut content = String::new();
        let _ = f.read_to_string(&mut content);
        for line in content.lines() {
            if line.contains("Error starting SoundSystem") {
                issues.push(DetectedIssue { kind: "sound_error".into(), details: "OpenAL/LWJGL no pudo inicializar audio".into(), pack: None, ns: None, line: line.to_string() });
            }
            if line.contains("{citresewn}") && line.contains("Skipped CIT") {
                let pack = parse_pack_from_line(line);
                issues.push(DetectedIssue { kind: "citresewn_pack_incompatible".into(), details: "CIT con paths no resolubles".into(), pack, ns: None, line: line.to_string() });
            }
            if line.contains("CIT Warning: Unknown enchantment") {
                issues.push(DetectedIssue { kind: "unknown_enchantment".into(), details: "ResourcePack hace referencia a encantamiento de mod ausente".into(), pack: None, ns: None, line: line.to_string() });
            }
            if line.contains("Failed to load model") && line.contains("vertical_slab_template.json") {
                let pack = parse_pack_from_line(line);
                issues.push(DetectedIssue { kind: "missing_model".into(), details: "Modelo faltante (vertical slab compat)".into(), pack, ns: None, line: line.to_string() });
            }
            // Nuevos patrones: modelo faltante de un namespace específico
            if line.contains("Failed to load model ") {
                if let Some(ns) = parse_namespace_from_model_line(line) {
                    issues.push(DetectedIssue { kind: "missing_model_ns".into(), details: "Modelo faltante por namespace".into(), pack: None, ns: Some(ns), line: line.to_string() });
                }
            }
            // Pack con ruta inválida
            if line.contains("Invalid path in pack:") {
                // Formato: Invalid path in pack: <id>:<path>
                if let Some(idx) = line.find("Invalid path in pack:") {
                    let rest = &line[idx + "Invalid path in pack:".len()..];
                    let rest = rest.trim();
                    if let Some(col) = rest.find(':') {
                        let id = rest[..col].trim();
                        let id = id.trim_matches(|c: char| c=='"' || c=='\'');
                        let pack = if id.is_empty() { None } else { Some(id.to_string()) };
                        issues.push(DetectedIssue { kind: "invalid_pack_path".into(), details: "Ruta inválida en pack".into(), pack, ns: None, line: line.to_string() });
                    }
                }
            }
            // Auth/user properties fetch issue
            if line.to_lowercase().contains("failed to fetch user properties") || line.to_lowercase().contains("authentication") {
                issues.push(DetectedIssue { kind: "auth_properties_fail".into(), details: "Fallo al obtener propiedades de usuario (auth/MSA/network)".into(), pack: None, ns: None, line: line.to_string() });
            }
        }
    }
    issues
}

fn ensure_dir(p: &Path) { let _ = fs::create_dir_all(p); }

fn disable_resource_pack(instance_dir: &Path, pack: &str, actions: &mut Vec<String>) {
    let opts = instance_dir.join("options.txt");
    let packs_disabled = instance_dir.join("resourcepacks.disabled");
    ensure_dir(&packs_disabled);

    // 1) Intentar quitarlo de la lista activa en options.txt
    if let Ok(mut text) = fs::read_to_string(&opts) {
        if let Some(idx) = text.find("resourcePacks:") {
            // Encuentra el array JSON [...] en esa línea
            let after = &text[idx..];
            if let Some(start_br) = after.find('[') {
                let start = idx + start_br;
                if let Some(end_rel) = after[start_br..].find(']') {
                    let end = start + end_rel + 1;
                    let json_arr = &text[start..end];
                    if let Ok(mut arr) = serde_json::from_str::<Vec<String>>(json_arr) {
                        let candidates = vec![pack.to_string(), format!("file/{}", pack)];
                        let before_len = arr.len();
                        arr.retain(|e| !candidates.iter().any(|c| e.contains(c)));
                        if arr.len() != before_len {
                            if let Ok(new_json) = serde_json::to_string(&arr) {
                                let mut new_text = text.clone();
                                new_text.replace_range(start..end, &new_json);
                                if fs::write(&opts, &new_text).is_ok() {
                                    actions.push(format!("options.txt: deshabilitado en lista resourcePacks -> {}", pack));
                                    text = new_text; // update local
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // 2) Mover el zip/dir del pack a resourcepacks.disabled si existe
    for candidate in [
        instance_dir.join("resourcepacks").join(pack),
        instance_dir.join("resourcepacks").join(pack).with_extension("")
    ] {
        if candidate.exists() {
            let dest = packs_disabled.join(candidate.file_name().unwrap_or_default());
            if fs::rename(&candidate, &dest).is_ok() {
                actions.push(format!("movido {} -> {}", candidate.display(), dest.display()));
                break;
            }
        }
    }
}

pub fn scan_and_autofix(instance_dir: &Path) -> String {
    let issues = scan_log(instance_dir);
    let mut actions: Vec<String> = Vec::new();
    let mut packs_to_disable: Vec<String> = Vec::new();
    let mut tips: Vec<String> = Vec::new();

    for iss in &issues {
        match iss.kind.as_str() {
            "citresewn_pack_incompatible" | "missing_model" => {
                if let Some(p) = &iss.pack { packs_to_disable.push(p.clone()); }
            }
            "invalid_pack_path" => {
                if let Some(p) = &iss.pack { packs_to_disable.push(p.clone()); }
            }
            "missing_model_ns" => {
                if let Some(ns) = &iss.ns {
                    // Si algún resourcepack en nombres contiene el namespace, deshabilitarlo (heurística no invasiva)
                    if let Ok(entries) = fs::read_dir(instance_dir.join("resourcepacks")) {
                        for e in entries.flatten() {
                            let name = e.file_name().to_string_lossy().to_lowercase();
                            if name.contains(&ns.to_lowercase()) { packs_to_disable.push(name); }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    packs_to_disable.sort();
    packs_to_disable.dedup();
    for p in packs_to_disable { disable_resource_pack(instance_dir, &p, &mut actions); }

    // Sugerencias para audio (no invasivas)
    if issues.iter().any(|i| i.kind == "sound_error") {
        tips.push("Audio: detectado fallo OpenAL. Si ejecutas como root, considera ejecutar CurseForge/Minecraft como usuario normal o habilitar puente de PulseAudio (PULSE_SERVER).".into());
        tips.push("Prueba a actualizar openal-soft/pulseaudio y cerrar apps de audio. Como mitigación, desactiva temporalmente sonidos en opciones.\n".into());
    }
    if issues.iter().any(|i| i.kind == "auth_properties_fail") {
        tips.push("Auth: 'Failed to fetch user properties'. Revisa que iniciaste sesión en CurseForge/Microsoft, que la hora del sistema esté sincronizada (chrony/ntp), y que no haya un firewall/proxy bloqueando api.minecraftservices.com y session.minecraft.net.".into());
    }

    json!({
        "instance": instance_dir.display().to_string(),
        "issues_detected": issues.iter().map(|i| json!({
            "kind": i.kind,
            "details": i.details,
            "pack": i.pack,
            "ns": i.ns,
        })).collect::<Vec<Value>>(),
        "actions_applied": actions,
        "tips": tips,
    }).to_string()
}

fn resolve_home() -> PathBuf {
    if let Ok(home) = std::env::var("HOME") { return PathBuf::from(home); }
    PathBuf::from("/root")
}

fn find_instances_dirs() -> Vec<PathBuf> {
    let home = resolve_home();
    let mut dirs = Vec::new();
    for d in [
        home.join(".local/share/CurseForge/Instances"),
        home.join(".local/share/Overwolf/CurseForge/Instances"),
        home.join(".config/CurseForge/Instances"),
    ] { if d.exists() { dirs.push(d); } }
    // Bottles Windows path
    let cf_bottle = home.join(".local/share/bottles/bottles/CurseForge/drive_c");
    if cf_bottle.exists() {
        let roaming_base = cf_bottle.join("users");
        if let Ok(users) = fs::read_dir(&roaming_base) {
            for u in users.flatten() {
                let inst = u.path().join("AppData/Roaming/CurseForge/Minecraft/Instances");
                if inst.exists() { dirs.push(inst); }
            }
        }
    }
    dirs
}

pub fn autofix_common_instances() -> String {
    let mut reports: Vec<Value> = Vec::new();
    for base in find_instances_dirs() {
        if let Ok(entries) = fs::read_dir(&base) {
            for e in entries.flatten() {
                let p = e.path();
                if p.is_dir() {
                    if p.join("manifest.json").exists() || p.join("instance.cfg").exists() || p.join("minecraftinstance.json").exists() {
                        let rep = scan_and_autofix(&p);
                        if let Ok(v) = serde_json::from_str::<Value>(&rep) { reports.push(v); }
                    }
                }
            }
        }
    }
    json!({ "reports": reports }).to_string()
}
