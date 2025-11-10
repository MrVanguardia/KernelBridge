use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;
use std::fs::File;
use std::io::{Read, Result as IoResult};
use sha2::{Sha256, Digest};

lazy_static! {
    static ref REPORTS: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

/// Endpoint simulado para Epic EOS Anti-Cheat Web API
pub fn eos_report_cheat(player_id: &str, reason: &str) {
    REPORTS.lock().unwrap().insert(player_id.to_string(), reason.to_string());
    println!("[EOS] Reporte de trampa: {} - {}", player_id, reason);
}

/// Endpoint simulado para Steamworks VAC
pub fn vac_apply_ban(steam_id: &str, game: &str) {
    REPORTS.lock().unwrap().insert(steam_id.to_string(), format!("Ban en {}", game));
    println!("[VAC] Ban aplicado a {} en {}", steam_id, game);
}

/// Consultar estado de reporte/baneo
pub fn query_report_status(id: &str) -> Option<String> {
    REPORTS.lock().unwrap().get(id).cloned()
}

/// Calcula el hash SHA-256 real de un archivo
pub fn get_file_hash(path: &str) -> IoResult<String> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 4096];
    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 { break; }
        hasher.update(&buffer[..n]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

// Se necesita una implementación más robusta para leer memoria de otro proceso.
pub fn get_memory_hash(_pid: u32, _address: usize, _size: usize) -> Result<String, String> {
    Ok("dummy_hash_for_now".to_string())
}

pub fn start_ebpf_monitor(_pid: u32) {
    println!("eBPF monitor started (simulated)");
}

pub fn start_ptrace_monitor(_pid: u32) {
    println!("ptrace monitor started (simulated)");
}
