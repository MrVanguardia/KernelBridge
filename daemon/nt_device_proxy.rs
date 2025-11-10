// NT Device Proxy: traductor de IOCTLs lógicos estilo NT a datos reales de Linux
// MVP: dos comandos
//  - GET_PROCESS_LIST -> lista de procesos con forma NT-like
//  - GET_ATTESTATION  -> estado de TPM/IMA/EVM

use std::fs;
use std::path::Path;
use std::collections::HashSet;

#[derive(serde::Serialize)]
struct NtProcessInfo {
    pid: i32,
    ppid: i32,
    name: String,
}

#[derive(serde::Serialize)]
struct NtThreadInfo {
    tid: i32,
    name: Option<String>,
}

#[derive(serde::Serialize)]
struct NtModuleInfo {
    path: String,
    start: Option<u64>,
    end: Option<u64>,
}

#[derive(serde::Serialize)]
struct NtHandleInfo {
    fd: i32,
    target: String,
}

#[derive(serde::Serialize, Clone)]
struct NtMemoryRegion {
    start: u64,
    end: u64,
    perms: String,
    offset: u64,
    dev: String,
    inode: u64,
    path: Option<String>,
}

#[derive(serde::Serialize)]
struct AttestationInfo {
    tpm_available: bool,
    ima_status: Option<String>,
    evm_status: Option<String>,
}

pub fn handle_ioctl_request(
    cmd: &str,
    tpm_available: bool,
    ima_status: Option<String>,
    evm_status: Option<String>,
) -> String {
    // Permitir comandos con argumento, ej: GET_THREAD_LIST:1234
    let (cmd_name, cmd_arg) = if let Some(idx) = cmd.find(':') {
        (&cmd[..idx], Some(cmd[idx + 1..].trim()))
    } else {
        (cmd, None)
    };

    match cmd_name {
        "GET_PROCESS_LIST" => {
            let list = collect_process_list();
            serde_json::to_string(&list).unwrap_or_else(|_| "[]".to_string())
        }
        "GET_ATTESTATION" => {
            let info = AttestationInfo { tpm_available, ima_status, evm_status };
            serde_json::to_string(&info).unwrap_or_else(|_| "{}".to_string())
        }
        "GET_ATTESTATION_EXT" => {
            // Placeholder: extender con PCRs firmadas si se desea
            let info = AttestationInfo { tpm_available, ima_status, evm_status };
            serde_json::json!({
                "attestation": info,
                "pcrs": read_tpm_pcrs(),
            }).to_string()
        }
        "GET_THREAD_LIST" => {
            if let Some(pid_str) = cmd_arg {
                if let Ok(pid) = pid_str.parse::<i32>() {
                    let list = collect_thread_list(pid);
                    return serde_json::to_string(&list).unwrap_or_else(|_| "[]".to_string());
                }
            }
            serde_json::json!({"error": "missing or invalid pid"}).to_string()
        }
        "GET_MODULES" => {
            if let Some(pid_str) = cmd_arg {
                if let Ok(pid) = pid_str.parse::<i32>() {
                    let list = collect_module_list(pid);
                    return serde_json::to_string(&list).unwrap_or_else(|_| "[]".to_string());
                }
            }
            serde_json::json!({"error": "missing or invalid pid"}).to_string()
        }
        "GET_HANDLE_TABLE" => {
            if let Some(pid_str) = cmd_arg {
                if let Ok(pid) = pid_str.parse::<i32>() {
                    let list = collect_handle_list(pid);
                    return serde_json::to_string(&list).unwrap_or_else(|_| "[]".to_string());
                }
            }
            serde_json::json!({"error": "missing or invalid pid"}).to_string()
        }
        "GET_PROCESS_MEMORY_MAP" => {
            if let Some(pid_str) = cmd_arg {
                if let Ok(pid) = pid_str.parse::<i32>() {
                    let list = collect_memory_map(pid);
                    return serde_json::to_string(&list).unwrap_or_else(|_| "[]".to_string());
                }
            }
            serde_json::json!({"error": "missing or invalid pid"}).to_string()
        }
        "CHECK_PROCESS_SECURITY" => {
            if let Some(pid_str) = cmd_arg {
                if let Ok(pid) = pid_str.parse::<i32>() {
                    let report = check_process_security(pid);
                    return serde_json::to_string(&report).unwrap_or_else(|_| "{}".to_string());
                }
            }
            serde_json::json!({"error": "missing or invalid pid"}).to_string()
        }
        "CHECK_DEBUG" => {
            if let Some(pid_str) = cmd_arg {
                if let Ok(pid) = pid_str.parse::<i32>() {
                    let tracer = read_tracer_pid(pid);
                    return serde_json::json!({
                        "pid": pid,
                        "tracer_pid": tracer,
                        "is_debugged": tracer.unwrap_or(0) != 0
                    }).to_string();
                }
            }
            serde_json::json!({"error": "missing or invalid pid"}).to_string()
        }
        "CHECK_SANDBOX_VM" => {
            let info = check_sandbox_vm();
            serde_json::to_string(&info).unwrap_or_else(|_| "{}".to_string())
        }
        "CHECK_MULTICLIENT" => {
            if let Some(name) = cmd_arg { return serde_json::json!({"name": name, "count": count_process_by_name(name)}).to_string(); }
            serde_json::json!({"error": "missing name"}).to_string()
        }
        other => {
            serde_json::json!({ "error": format!("unknown command: {}", other) }).to_string()
        }
    }
}

