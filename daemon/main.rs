// KernelBridge Daemon
// Rust, preparado para D-Bus y sockets UNIX

use std::io::{Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::thread;
use std::sync::Arc;
use std::sync::Mutex;
use std::fs;
use std::path::Path;

// Importar módulos
mod tpm_manager;
mod integrity_monitor;
mod anti_cheat_gateway;
mod game_launcher;
mod nt_api;
mod event_broker;
mod memory_auditor;
mod kernel_validator;
mod system_bridge_api;
mod nt_device_proxy;
mod mod_compat;
mod minecraft_auto_fix;
mod mod_resolver;
mod ace_handler;


use tpm_manager::TpmManager;
use integrity_monitor::IntegrityReport;
use anti_cheat_gateway::start_gateway;
use game_launcher::{GameConfig, launch_game};
use event_broker::start_event_broker;
use kernel_validator::validate_kernel;
use system_bridge_api::start_system_bridge;
use ace_handler::{setup_ace_for_game, AceEnvironment};

mod ac_integration;
use ac_integration::{
    eos_report_cheat,
    vac_apply_ban,
    query_report_status,
    get_file_hash,
    get_memory_hash,
    start_ebpf_monitor,
    start_ptrace_monitor,
};

// Para D-Bus
// use zbus::{Connection, fdo}; // Descomentar si se usa D-Bus

struct DaemonState {
    tpm: TpmManager,
    integrity: IntegrityReport,
}

fn handle_unix_client(mut stream: UnixStream, state: Arc<Mutex<DaemonState>>) {
    let mut buffer = [0u8; 1024];
    match stream.read(&mut buffer) {
        Ok(size) => {
            let request = String::from_utf8_lossy(&buffer[..size]);
            // Ejemplo de integración: monitoreo kernel y APIs anti-cheat
            if request.starts_with("MONITOR_KERNEL:") {
                if let Ok(pid) = request[15..].trim().parse::<u32>() {
                    let _ = start_ebpf_monitor(pid);
                    let _ = start_ptrace_monitor(pid);
                    let _ = stream.write_all(b"Monitoreo kernel iniciado\n");
                }
                return; // evitar escribir respuesta por defecto adicional
            } else if request.starts_with("PREPARE_GAME:") {
                // Formato: PREPARE_GAME:<source>:<id>
                let args = request[13..].trim();
                let parts: Vec<&str> = args.split(':').collect();
                let _source = parts.get(0).copied().unwrap_or("");
                let _id = parts.get(1).copied().unwrap_or("");
                // Reglas simples: exigir TPM si el juego lo pudiera requerir (placeholder)
                let state_guard = state.lock().unwrap();
                if !state_guard.tpm.available {
                    let _ = stream.write_all(b"ERROR: TPM no disponible\n");
                } else {
                    let _ = stream.write_all(b"OK: Listo para lanzar\n");
                }
                return; // evitar escribir respuesta por defecto adicional
            } else if request.starts_with("NT_IOCTL:") {
                // Formato simple: NT_IOCTL:<COMMAND> (p.ej. GET_PROCESS_LIST | GET_ATTESTATION)
                let cmd = request[9..].trim();
                let state_guard = state.lock().unwrap();
                let json = nt_device_proxy::handle_ioctl_request(
                    cmd,
                    state_guard.tpm.available,
                    state_guard.integrity.ima_status.clone(),
                    state_guard.integrity.evm_status.clone(),
                );
                let _ = stream.write_all(json.as_bytes());
                return; // evitar escribir respuesta por defecto adicional
            } else if request.starts_with("REPORT_CHEAT:") {
                let parts: Vec<&str> = request[13..].trim().split(':').collect();
                if parts.len() == 2 {
                    eos_report_cheat(parts[0], parts[1]);
                    let _ = stream.write_all(b"Reporte EOS registrado\n");
                }
                return; // evitar escribir respuesta por defecto adicional
            } else if request.starts_with("VAC_BAN:") {
                let parts: Vec<&str> = request[8..].trim().split(':').collect();
                if parts.len() == 2 {
                    vac_apply_ban(parts[0], parts[1]);
                    let _ = stream.write_all(b"Ban VAC registrado\n");
                }
                return; // evitar escribir respuesta por defecto adicional
            } else if request.starts_with("QUERY_REPORT:") {
                let id = request[13..].trim();
                if let Some(status) = query_report_status(id) {
                    let _ = stream.write_all(format!("Estado: {}\n", status).as_bytes());
                } else {
                    let _ = stream.write_all(b"Sin reporte\n");
                }
                return; // evitar escribir respuesta por defecto adicional
            } else if request.starts_with("FILE_HASH:") {
                let path = request[10..].trim();
                match get_file_hash(path) {
                    Ok(hash) => { let _ = stream.write_all(format!("Hash: {}\n", hash).as_bytes()); },
                    Err(_) => { let _ = stream.write_all(b"Error al calcular hash\n"); },
                }
                return; // evitar escribir respuesta por defecto adicional
            } else if request.starts_with("MEMORY_HASH:") {
                let parts: Vec<&str> = request[12..].trim().split(':').collect();
                if parts.len() == 3 {
                    if let (Ok(pid), Ok(addr), Ok(size)) = (parts[0].parse::<u32>(), parts[1].parse::<usize>(), parts[2].parse::<usize>()) {
                        match get_memory_hash(pid, addr, size) {
                            Ok(hash) => { let _ = stream.write_all(format!("Hash: {}\n", hash).as_bytes()); },
                            Err(_) => { let _ = stream.write_all(b"Error al calcular hash\n"); },
                        }
                    }
                }
                return; // evitar escribir respuesta por defecto adicional
            } else if request.starts_with("VM_START:") {
                let name = request[9..].trim();
                // Intentar primero system, luego session si el dominio no se encuentra
                let out_sys = std::process::Command::new("virsh").args(["-c", "qemu:///system", "start", name]).output();
                let mut retried = false;
                match out_sys {
                    Ok(o) if o.status.success() => {
                        let _ = stream.write_all(format!("OK: VM '{}' iniciada (system)\n", name).as_bytes());
                        return;
                    }
                    Ok(o) => {
                        let msg = String::from_utf8_lossy(&o.stderr);
                        if msg.to_lowercase().contains("domain not found") || msg.contains("Error al obtener el dominio") {
                            retried = true;
                        } else {
                            let _ = stream.write_all(format!("ERROR: virsh start fallo: {}\n", msg).as_bytes());
                            return;
                        }
                    }
                    Err(e) => { let _ = stream.write_all(format!("ERROR: virsh no disponible: {}\n", e).as_bytes()); return; }
                }
                if retried {
                    let out_sess = std::process::Command::new("virsh").args(["-c", "qemu:///session", "start", name]).output();
                    match out_sess {
                        Ok(o) if o.status.success() => { let _ = stream.write_all(format!("OK: VM '{}' iniciada (session)\n", name).as_bytes()); }
                        Ok(o) => { let msg = String::from_utf8_lossy(&o.stderr); let _ = stream.write_all(format!("ERROR: virsh start fallo (session): {}\n", msg).as_bytes()); }
                        Err(e) => { let _ = stream.write_all(format!("ERROR: virsh no disponible (session): {}\n", e).as_bytes()); }
                    }
                }
                return;
            } else if request.starts_with("VM_STOP:") {
                let name = request[8..].trim();
                // Intentar system, luego session
                let out = std::process::Command::new("virsh").args(["-c", "qemu:///system", "shutdown", name]).output();
                if let Ok(o) = &out { if o.status.success() { let _ = stream.write_all(format!("OK: VM '{}' detenida (shutdown system)\n", name).as_bytes()); return; } }
                let out2 = std::process::Command::new("virsh").args(["-c", "qemu:///system", "destroy", name]).output();
                if let Ok(o) = &out2 { if o.status.success() { let _ = stream.write_all(format!("OK: VM '{}' forzada (destroy system)\n", name).as_bytes()); return; } }
                let out3 = std::process::Command::new("virsh").args(["-c", "qemu:///session", "shutdown", name]).output();
                if let Ok(o) = &out3 { if o.status.success() { let _ = stream.write_all(format!("OK: VM '{}' detenida (shutdown session)\n", name).as_bytes()); return; } }
                let out4 = std::process::Command::new("virsh").args(["-c", "qemu:///session", "destroy", name]).output();
                match out4 {
                    Ok(o) if o.status.success() => { let _ = stream.write_all(format!("OK: VM '{}' forzada (destroy session)\n", name).as_bytes()); }
                    Ok(o) => { let msg = String::from_utf8_lossy(&o.stderr); let _ = stream.write_all(format!("ERROR: virsh stop fallo: {}\n", msg).as_bytes()); }
                    Err(e) => { let _ = stream.write_all(format!("ERROR: virsh no disponible: {}\n", e).as_bytes()); }
                }
                return;
            } else if request.starts_with("VM_STATUS:") {
                let name = request[10..].trim();
                let out_sys = std::process::Command::new("virsh").args(["-c", "qemu:///system", "domstate", name]).output();
                match out_sys {
                    Ok(o) if o.status.success() => { let state = String::from_utf8_lossy(&o.stdout).trim().to_string(); let _ = stream.write_all(format!("OK: {}\n", state).as_bytes()); }
                    Ok(o) => {
                        let msg = String::from_utf8_lossy(&o.stderr);
                        if msg.to_lowercase().contains("domain not found") || msg.contains("Error al obtener el dominio") {
                            let out_sess = std::process::Command::new("virsh").args(["-c", "qemu:///session", "domstate", name]).output();
                            match out_sess {
                                Ok(o2) if o2.status.success() => { let state = String::from_utf8_lossy(&o2.stdout).trim().to_string(); let _ = stream.write_all(format!("OK: {}\n", state).as_bytes()); }
                                Ok(o2) => { let msg2 = String::from_utf8_lossy(&o2.stderr); let _ = stream.write_all(format!("ERROR: {}\n", msg2).as_bytes()); }
                                Err(e2) => { let _ = stream.write_all(format!("ERROR: virsh no disponible (session): {}\n", e2).as_bytes()); }
                            }
                        } else { let _ = stream.write_all(format!("ERROR: {}\n", msg).as_bytes()); }
                    }
                    Err(e) => { let _ = stream.write_all(format!("ERROR: virsh no disponible: {}\n", e).as_bytes()); }
                }
                return;
            } else if request.starts_with("VM_HEALTH:") {
                // Devuelve direcciones e interfaces conocidas de la VM (requiere qemu-guest-agent para domifaddr detallado)
                let name = request[10..].trim();
                let out_sys = std::process::Command::new("virsh").args(["-c", "qemu:///system", "domifaddr", name]).output();
                match out_sys {
                    Ok(o) if o.status.success() => { let info = String::from_utf8_lossy(&o.stdout).to_string(); let _ = stream.write_all(format!("OK: {}\n", info.trim()).as_bytes()); }
                    Ok(o) => {
                        let msg = String::from_utf8_lossy(&o.stderr);
                        if msg.to_lowercase().contains("domain not found") || msg.contains("Error al obtener el dominio") {
                            let out_sess = std::process::Command::new("virsh").args(["-c", "qemu:///session", "domifaddr", name]).output();
                            match out_sess {
                                Ok(o2) if o2.status.success() => { let info = String::from_utf8_lossy(&o2.stdout).to_string(); let _ = stream.write_all(format!("OK: {}\n", info.trim()).as_bytes()); }
                                Ok(o2) => { let msg2 = String::from_utf8_lossy(&o2.stderr); let _ = stream.write_all(format!("ERROR: {}\n", msg2).as_bytes()); }
                                Err(e2) => { let _ = stream.write_all(format!("ERROR: virsh no disponible (session): {}\n", e2).as_bytes()); }
                            }
                        } else { let _ = stream.write_all(format!("ERROR: {}\n", msg).as_bytes()); }
                    }
                    Err(e) => { let _ = stream.write_all(format!("ERROR: virsh no disponible: {}\n", e).as_bytes()); }
                }
                return;
            } else if request.trim() == "VM_LIST" {
                // Lista nombres de dominios definidos en libvirt
                let out = std::process::Command::new("virsh").args(["-c", "qemu:///system", "list", "--all", "--name"]).output();
                match out {
                    Ok(o) if o.status.success() => {
                        let names = String::from_utf8_lossy(&o.stdout).to_string();
                        let _ = stream.write_all(format!("OK: {}\n", names.trim()).as_bytes());
                    }
                    Ok(o) => { let msg = String::from_utf8_lossy(&o.stderr); let _ = stream.write_all(format!("ERROR: {}\n", msg).as_bytes()); }
                    Err(e) => { let _ = stream.write_all(format!("ERROR: virsh no disponible: {}\n", e).as_bytes()); }
                }
                return;
            } else if request.trim() == "VM_LIST_ALL" {
                // Listar dominios en system y session
                let sys = std::process::Command::new("virsh").args(["-c", "qemu:///system", "list", "--all", "--name"]).output().ok();
                let ses = std::process::Command::new("virsh").args(["-c", "qemu:///session", "list", "--all", "--name"]).output().ok();
                let sys_s = sys.and_then(|o| if o.status.success() { Some(String::from_utf8_lossy(&o.stdout).to_string()) } else { None }).unwrap_or_default();
                let ses_s = ses.and_then(|o| if o.status.success() { Some(String::from_utf8_lossy(&o.stdout).to_string()) } else { None }).unwrap_or_default();
                let _ = stream.write_all(format!("OK: system:\n{}\nsession:\n{}\n", sys_s.trim(), ses_s.trim()).as_bytes());
                return;
            } else if request.starts_with("VM_CREATE:") {
                // Formato: VM_CREATE name|iso_path|disk_gb (defaults si faltan)
                let args = request[10..].trim();
                let parts: Vec<&str> = args.split('|').collect();
                let name = parts.get(0).map(|s| s.trim()).filter(|s| !s.is_empty()).unwrap_or("Windows-Gaming");
                let iso = parts.get(1).map(|s| s.trim()).unwrap_or("");
                let disk_gb = parts.get(2).and_then(|s| s.trim().parse::<u32>().ok()).unwrap_or(60);
                let mem_mb = 8192u32;
                let vcpus = 4u32;
                if iso.is_empty() {
                    let _ = stream.write_all(b"ERROR: Debes proporcionar ruta del ISO de Windows (2do parametro)\n");
                    return;
                }
                // Comando virt-install básico con UEFI y virtio
                let cmd = [
                    "--connect", "qemu:///system",
                    "--name", name,
                    "--memory", &mem_mb.to_string(),
                    "--vcpus", &vcpus.to_string(),
                    "--cpu", "host",
                    "--disk", &format!("size={},bus=virtio", disk_gb),
                    "--cdrom", iso,
                    "--os-variant", "win11",
                    "--graphics", "spice",
                    "--network", "network=default,model=virtio",
                    "--boot", "uefi",
                    "--noautoconsole",
                    "--wait", "-1",
                ];
                let out = std::process::Command::new("virt-install").args(cmd).output();
                match out {
                    Ok(o) if o.status.success() => { let _ = stream.write_all(format!("OK: VM '{}' creada\n", name).as_bytes()); }
                    Ok(o) => { let msg = String::from_utf8_lossy(&o.stderr); let _ = stream.write_all(format!("ERROR: virt-install fallo: {}\n", msg).as_bytes()); }
                    Err(e) => { let _ = stream.write_all(format!("ERROR: virt-install no disponible: {}\n", e).as_bytes()); }
                }
                return;
            } else if request.trim() == "CF_UNIFY_STORES" {
                let out = mod_compat::unify_curseforge_stores();
                let _ = stream.write_all(format!("{}\n", out).as_bytes());
                return;
            } else if request.trim() == "CF_VALIDATE_COMMON" {
                let out = mod_compat::validate_common_instances();
                let _ = stream.write_all(out.as_bytes());
                return;
            } else if request.starts_with("CF_VALIDATE_DIR:") {
                let path = request[16..].trim();
                let v = mod_compat::validate_modpack_dir(Path::new(path));
                let _ = stream.write_all(v.to_string().as_bytes());
                return;
            } else if request.starts_with("CF_PREPARE_DIR:") {
                let path = request[15..].trim();
                let out = mod_compat::prepare_modpack_dir(Path::new(path));
                let _ = stream.write_all(format!("{}\n", out).as_bytes());
                return;
            } else if request.starts_with("MC_AUTOFIX_DIR:") {
                // Analiza logs/latest.log del perfil y deshabilita packs problemáticos automáticamente
                let path = request[15..].trim();
                let rep = minecraft_auto_fix::scan_and_autofix(Path::new(path));
                let _ = stream.write_all(rep.as_bytes());
                return;
            } else if request.trim() == "MC_AUTOFIX_COMMON" {
                let rep = minecraft_auto_fix::autofix_common_instances();
                let _ = stream.write_all(rep.as_bytes());
                return;
            } else if request.starts_with("MC_SUGGEST_MISSING:") {
                let path = request[19..].trim();
                let rep = mod_resolver::suggest_missing_mods_dir(Path::new(path));
                let _ = stream.write_all(rep.as_bytes());
                return;
            } else if request.trim() == "MC_SUGGEST_MISSING_COMMON" {
                let rep = mod_resolver::suggest_missing_mods_common();
                let _ = stream.write_all(rep.as_bytes());
                return;
            } else if request.starts_with("MC_INSTALL_SUGGESTED:") {
                let path = request[21..].trim();
                let rep = mod_resolver::install_suggested_mods_dir(Path::new(path));
                let _ = stream.write_all(rep.as_bytes());
                return;
            } else if request.trim() == "MC_INSTALL_SUGGESTED_COMMON" {
                let rep = mod_resolver::install_suggested_mods_common();
                let _ = stream.write_all(rep.as_bytes());
                return;
            } else if request.starts_with("MC_SUGGEST_MISSING_CF:") {
                let path = request[22..].trim();
                let rep = mod_resolver::suggest_missing_mods_cf_dir(Path::new(path));
                let _ = stream.write_all(rep.as_bytes());
                return;
            } else if request.trim() == "MC_SUGGEST_MISSING_CF_COMMON" {
                let rep = mod_resolver::suggest_missing_mods_cf_common();
                let _ = stream.write_all(rep.as_bytes());
                return;
            } else if request.starts_with("MC_INSTALL_SUGGESTED_CF:") {
                let path = request[25..].trim();
                let rep = mod_resolver::install_suggested_mods_cf_dir(Path::new(path));
                let _ = stream.write_all(rep.as_bytes());
                return;
            } else if request.trim() == "MC_INSTALL_SUGGESTED_CF_COMMON" {
                let rep = mod_resolver::install_suggested_mods_cf_common();
                let _ = stream.write_all(rep.as_bytes());
                return;
            } else if request.starts_with("SETUP_ACE:") {
                // Formato: SETUP_ACE:game_path|wine_prefix
                let args = request[10..].trim();
                let parts: Vec<&str> = args.split('|').collect();
                if parts.len() >= 2 {
                    let game_path = Path::new(parts[0]);
                    let wine_prefix = Path::new(parts[1]);
                    match setup_ace_for_game(game_path, wine_prefix) {
                        Ok(msg) => { let _ = stream.write_all(format!("OK: {}\n", msg).as_bytes()); }
                        Err(e) => { let _ = stream.write_all(format!("ERROR: {}\n", e).as_bytes()); }
                    }
                } else {
                    let _ = stream.write_all(b"ERROR: Formato: SETUP_ACE:game_path|wine_prefix\n");
                }
                return;
            } else if request.starts_with("PREPARE_DELTA_FORCE:") {
                // Formato: PREPARE_DELTA_FORCE:game_path
                let game_path = request[20..].trim();
                let mut ace_env = AceEnvironment::new();
                match ace_env.prepare_delta_force(Path::new(game_path)) {
                    Ok(msg) => { let _ = stream.write_all(format!("OK: {}\n", msg).as_bytes()); }
                    Err(e) => { let _ = stream.write_all(format!("ERROR: {}\n", e).as_bytes()); }
                }
                return;
            }
            println!("[Daemon] Recibido por UNIX: {}", request);
            let response = match request.trim() {
                "CHECK_TPM" => {
                    let tpm = &state.lock().unwrap().tpm;
                    if tpm.available {
                        "TPM disponible".to_string()
                    } else {
                        "TPM no disponible".to_string()
                    }
                }
                "SHUTDOWN" => {
                    println!("[Daemon] Orden de apagado recibida. Cerrando...");
                    // Intentar limpiar PID file antes de salir
                    let _ = fs::remove_file("/tmp/kernelbridge-daemon.pid");
                    std::process::exit(0);
                }
                "GET_INTEGRITY_REPORT" => {
                    let integrity = &state.lock().unwrap().integrity;
                    format!("IMA: {:?}, EVM: {:?}, AppArmor: {:?}, SELinux: {:?}, TPM: {:?}",
                            integrity.ima_status, integrity.evm_status, integrity.apparmor_status,
                            integrity.selinux_status, integrity.tpm_status)
                }
                "LAUNCH_GAME:test_game" => {
                    let config = GameConfig {
                        path: "/usr/bin/test_game".to_string(), // Ejemplo
                        requires_tpm: true,
                        anti_cheat: true,
                    };
                    match launch_game(&config) {
                        Ok(_) => "Juego lanzado".to_string(),
                        Err(e) => format!("Error: {}", e),
                    }
                }
                _ => "Comando desconocido".to_string(),
            };
            let _ = stream.write_all(response.as_bytes());
        }
        Err(e) => eprintln!("Error leyendo cliente UNIX: {}", e),
    }
}

fn start_unix_socket(path: &str, state: Arc<Mutex<DaemonState>>) {
    let _ = std::fs::remove_file(path); // Limpia socket anterior
    let listener = UnixListener::bind(path).expect("No se pudo crear el socket UNIX");
    println!("[Daemon] Esperando conexiones por UNIX en {}...", path);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let state_clone = Arc::clone(&state);
                thread::spawn(move || handle_unix_client(stream, state_clone));
            }
            Err(e) => eprintln!("Error en conexión UNIX: {}", e),
        }
    }
}

