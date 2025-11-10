use serde_json::{json, Value};
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

use crate::mod_compat::parse_cf_manifest;

#[derive(Debug, Clone)]
struct MissingSuggest {
    modid: String,
    source: String,     // "modrinth" | "curseforge"
    project_id: String, // modrinth project id OR curseforge modId
    slug: String,
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

fn latest_log_path(instance_dir: &Path) -> Option<PathBuf> {
    for p in [
        instance_dir.join("logs/latest.log"),
        instance_dir.join(".minecraft/logs/latest.log"),
    ] { if p.exists() { return Some(p); } }
    None
}

pub fn parse_missing_modids_from_log(instance_dir: &Path) -> Vec<String> {
    let mut ids: Vec<String> = Vec::new();
    let Some(logp) = latest_log_path(instance_dir) else { return ids; };
    if let Ok(mut f) = fs::File::open(logp) {
        let mut s = String::new();
        let _ = f.read_to_string(&mut s);
        let lines: Vec<&str> = s.lines().collect();
        let mut in_missing_block = false;
        for line in lines {
            let l = line.to_lowercase();
            if l.contains("missing mods:") || l.contains("missing mod:") {
                in_missing_block = true;
                continue;
            }
            if in_missing_block {
                if l.trim().is_empty() { in_missing_block = false; continue; }
                // Formatos comunes: "- Some Name (modid)", "- modid"
                let mut cand = None;
                if let Some(start) = line.find('(') {
                    if let Some(end) = line[start+1..].find(')') { cand = Some(line[start+1..start+1+end].to_string()); }
                }
                if cand.is_none() {
                    let t = line.trim().trim_start_matches('-').trim();
                    if !t.is_empty() { cand = Some(t.to_string()); }
                }
                if let Some(x) = cand { if x.chars().all(|c| c.is_ascii_alphanumeric() || c=='_' || c=='-' ) { ids.push(x); } }
                continue;
            }
            // Fabric: "Could not find required mod: <modid>"
            if let Some(idx) = l.find("could not find required mod:") {
                let rest = line[idx+"could not find required mod:".len()..].trim();
                let modid = rest.split_whitespace().next().unwrap_or("").trim_matches(|c: char| c==',' || c=='.');
                if !modid.is_empty() { ids.push(modid.to_string()); }
            }
            // Forge newer: "requires {modid @...}" or "requires [modid]"
            if let Some(i) = l.find(" requires {") {
                if let Some(j) = line[i..].find('}') { let inside = &line[i+10..i+j]; for token in inside.split(',') { let id = token.trim().split_whitespace().next().unwrap_or(""); if !id.is_empty() { ids.push(id.to_string()); } } }
            }
        }
    }
    ids.sort(); ids.dedup();
    ids
}

fn ensure_dir(p: &Path) { let _ = fs::create_dir_all(p); }

fn loader_from_manifest(dir: &Path) -> String {
    let m = dir.join("manifest.json");
    if let Some((_mc, loader)) = parse_cf_manifest(&m) { return loader; }
    String::new()
}

fn mc_version_from_manifest(dir: &Path) -> String {
    let m = dir.join("manifest.json");
    if m.exists() {
        if let Ok(txt) = fs::read_to_string(&m) {
            if let Ok(v) = serde_json::from_str::<Value>(&txt) {
                if let Some(ver) = v.get("minecraft").and_then(|m| m.get("version")).and_then(|x| x.as_str()) { return ver.to_string(); }
            }
        }
    }
    String::new()
}

fn modrinth_search_project(modid: &str) -> Option<(String,String)> {
    let url = format!("https://api.modrinth.com/v2/search?limit=5&query={}", urlencoding::encode(modid));
    let client = reqwest::blocking::Client::builder().user_agent("KernelBridge/0.1").build().ok()?;
    let res = client.get(url).send().ok()?.json::<Value>().ok()?;
    let hits = res.get("hits")?.as_array()?;
    for h in hits {
        let pid = h.get("project_id").and_then(|x| x.as_str()).unwrap_or("");
        let slug = h.get("slug").and_then(|x| x.as_str()).unwrap_or("");
        let ptype = h.get("project_type").and_then(|x| x.as_str()).unwrap_or("");
        if ptype == "mod" && !pid.is_empty() { return Some((pid.to_string(), slug.to_string())); }
    }
    None
}

fn modrinth_pick_version(project_id: &str, loader: &str, mc_ver: &str) -> Option<(String,String)> {
    let mut loaders: Vec<&str> = Vec::new();
    if loader.to_lowercase().contains("fabric") { loaders.push("fabric"); }
    if loader.to_lowercase().contains("forge") { loaders.push("forge"); }
    if loaders.is_empty() { loaders = vec!["fabric","forge"]; }
    let url = format!(
        "https://api.modrinth.com/v2/project/{}/version?loaders={}&game_versions={}",
        project_id,
        urlencoding::encode(&serde_json::to_string(&loaders).ok()? ),
        urlencoding::encode(&serde_json::to_string(&vec![mc_ver]).ok()? )
    );
    let client = reqwest::blocking::Client::builder().user_agent("KernelBridge/0.1").build().ok()?;
    let res = client.get(url).send().ok()?.json::<Value>().ok()?;
    let arr = res.as_array()?;
    // Prefer release
    for v in arr {
        if v.get("version_type").and_then(|x| x.as_str()) == Some("release") {
            if let Some(files) = v.get("files").and_then(|x| x.as_array()) { if let Some(f) = files.first() {
                let url = f.get("url").and_then(|x| x.as_str()).unwrap_or("");
                let filename = f.get("filename").and_then(|x| x.as_str()).unwrap_or("");
                if !url.is_empty() && !filename.is_empty() { return Some((filename.to_string(), url.to_string())); }
            } }
        }
    }
    // Fallback a la primera
    if let Some(v) = arr.first() {
        if let Some(files) = v.get("files").and_then(|x| x.as_array()) { if let Some(f) = files.first() {
            let url = f.get("url").and_then(|x| x.as_str()).unwrap_or("");
            let filename = f.get("filename").and_then(|x| x.as_str()).unwrap_or("");
            if !url.is_empty() && !filename.is_empty() { return Some((filename.to_string(), url.to_string())); }
        } }
    }
    None
}

pub fn suggest_missing_mods_dir(dir: &Path) -> String {
    let missing = parse_missing_modids_from_log(dir);
    let loader = loader_from_manifest(dir);
    let mc_ver = mc_version_from_manifest(dir);

    let mut suggestions: Vec<Value> = Vec::new();
    for modid in &missing {
        if let Some((pid, slug)) = modrinth_search_project(modid) {
            suggestions.push(json!({
                "modid": modid,
                "source": "modrinth",
                "project_id": pid,
                "slug": slug,
            }));
        } else if let Some((cf_id, cf_slug)) = curseforge_search_project(modid) {
            suggestions.push(json!({
                "modid": modid,
                "source": "curseforge",
                "project_id": cf_id,
                "slug": cf_slug,
            }));
        }
    }

    json!({
        "instance": dir.display().to_string(),
        "mc_version": mc_ver,
        "loader": loader,
        "missing_modids": missing,
        "suggestions": suggestions,
    }).to_string()
}

pub fn install_suggested_mods_dir(dir: &Path) -> String {
    let loader = loader_from_manifest(dir);
    let mc_ver = mc_version_from_manifest(dir);
    let report = suggest_missing_mods_dir(dir);
    let value: Value = serde_json::from_str(&report).unwrap_or(json!({"missing_modids": [], "suggestions": []}));
    let sugg = value.get("suggestions").and_then(|x| x.as_array()).cloned().unwrap_or_default();

    let mods_dir = dir.join("mods");
    ensure_dir(&mods_dir);

    let client = reqwest::blocking::Client::builder().user_agent("KernelBridge/0.1").build();
    let client = match client { Ok(c) => c, Err(_) => return json!({"error": "client"}).to_string() };

    let mut results: Vec<Value> = Vec::new();
    for s in sugg {
        let modid = s.get("modid").and_then(|x| x.as_str()).unwrap_or("").to_string();
        let pid = s.get("project_id").and_then(|x| x.as_str()).unwrap_or("").to_string();
        let source = s.get("source").and_then(|x| x.as_str()).unwrap_or("");
        if pid.is_empty() { continue; }
        let file_url: Option<(String,String)> = if source == "curseforge" {
            curseforge_pick_file(&pid, &loader, &mc_ver)
        } else {
            modrinth_pick_version(&pid, &loader, &mc_ver)
        };
        if let Some((filename, url)) = file_url {
            let target = mods_dir.join(&filename);
            if target.exists() {
                results.push(json!({"modid": modid, "status": "exists", "file": filename}));
                continue;
            }
            let resp = client.get(&url).send();
            match resp {
                Ok(mut r) => {
                    if r.status().is_success() {
                        let bytes = r.bytes();
                        match bytes {
                            Ok(b) => {
                                if fs::write(&target, &b).is_ok() {
                                    results.push(json!({"modid": modid, "status": "installed", "file": filename}));
                                } else {
                                    results.push(json!({"modid": modid, "status": "write_error", "file": filename}));
                                }
                            }
                            Err(_) => results.push(json!({"modid": modid, "status": "download_read_error", "file": filename})),
                        }
                    } else {
                        results.push(json!({"modid": modid, "status": "http_error", "code": r.status().as_u16(), "file": filename}));
                    }
                }
                Err(_) => results.push(json!({"modid": modid, "status": "network_error"})),
            }
        } else {
            results.push(json!({"modid": modid, "status": "no_version_match"}));
        }
    }

    json!({
        "instance": dir.display().to_string(),
        "mc_version": mc_ver,
        "loader": loader,
        "results": results,
    }).to_string()
}

// ===== CurseForge API support (requires CF_API_KEY env) =====

fn curseforge_key() -> Option<String> {
    std::env::var("CF_API_KEY").ok().filter(|s| !s.is_empty())
}

fn curseforge_loader_code(loader: &str) -> i32 {
    let l = loader.to_lowercase();
    if l.contains("fabric") { 4 } else if l.contains("forge") { 1 } else if l.contains("quilt") { 5 } else { 0 }
}

fn curseforge_search_project(modid: &str) -> Option<(String,String)> {
    let key = curseforge_key()?;
    let url = format!("https://api.curseforge.com/v1/mods/search?gameId=432&searchFilter={}", urlencoding::encode(modid));
    let client = reqwest::blocking::Client::builder().user_agent("KernelBridge/0.1").build().ok()?;
    let res = client.get(url).header("x-api-key", key).send().ok()?.json::<Value>().ok()?;
    let arr = res.get("data")?.as_array()?;
    for m in arr {
        if let Some(id) = m.get("id").and_then(|x| x.as_i64()) {
            let slug = m.get("slug").and_then(|x| x.as_str()).unwrap_or("").to_string();
            return Some((id.to_string(), slug));
        }
    }
    None
}

fn curseforge_pick_file(project_id: &str, loader: &str, mc_ver: &str) -> Option<(String,String)> {
    let key = curseforge_key()?;
    let loader_code = curseforge_loader_code(loader);
    let client = reqwest::blocking::Client::builder().user_agent("KernelBridge/0.1").build().ok()?;
    let url = if loader_code != 0 {
        format!("https://api.curseforge.com/v1/mods/{}/files?gameVersion={}&modLoaderType={}", project_id, urlencoding::encode(mc_ver), loader_code)
    } else {
        format!("https://api.curseforge.com/v1/mods/{}/files?gameVersion={}", project_id, urlencoding::encode(mc_ver))
    };
    let res = client.get(url).header("x-api-key", &key).send().ok()?.json::<Value>().ok()?;
    let files = res.get("data")?.as_array()?;
    // Prefer releaseType == 1
    let mut cand = None;
    for f in files {
        if f.get("releaseType").and_then(|x| x.as_i64()) == Some(1) { cand = Some(f.clone()); break; }
    }
    let f = cand.or_else(|| files.first().cloned())?;
    let file_id = f.get("id").and_then(|x| x.as_i64())?;
    let filename = f.get("fileName").and_then(|x| x.as_str())?.to_string();
    // Obtain download URL
    let url2 = format!("https://api.curseforge.com/v1/mods/{}/files/{}/download-url", project_id, file_id);
    let res2 = client.get(url2).header("x-api-key", &key).send().ok()?.json::<Value>().ok()?;
    let dl = res2.get("data").and_then(|x| x.as_str())?.to_string();
    Some((filename, dl))
}

pub fn suggest_missing_mods_cf_dir(dir: &Path) -> String {
    let missing = parse_missing_modids_from_log(dir);
    let mut suggestions: Vec<Value> = Vec::new();
    for modid in &missing {
        if let Some((cf_id, cf_slug)) = curseforge_search_project(modid) {
            suggestions.push(json!({
                "modid": modid,
                "source": "curseforge",
                "project_id": cf_id,
                "slug": cf_slug,
            }));
        }
    }
    json!({
        "instance": dir.display().to_string(),
        "missing_modids": missing,
        "suggestions": suggestions,
    }).to_string()
}

pub fn install_suggested_mods_cf_dir(dir: &Path) -> String {
    let loader = loader_from_manifest(dir);
    let mc_ver = mc_version_from_manifest(dir);
    let report = suggest_missing_mods_cf_dir(dir);
    let value: Value = serde_json::from_str(&report).unwrap_or(json!({"missing_modids": [], "suggestions": []}));
    let sugg = value.get("suggestions").and_then(|x| x.as_array()).cloned().unwrap_or_default();
    let mods_dir = dir.join("mods");
    ensure_dir(&mods_dir);
    let client = reqwest::blocking::Client::builder().user_agent("KernelBridge/0.1").build();
    let client = match client { Ok(c) => c, Err(_) => return json!({"error": "client"}).to_string() };
    let mut results: Vec<Value> = Vec::new();
    for s in sugg {
        let modid = s.get("modid").and_then(|x| x.as_str()).unwrap_or("").to_string();
        let pid = s.get("project_id").and_then(|x| x.as_str()).unwrap_or("").to_string();
        if pid.is_empty() { continue; }
        if let Some((filename, url)) = curseforge_pick_file(&pid, &loader, &mc_ver) {
            let target = mods_dir.join(&filename);
            if target.exists() { results.push(json!({"modid": modid, "status": "exists", "file": filename})); continue; }
            let resp = client.get(&url).send();
            match resp {
                Ok(r) => {
                    if r.status().is_success() {
                        if let Ok(b) = r.bytes() {
                            if fs::write(&target, &b).is_ok() {
                                results.push(json!({"modid": modid, "status": "installed", "file": filename}));
                            } else { results.push(json!({"modid": modid, "status": "write_error", "file": filename})); }
                        } else { results.push(json!({"modid": modid, "status": "download_read_error", "file": filename})); }
                    } else {
                        results.push(json!({"modid": modid, "status": "http_error", "code": r.status().as_u16(), "file": filename}));
                    }
                }
                Err(_) => results.push(json!({"modid": modid, "status": "network_error"})),
            }
        } else {
            results.push(json!({"modid": modid, "status": "no_version_match"}));
        }
    }
    json!({
        "instance": dir.display().to_string(),
        "mc_version": mc_ver,
        "loader": loader,
        "results": results,
    }).to_string()
}

pub fn suggest_missing_mods_cf_common() -> String {
    let mut reports: Vec<Value> = Vec::new();
    for base in find_instances_dirs() {
        if let Ok(entries) = fs::read_dir(&base) {
            for e in entries.flatten() {
                let p = e.path();
                if p.is_dir() {
                    if p.join("manifest.json").exists() || p.join("minecraftinstance.json").exists() {
                        let rep = suggest_missing_mods_cf_dir(&p);
                        if let Ok(v) = serde_json::from_str::<Value>(&rep) { reports.push(v); }
                    }
                }
            }
        }
    }
    json!({"reports": reports}).to_string()
}

pub fn install_suggested_mods_cf_common() -> String {
    let mut reports: Vec<Value> = Vec::new();
    for base in find_instances_dirs() {
        if let Ok(entries) = fs::read_dir(&base) {
            for e in entries.flatten() {
                let p = e.path();
                if p.is_dir() {
                    if p.join("manifest.json").exists() || p.join("minecraftinstance.json").exists() {
                        let rep = install_suggested_mods_cf_dir(&p);
                        if let Ok(v) = serde_json::from_str::<Value>(&rep) { reports.push(v); }
                    }
                }
            }
        }
    }
    json!({"reports": reports}).to_string()
}

pub fn suggest_missing_mods_common() -> String {
    let mut reports: Vec<Value> = Vec::new();
    for base in find_instances_dirs() {
        if let Ok(entries) = fs::read_dir(&base) {
            for e in entries.flatten() {
                let p = e.path();
                if p.is_dir() {
                    if p.join("manifest.json").exists() || p.join("minecraftinstance.json").exists() {
                        let rep = suggest_missing_mods_dir(&p);
                        if let Ok(v) = serde_json::from_str::<Value>(&rep) { reports.push(v); }
                    }
                }
            }
        }
    }
    json!({"reports": reports}).to_string()
}

pub fn install_suggested_mods_common() -> String {
    let mut reports: Vec<Value> = Vec::new();
    for base in find_instances_dirs() {
        if let Ok(entries) = fs::read_dir(&base) {
            for e in entries.flatten() {
                let p = e.path();
                if p.is_dir() {
                    if p.join("manifest.json").exists() || p.join("minecraftinstance.json").exists() {
                        let rep = install_suggested_mods_dir(&p);
                        if let Ok(v) = serde_json::from_str::<Value>(&rep) { reports.push(v); }
                    }
                }
            }
        }
    }
    json!({"reports": reports}).to_string()
}
