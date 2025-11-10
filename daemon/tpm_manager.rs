// TPM Manager: detección y gestión real de TPM 2.0
// Usa tpm2-tools y libtpm

use std::process::Command;
use std::path::Path;

pub struct TpmManager {
    pub available: bool,
    pub version: Option<String>,
    pub pcrs: Option<String>,
}

impl TpmManager {
    /// Detecta si existe TPM real en /dev/tpm0
    pub fn detect() -> Self {
        let available = Path::new("/dev/tpm0").exists();
        let version = if available {
            Command::new("tpm2_getcap").arg("-c").arg("properties-fixed").output().ok().map(|o| String::from_utf8_lossy(&o.stdout).to_string())
        } else { None };
        let pcrs = if available {
            Command::new("tpm2_pcrread").output().ok().map(|o| String::from_utf8_lossy(&o.stdout).to_string())
        } else { None };
        TpmManager { available, version, pcrs }
    }
}