fn collect_process_list() -> Vec<NtProcessInfo> {
    let mut out = Vec::new();
    let proc_dir = Path::new("/proc");
    if let Ok(entries) = fs::read_dir(proc_dir) {
        for e in entries.flatten() {
            let name = e.file_name();
            let name_str = name.to_string_lossy();
            if let Ok(pid) = name_str.parse::<i32>() { // solo directorios numéricos
                if let Some((ppid, comm)) = read_stat(pid) {
                    out.push(NtProcessInfo { pid, ppid, name: comm });
                }
            }
        }
    }
    out
}

fn read_stat(pid: i32) -> Option<(i32, String)> {
    let path = format!("/proc/{}/stat", pid);
    let content = fs::read_to_string(&path).ok()?;
    // /proc/[pid]/stat: pid (1) comm (2) state (3) ppid (4) ...
    // comm puede contener espacios, viene entre paréntesis
    // ejemplo: 1234 (my proc) R 1 ...
    let open = content.find('(')?;
    let close = content.rfind(')')?;
    let comm = content[(open + 1)..close].to_string();
    let after = &content[(close + 1)..];
    let parts: Vec<&str> = after.split_whitespace().collect();
    if parts.len() >= 2 {
        let ppid = parts[1].parse::<i32>().ok()?; // ppid es el 4to campo total
        return Some((ppid, comm));
    }
    None
}

fn collect_thread_list(pid: i32) -> Vec<NtThreadInfo> {
    let mut out = Vec::new();
    let path = format!("/proc/{}/task", pid);
    if let Ok(entries) = fs::read_dir(path) {
        for e in entries.flatten() {
            if let Ok(tid) = e.file_name().to_string_lossy().parse::<i32>() {
                let comm = fs::read_to_string(format!("/proc/{}/task/{}/comm", pid, tid)).ok().map(|s| s.trim().to_string());
                out.push(NtThreadInfo { tid, name: comm });
            }
        }
    }
    out
}

fn collect_module_list(pid: i32) -> Vec<NtModuleInfo> {
    let mut set: HashSet<String> = HashSet::new();
    let mut out = Vec::new();
    let path = format!("/proc/{}/maps", pid);
    if let Ok(content) = fs::read_to_string(path) {
        for line in content.lines() {
            // formato: start-end perms offset dev inode path
            let mut parts = line.split_whitespace();
            let range = parts.next().unwrap_or("");
            let perms = parts.next();
            let _offset = parts.next();
            let _dev = parts.next();
            let _inode = parts.next();
            let pth = parts.next();
            if let Some(path_str) = pth {
                // filtrar mappings anónimos
                if path_str.starts_with('/') {
                    if set.insert(path_str.to_string()) {
                        if let Some((s, e)) = parse_range(range) {
                            out.push(NtModuleInfo { path: path_str.to_string(), start: Some(s), end: Some(e) });
                        } else {
                            out.push(NtModuleInfo { path: path_str.to_string(), start: None, end: None });
                        }
                    }
                }
            }
        }
    }
    out
}

fn collect_handle_list(pid: i32) -> Vec<NtHandleInfo> {
    let mut out = Vec::new();
    let path = format!("/proc/{}/fd", pid);
    if let Ok(entries) = fs::read_dir(path) {
        for e in entries.flatten() {
            let name = e.file_name().to_string_lossy().to_string();
            if let Ok(fd) = name.parse::<i32>() {
                let target = fs::read_link(e.path()).map(|p| p.to_string_lossy().to_string()).unwrap_or_else(|_| String::from("?"));
                out.push(NtHandleInfo { fd, target });
            }
        }
    }
    out
}

