// Anti-Cheat Gateway
// Expone estructuras NT simuladas con datos reales a los anti-cheats

use core::nt_structs::{EProcess, HandleTable, ObjectHeader, EThread, CriticalSection, Mutex, Semaphore, Section, MemoryRegion, get_memory_regions};
use std::os::unix::net::{UnixListener, UnixStream};
use std::io::{Read, Write};
use std::thread;

pub fn start_gateway(socket_path: &str) {
    let _ = std::fs::remove_file(socket_path);
    let listener = UnixListener::bind(socket_path).expect("No se pudo crear el socket UNIX para Anti-Cheat Gateway");
    println!("[AntiCheatGateway] Esperando conexiones en {}...", socket_path);
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                thread::spawn(move || handle_client(&mut stream));
            }
            Err(e) => eprintln!("Error en conexión Anti-Cheat Gateway: {}", e),
        }
    }
}

fn handle_client(stream: &mut UnixStream) {
    let mut buffer = [0u8; 1024];
    if let Ok(size) = stream.read(&mut buffer) {
        let request = String::from_utf8_lossy(&buffer[..size]);
        // Comandos soportados para máxima compatibilidad
        if request.starts_with("GET_EPROCESS:") {
            if let Ok(pid) = request[13..].trim().parse::<u32>() {
                if let Some(eproc) = EProcess::from_pid(pid) {
                    let response = format!("EPROCESS: {:?}", eproc);
                    let _ = stream.write_all(response.as_bytes());
                }
            }
        } else if request.starts_with("GET_HANDLE_TABLE:") {
            if let Ok(pid) = request[16..].trim().parse::<u32>() {
                if let Some(ht) = HandleTable::from_pid(pid) {
                    let response = format!("HANDLE_TABLE: {:?}", ht);
                    let _ = stream.write_all(response.as_bytes());
                }
            }
        } else if request.starts_with("GET_OBJECT_HEADER:") {
            let response = format!("OBJECT_HEADER: {:?}", ObjectHeader::new("File", 1, 0));
            let _ = stream.write_all(response.as_bytes());
        } else if request.starts_with("GET_ETHREAD:") {
            let parts: Vec<&str> = request[12..].trim().split(':').collect();
            if parts.len() == 2 {
                if let (Ok(pid), Ok(tid)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>()) {
                    if let Some(ethread) = EThread::from_pid_tid(pid, tid) {
                        let response = format!("ETHREAD: {:?}", ethread);
                        let _ = stream.write_all(response.as_bytes());
                    }
                }
            }
        } else if request.starts_with("GET_CRITICAL_SECTION") {
            let cs = CriticalSection::new();
            let response = format!("CRITICAL_SECTION: {:?}", cs);
            let _ = stream.write_all(response.as_bytes());
        } else if request.starts_with("GET_MUTEX") {
            let mutex = Mutex::new();
            let response = format!("MUTEX: {:?}", mutex);
            let _ = stream.write_all(response.as_bytes());
        } else if request.starts_with("GET_SEMAPHORE") {
            let sem = Semaphore::new(10);
            let response = format!("SEMAPHORE: {:?}", sem);
            let _ = stream.write_all(response.as_bytes());
        } else if request.starts_with("GET_SECTION") {
            let section = Section::new(0x1000, 0x10000, 2);
            let response = format!("SECTION: {:?}", section);
            let _ = stream.write_all(response.as_bytes());
        } else if request.starts_with("GET_MEMORY_REGIONS:") {
            if let Ok(pid) = request[18..].trim().parse::<u32>() {
                let regions = get_memory_regions(pid);
                let response = format!("MEMORY_REGIONS: {:?}", regions);
                let _ = stream.write_all(response.as_bytes());
            }
        } else if request.starts_with("WINE_PROTON_INFO") {
            // Responde con info de integración Wine/Proton
            let response = format!("WINE: {}, PROTON: {}", std::env::var("WINE_VERSION").unwrap_or_default(), std::env::var("PROTON_VERSION").unwrap_or_default());
            let _ = stream.write_all(response.as_bytes());
        } else {
            let response = "Comando desconocido";
            let _ = stream.write_all(response.as_bytes());
        }
    }
}