// Esqueleto para D-Bus
// fn start_dbus() {
//     let connection = Connection::session().unwrap();
//     // Registrar interfaz y métodos aquí
//     println!("[Daemon] Esperando comandos por D-Bus...");
//     loop { /* Espera y procesa mensajes D-Bus */ }
// }

fn main() {
    let state = Arc::new(Mutex::new(DaemonState {
        tpm: TpmManager::detect(),
        integrity: IntegrityReport::generate(),
    }));

    // Arranca Anti-Cheat Gateway en hilo separado
    let state_clone = Arc::clone(&state);
    thread::spawn(move || {
        start_gateway("/tmp/kernelbridge_acgw.sock");
    });

    // Arranca Event Broker
    let state_clone2 = Arc::clone(&state);
    thread::spawn(move || {
        start_event_broker();
    });

    // Arranca System Bridge API
    let state_clone3 = Arc::clone(&state);
    thread::spawn(move || {
        start_system_bridge();
    });

    // Validar kernel al iniciar
    match validate_kernel() {
        Ok(valid) => {
            if valid {
                println!("[Daemon] Kernel validado correctamente.");
            } else {
                eprintln!("[Daemon] Kernel no coincide con la firma esperada.");
            }
        }
        Err(e) => eprintln!("Error validando kernel: {}", e),
    }

    // Arranca socket UNIX del daemon
    let unix_path = "/tmp/kernelbridge.sock";
    let state_clone2 = Arc::clone(&state);
    thread::spawn(move || start_unix_socket(unix_path, state_clone2));

    println!("[Daemon] KernelBridge Daemon iniciado y todos los servicios activos.");

    // Registrar PID para que la GUI pueda finalizar el daemon al cerrar
    let pid = std::process::id();
    if let Err(e) = fs::write("/tmp/kernelbridge-daemon.pid", pid.to_string()) {
        eprintln!("[Daemon] No se pudo escribir PID file: {}", e);
    }

    // Manejo simple de señales para limpiar PID en salida
    // Nota: Dependemos principalmente de SHUTDOWN o de kill -TERM externo
    loop { thread::park(); } // Mantiene el daemon vivo
}
