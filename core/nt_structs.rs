
// Estructuras NT simuladas con datos reales
// EPROCESS, HANDLE_TABLE, OBJECT_HEADER, CRITICAL_SECTION, MUTEX, SEMAPHORE, SECTION, THREAD, MEMORY_REGION, etc.

use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct CriticalSection {
    pub debug_info: u64,
    pub lock_count: i32,
    pub recursion_count: i32,
    pub owning_thread: u64,
    pub lock_semaphore: u64,
    pub spin_count: u32,
}

impl CriticalSection {
    pub fn new() -> Self {
        CriticalSection {
            debug_info: 0,
            lock_count: 0,
            recursion_count: 0,
            owning_thread: 0,
            lock_semaphore: 0,
            spin_count: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Mutex {
    pub owner: u64,
    pub count: i32,
    pub abandoned: bool,
}

impl Mutex {
    pub fn new() -> Self {
        Mutex {
            owner: 0,
            count: 1,
            abandoned: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Semaphore {
    pub count: i32,
    pub max_count: i32,
}

impl Semaphore {
    pub fn new(max_count: i32) -> Self {
        Semaphore {
            count: max_count,
            max_count,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Section {
    pub base_address: u64,
    pub size: u64,
    pub protection: u32,
}

impl Section {
    pub fn new(base_address: u64, size: u64, protection: u32) -> Self {
        Section { base_address, size, protection }
    }
}

#[derive(Debug, Clone)]
pub struct MemoryRegion {
    pub base_address: u64,
    pub size: u64,
    pub protection: u32,
    pub state: String,
}

impl MemoryRegion {
    pub fn new(base_address: u64, size: u64, protection: u32, state: &str) -> Self {
        MemoryRegion { base_address, size, protection, state: state.to_string() }
    }
}

/// Utilidades para obtener todas las regiones de memoria de un proceso
pub fn get_memory_regions(pid: u32) -> Vec<MemoryRegion> {
    let mut regions = Vec::new();
    let maps_path = format!("/proc/{}/maps", pid);
    if let Ok(maps) = fs::read_to_string(&maps_path) {
        for line in maps.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let addr_range = parts[0];
            let prot = parts[1];
            let state = if parts.len() > 5 { parts[5] } else { "" };
            let addrs: Vec<&str> = addr_range.split('-').collect();
            if addrs.len() == 2 {
                let base_address = u64::from_str_radix(addrs[0], 16).unwrap_or(0);
                let end_address = u64::from_str_radix(addrs[1], 16).unwrap_or(0);
                let size = end_address.saturating_sub(base_address);
                let protection = match prot {
                    "r--p" => 1,
                    "rw-p" => 2,
                    "r-xp" => 4,
                    _ => 0,
                };
                regions.push(MemoryRegion::new(base_address, size, protection, state));
            }
        }
    }
    regions
}

#[derive(Debug, Clone)]
pub struct EProcess {
    pub pid: u32,
    pub name: String,
    pub ppid: u32,
    pub threads: u32,
    pub start_time: u64,
    pub uid: u32,
    pub gid: u32,
    pub memory_usage: u64,
    pub state: String,
}

impl EProcess {
    /// Construye EPROCESS con datos reales de /proc/[pid]/stat y /proc/[pid]/status
    pub fn from_pid(pid: u32) -> Option<Self> {
        let stat_path = format!("/proc/{}/stat", pid);
        let status_path = format!("/proc/{}/status", pid);
        if let Ok(stat) = fs::read_to_string(&stat_path) {
            let parts: Vec<&str> = stat.split_whitespace().collect();
            if parts.len() > 21 {
                let name = parts[1].trim_matches('(').trim_matches(')').to_string();
                let ppid = parts[3].parse().unwrap_or(0);
                let threads = parts[19].parse().unwrap_or(0);
                let start_time = parts[21].parse().unwrap_or(0);
                let state = parts[2].to_string();
                let mut uid = 0;
                let mut gid = 0;
                let mut memory_usage = 0;
                if let Ok(status) = fs::read_to_string(&status_path) {
                    for line in status.lines() {
                        if line.starts_with("Uid:") {
                            uid = line.split_whitespace().nth(1).unwrap_or("0").parse().unwrap_or(0);
                        } else if line.starts_with("Gid:") {
                            gid = line.split_whitespace().nth(1).unwrap_or("0").parse().unwrap_or(0);
                        } else if line.starts_with("VmSize:") {
                            memory_usage = line.split_whitespace().nth(1).unwrap_or("0").parse().unwrap_or(0);
                        }
                    }
                }
                return Some(EProcess { pid, name, ppid, threads, start_time, uid, gid, memory_usage, state });
            }
        }
        None
    }
}

#[derive(Debug, Clone)]
pub struct HandleTable {
    pub pid: u32,
    pub handles: Vec<String>,
}

impl HandleTable {
    /// Construye HANDLE_TABLE con datos reales de /proc/[pid]/fd
    pub fn from_pid(pid: u32) -> Option<Self> {
        let fd_path = format!("/proc/{}/fd", pid);
        if Path::new(&fd_path).exists() {
            let mut handles = Vec::new();
            if let Ok(entries) = fs::read_dir(&fd_path) {
                for entry in entries.flatten() {
                    if let Ok(target) = fs::read_link(entry.path()) {
                        handles.push(target.to_string_lossy().to_string());
                    }
                }
            }
            return Some(HandleTable { pid, handles });
        }
        None
    }
}

#[derive(Debug, Clone)]
pub struct ObjectHeader {
    pub object_type: String,
    pub handle_count: u32,
    pub flags: u32,
}

impl ObjectHeader {
    pub fn new(object_type: &str, handle_count: u32, flags: u32) -> Self {
        ObjectHeader {
            object_type: object_type.to_string(),
            handle_count,
            flags,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EThread {
    pub tid: u32,
    pub owner_pid: u32,
    pub state: String,
}

impl EThread {
    /// Construye ETHREAD con datos reales de /proc/[pid]/task/[tid]/status
    pub fn from_pid_tid(pid: u32, tid: u32) -> Option<Self> {
        let status_path = format!("/proc/{}/task/{}/status", pid, tid);
        if let Ok(status) = fs::read_to_string(&status_path) {
            let mut state = String::new();
            for line in status.lines() {
                if line.starts_with("State:") {
                    state = line[6..].trim().to_string();
                }
            }
            return Some(EThread { tid, owner_pid: pid, state });
        }
        None
    }
}
