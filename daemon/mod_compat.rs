use std::fs;
use std::path::{Path, PathBuf};
use serde_json::{json, Value};

fn resolve_home() -> PathBuf {
    if let Ok(home) = std::env::var("HOME") { return PathBuf::from(home); }
    PathBuf::from("/root")
}

fn java_candidates() -> Vec<PathBuf> {
    let mut list = Vec::new();
    for d in ["/usr/lib/jvm", "/usr/java", "/opt/java", "/opt/jdk", "/lib/jvm" ] {
        let p = PathBuf::from(d);
        if p.exists() {
            if let Ok(entries) = fs::read_dir(&p) {
                for e in entries.flatten() {
                    let path = e.path();
                    let bin = path.join("bin/java");
                    if bin.exists() { list.push(bin); }
                }
            }
        }
    }
    // Fallbacks comunes
    for b in ["/usr/bin/java", "/bin/java"] {
        let p = PathBuf::from(b);
        if p.exists() { list.push(p); }
    }
    list
}

fn java_major_from_output(out: &str) -> Option<u32> {
    // Detectar "version \"1.8.0_...\"" -> 8, "version \"17.0.8\"" -> 17, "21..." -> 21
    let s = out.to_lowercase();
    if let Some(idx) = s.find("version") {
        let rest = &s[idx..];
        if let Some(q) = rest.find('"') {
            let rem = &rest[q+1..];
            if let Some(q2) = rem.find('"') {
                let ver = &rem[..q2];
                if ver.starts_with("1.8") { return Some(8); }
                if let Some((maj,_)) = ver.split_once('.') {
                    if let Ok(m) = maj.parse::<u32>() { return Some(m); }
                }
            }
        }
    }
    None
}

fn detect_java_versions() -> Vec<(PathBuf,u32)> {
    let mut res = Vec::new();
    for j in java_candidates() {
        let out = std::process::Command::new(&j).arg("-version").output();
        if let Ok(o) = out {
            let text = String::from_utf8_lossy(&o.stderr).to_string() + &String::from_utf8_lossy(&o.stdout);
            if let Some(maj) = java_major_from_output(&text) {
                res.push((j.clone(), maj));
            }
        }
    }
    res
}

fn required_java_for_mc(mc_ver: &str) -> u32 {
    // Simplificación común:
    // <=1.16.x -> 8
    // 1.17.x -> 16
    // 1.18-1.20.4 -> 17
    // >=1.20.5 -> 21
    let v = mc_ver.trim().to_lowercase();
    let mut parts = v.split('.');
    let maj = parts.next().and_then(|s| s.parse::<u32>().ok()).unwrap_or(0);
    let min = parts.next().and_then(|s| s.parse::<u32>().ok()).unwrap_or(0);
    let pat = parts.next().and_then(|s| s.parse::<u32>().ok()).unwrap_or(0);
    if maj == 1 && min <= 16 { return 8; }
    if maj == 1 && min == 17 { return 16; }
    if maj == 1 && min >= 20 && pat >= 5 { return 21; }
    17
}

pub fn parse_cf_manifest(manifest_path: &Path) -> Option<(String,String)> {
    // Devuelve (mc_version, loader) si es posible
    let data = fs::read_to_string(manifest_path).ok()?;
    let v: Value = serde_json::from_str(&data).ok()?;
    let mc_version = v.get("minecraft").and_then(|m| m.get("version")).and_then(|s| s.as_str()).unwrap_or("").to_string();
    let loader = v.get("minecraft").and_then(|m| m.get("modLoaders")).and_then(|l| l.as_array()).and_then(|arr| arr.iter().find(|x| x.get("primary").and_then(|p| p.as_bool()).unwrap_or(false))).and_then(|x| x.get("id")).and_then(|s| s.as_str()).unwrap_or("").to_string();
    Some((mc_version, loader))
}

