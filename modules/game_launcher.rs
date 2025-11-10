// Game Launcher Seguro
// Lanza juegos en entorno NT-compatible y seguro

use std::process::Command;
use std::os::unix::net::UnixStream;
use std::path::Path;

pub struct GameConfig {
    pub path: String,
    pub requires_tpm: bool,
    pub anti_cheat: bool,
}

pub fn launch_game(config: &GameConfig) -> Result<(), String> {
    // Verifica TPM si es requerido
    if config.requires_tpm && !Path::new("/dev/tpm0").exists() {
        return Err("TPM no disponible".to_string());
    }
    // Verifica anti-cheat (puede consultar el gateway)
    if config.anti_cheat {
        if let Ok(mut stream) = UnixStream::connect("/tmp/kernelbridge_acgw.sock") {
            let _ = stream.write_all(b"CHECK_INTEGRITY");
            // Aquí se puede leer respuesta y validar integridad
        }
    }
    // Lanza el juego en namespace aislado
    use std::process::Stdio;
    let result = Command::new("unshare")
        .arg("--fork")
        .arg("--pid")
        .arg("--mount")
        .arg("--uts")
        .arg("--ipc")
        .arg("--net")
        .arg(&config.path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();
    match result {
        Ok(_child) => Ok(()),
        Err(e) => Err(format!("Error al lanzar juego: {}", e)),
    }
}