fn collect_memory_map(pid: i32) -> Vec<NtMemoryRegion> {
    let mut out = Vec::new();
    let path = format!("/proc/{}/maps", pid);
    if let Ok(content) = fs::read_to_string(path) {
        for line in content.lines() {
            let mut parts = line.split_whitespace();
            let range = parts.next().unwrap_or("");
            let perms = parts.next().unwrap_or("").to_string();
            let offset = parts.next().and_then(|s| u64::from_str_radix(s, 16).ok()).unwrap_or(0);
            let dev = parts.next().unwrap_or("").to_string();
            let inode = parts.next().and_then(|s| s.parse::<u64>().ok()).unwrap_or(0);
            let path_opt = parts.next().map(|s| s.to_string());
            if let Some((start, end)) = parse_range(range) {
                out.push(NtMemoryRegion { start, end, perms, offset, dev, inode, path: path_opt });
            }
        }
    }
    out
}

fn parse_range(r: &str) -> Option<(u64, u64)> {
    let mut it = r.split('-');
    let a = it.next().and_then(|s| u64::from_str_radix(s, 16).ok())?;
    let b = it.next().and_then(|s| u64::from_str_radix(s, 16).ok())?;
    Some((a, b))
}

fn read_tpm_pcrs() -> Option<String> {
    // Placeholder simple: si tpm2_pcrread está disponible, devolver salida bruta
    std::process::Command::new("tpm2_pcrread")
        .output()
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
}

#[derive(serde::Serialize)]
struct ProcSecurityReport {
    pid: i32,
    has_wx_pages: bool,
    has_anon_exec: bool,
    has_memfd_exec: bool,
    is_debugged: bool,
    tracer_pid: Option<i32>,
    suspicious_regions: Vec<NtMemoryRegion>,
}

fn check_process_security(pid: i32) -> ProcSecurityReport {
    let maps = collect_memory_map(pid);
    let mut has_wx = false;
    let mut has_anon_exec = false;
    let mut has_memfd_exec = false;
    let mut suspicious = Vec::new();
    for m in &maps {
        let exec = m.perms.contains('x');
        let write = m.perms.contains('w');
        let path = m.path.clone().unwrap_or_default();
        if exec && write { has_wx = true; suspicious.push(m.clone()); }
        if exec {
            if path.is_empty() || path == "[anon]" || path.starts_with('[') { has_anon_exec = true; suspicious.push(m.clone()); }
            if path.starts_with("/memfd:") { has_memfd_exec = true; suspicious.push(m.clone()); }
        }
    }
    let tracer = read_tracer_pid(pid);
    ProcSecurityReport {
        pid,
        has_wx_pages: has_wx,
        has_anon_exec,
        has_memfd_exec,
        is_debugged: tracer.unwrap_or(0) != 0,
        tracer_pid: tracer,
        suspicious_regions: dedup_regions(suspicious),
    }
}

fn read_tracer_pid(pid: i32) -> Option<i32> {
    let path = format!("/proc/{}/status", pid);
    if let Ok(txt) = fs::read_to_string(path) {
        for line in txt.lines() {
            if line.starts_with("TracerPid:") {
                return line[10..].trim().parse::<i32>().ok();
            }
        }
    }
    None
}

fn dedup_regions(mut v: Vec<NtMemoryRegion>) -> Vec<NtMemoryRegion> {
    v.sort_by_key(|r| (r.start, r.end));
    v.dedup_by(|a, b| a.start == b.start && a.end == b.end && a.perms == b.perms && a.path == b.path);
    v
}

#[derive(serde::Serialize)]
struct SandboxVmInfo {
    hypervisor_flag: bool,
    dmi_vendor: Option<String>,
    dmi_product: Option<String>,
    is_vm: bool,
}

fn check_sandbox_vm() -> SandboxVmInfo {
    let hypervisor_flag = fs::read_to_string("/proc/cpuinfo").map(|t| t.contains("hypervisor")).unwrap_or(false);
    let dmi_vendor = fs::read_to_string("/sys/class/dmi/id/sys_vendor").ok().map(|s| s.trim().to_string());
    let dmi_product = fs::read_to_string("/sys/class/dmi/id/product_name").ok().map(|s| s.trim().to_string());
    let is_vm = hypervisor_flag || dmi_vendor.as_deref().map(|v| v.to_lowercase().contains("qemu") || v.to_lowercase().contains("vmware") || v.to_lowercase().contains("microsoft")).unwrap_or(false);
    SandboxVmInfo { hypervisor_flag, dmi_vendor, dmi_product, is_vm }
}

fn count_process_by_name(name: &str) -> usize {
    let needle = name.to_lowercase();
    let mut count = 0;
    let proc_dir = Path::new("/proc");
    if let Ok(entries) = fs::read_dir(proc_dir) {
        for e in entries.flatten() {
            let fname = e.file_name();
            if let Ok(_pid) = fname.to_string_lossy().parse::<i32>() {
                if let Some((_ppid, comm)) = read_stat(_pid) {
                    if comm.to_lowercase().contains(&needle) { count += 1; }
                }
            }
        }
    }
    count
}
