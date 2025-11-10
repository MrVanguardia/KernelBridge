// TPM Manager: detección y gestión real de TPM 2.0
// Usa tpm2-tools y libtpm

use std::process::Command;
use std::path::Path;

pub struct TpmManager {
    pub available: bool,
    pub version: Option<String>,
    pub pcrs: Option<String>,
    pub secure_boot: bool,
}

impl TpmManager {
    /// Detecta si existe TPM real en /dev/tpm0 y Secure Boot
    pub fn detect() -> Self {
        let available = Path::new("/dev/tpm0").exists();
        let version = if available {
            Command::new("tpm2_getcap").arg("-c").arg("properties-fixed").output().ok().map(|o| String::from_utf8_lossy(&o.stdout).to_string())
        } else { None };
        let pcrs = if available {
            Command::new("tpm2_pcrread").output().ok().map(|o| String::from_utf8_lossy(&o.stdout).to_string())
        } else { None };
        let secure_boot = fs::read_to_string("/sys/firmware/efi/efivars/SecureBoot-8be4df61-93ca-11d2-aa0d-00e098032b8c").is_ok();
        TpmManager { available, version, pcrs, secure_boot }
    }
}
