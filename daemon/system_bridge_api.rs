use std::os::unix::net::{UnixListener, UnixStream};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::fs;

pub struct SystemBridgeApi {
    state: Arc<Mutex<String>>,
}

impl SystemBridgeApi {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new("idle".to_string())),
        }
    }

    pub fn start_bridge(&self) -> Result<(), String> {
        let path = "/tmp/kernelbridge_api.sock";
        let _ = fs::remove_file(path);
        let listener = UnixListener::bind(path).map_err(|e| format!("Bind failed: {}", e))?;
        println!("[BridgeAPI] Listening on {}", path);

        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let mut buffer = [0; 1024];
                    let size = match stream.read(&mut buffer) { Ok(n) => n, Err(e) => { eprintln!("[BridgeAPI] read err: {}", e); continue; } };
                    let msg = String::from_utf8_lossy(&buffer[..size]).trim().to_string();
                    let resp = self.handle_message(&msg);
                    let _ = stream.write_all(resp.as_bytes());
                }
                Err(e) => eprintln!("[BridgeAPI] Connection failed: {}", e),
            }
        }
        Ok(())
    }

    fn handle_message(&self, msg: &str) -> String {
        match msg.to_uppercase().as_str() {
            "PING" => {
                format!("OK: bridge alive | version={}\n", env!("CARGO_PKG_VERSION"))
            }
            "HEALTH" => {
                let state = self.state.lock().unwrap().clone();
                format!("{{\"status\":\"ok\",\"state\":\"{}\"}}\n", state)
            }
            other => {
                format!("ERR: unknown command '{}': use PING|HEALTH\n", other)
            }
        }
    }
}

pub fn start_system_bridge() {
    let bridge = SystemBridgeApi::new();
    let _ = bridge.start_bridge();
}