fn find_instances_dirs() -> Vec<PathBuf> {
    let home = resolve_home();
    let mut dirs = Vec::new();
    // Intentos de CF nativo
    for d in [
        home.join(".local/share/CurseForge/Instances"),
        home.join(".local/share/Overwolf/CurseForge/Instances"),
        home.join(".config/CurseForge/Instances"),
    ] { if d.exists() { dirs.push(d); } }
    // Bottles (Windows)
    let cf_bottle = home.join(".local/share/bottles/bottles/CurseForge/drive_c");
    if cf_bottle.exists() {
        // Buscar Users/*/AppData/Roaming/CurseForge/Minecraft/Instances
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

pub fn unify_curseforge_stores() -> String {
    let dirs = find_instances_dirs();
    if dirs.len() < 2 { return "Nada que unificar (encontrado 0/1 almacenes)".to_string(); }
    // Elegir como canónico el primero de la lista (preferencia nativo)
    let primary = dirs[0].clone();
    let mut actions = Vec::new();
    for d in dirs.iter().skip(1) {
        // si uno está vacío y el otro no, enlazar el vacío al no vacío
        let empty_d = fs::read_dir(&d).map(|mut it| it.next().is_none()).unwrap_or(true);
        let empty_p = fs::read_dir(&primary).map(|mut it| it.next().is_none()).unwrap_or(true);
        if empty_d && !empty_p {
            let _ = fs::remove_dir(&d);
            if std::os::unix::fs::symlink(&primary, &d).is_ok() {
                actions.push(format!("symlink {} -> {}", d.display(), primary.display()));
            }
        } else if !empty_d && empty_p {
            let _ = fs::remove_dir(&primary);
            if std::os::unix::fs::symlink(&d, &primary).is_ok() {
                actions.push(format!("symlink {} -> {}", primary.display(), d.display()));
            }
        } else {
            actions.push(format!("sin cambios entre {} y {} (ambos con contenido)", primary.display(), d.display()));
        }
    }
    if actions.is_empty() { "Sin acciones".to_string() } else { actions.join(" | ") }
}

pub fn validate_common_instances() -> String {
    let mut reports = Vec::new();
    for dir in find_instances_dirs() {
        // Cada instancia suele tener subcarpetas; buscar manifest.json
        if let Ok(entries) = fs::read_dir(&dir) {
            for e in entries.flatten() {
                let p = e.path();
                if p.is_dir() {
                    let manifest = p.join("manifest.json");
                    if manifest.exists() {
                        let rep = validate_modpack_dir(&p);
                        reports.push(rep);
                    }
                }
            }
        }
    }
    json!({"reports": reports}).to_string()
}

pub fn validate_modpack_dir(dir: &Path) -> Value {
    let manifest = dir.join("manifest.json");
    let (mc_ver, loader) = parse_cf_manifest(&manifest).unwrap_or(("".into(), "".into()));
    let required_java = if mc_ver.is_empty() { None } else { Some(required_java_for_mc(&mc_ver)) };
    let javas = detect_java_versions();
    let have_required = required_java.map(|maj| javas.iter().any(|(_,m)| *m == maj)).unwrap_or(false);
    json!({
        "instance": dir.display().to_string(),
        "mc_version": mc_ver,
        "loader": loader,
        "required_java": required_java,
        "java_found": javas.iter().map(|(p,m)| json!({"path": p, "major": m})).collect::<Vec<_>>(),
        "have_required_java": have_required,
    })
}

pub fn prepare_modpack_dir(dir: &Path) -> String {
    // Crear carpetas típicas y permisos básicos
    for sub in ["mods","config","resourcepacks","shaderpacks"] {
        let p = dir.join(sub);
        let _ = fs::create_dir_all(&p);
    }
    // Asegurar permisos de ejecución en scripts sh si existieran
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(entries) = fs::read_dir(dir) {
            for e in entries.flatten() {
                let p = e.path();
                if let Some(ext) = p.extension().and_then(|s| s.to_str()) {
                    if ext == "sh" {
                        if let Ok(mut perm) = fs::metadata(&p).map(|m| m.permissions()) {
                            perm.set_mode(0o755);
                            let _ = fs::set_permissions(&p, perm);
                        }
                    }
                }
            }
        }
    }
    "OK: entorno preparado".to_string()
}
