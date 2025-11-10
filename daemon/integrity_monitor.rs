// Integrity Monitor: reporta estado real del sistema
// Usa IMA/EVM, AppArmor, SELinux, TPM

use std::process::Command;

pub struct IntegrityReport {
    pub ima_status: Option<String>,
    pub evm_status: Option<String>,
    pub apparmor_status: Option<String>,
    pub selinux_status: Option<String>,
    pub tpm_status: Option<String>,
}

impl IntegrityReport {
    pub fn generate() -> Self {
        let ima_status = Command::new("cat").arg("/sys/kernel/security/ima/status").output().ok().map(|o| String::from_utf8_lossy(&o.stdout).to_string());
        let evm_status = Command::new("cat").arg("/sys/kernel/security/evm/status").output().ok().map(|o| String::from_utf8_lossy(&o.stdout).to_string());
        let apparmor_status = Command::new("aa-status").output().ok().map(|o| String::from_utf8_lossy(&o.stdout).to_string());
        let selinux_status = Command::new("getenforce").output().ok().map(|o| String::from_utf8_lossy(&o.stdout).to_string());
        let tpm_status = Command::new("tpm2_pcrread").output().ok().map(|o| String::from_utf8_lossy(&o.stdout).to_string());
        IntegrityReport { ima_status, evm_status, apparmor_status, selinux_status, tpm_status }
    }
}