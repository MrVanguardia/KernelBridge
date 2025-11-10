// Event Broker
// Servicio ligero que registra eventos del sistema y los envía en tiempo real al Anti-Cheat Gateway

use std::os::unix::net::UnixStream;
use std::io::Write;
use std::thread;
use std::time::Duration;

pub fn start_event_broker(acgw_socket: &str) {
    thread::spawn(move || {
        loop {
            // Simular eventos del sistema (ej: cambios en /proc, logs de auditd)
            let event = "EVENT: Proceso nuevo detectado";
            if let Ok(mut stream) = UnixStream::connect(acgw_socket) {
                let _ = stream.write_all(event.as_bytes());
            }
            thread::sleep(Duration::from_secs(10)); // Enviar cada 10s
        }
    });
}