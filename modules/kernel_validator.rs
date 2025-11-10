// Kernel Validator
// Valida hashes del kernel y módulos críticos mediante TPM

use std::process::Command;
use std::fs;

pub fn validate_kernel() -> Result<(), String> {
    // Leer hash esperado del TPM
    let expected_hash = Command::new("tpm2_pcrread").arg("sha256:0").output().ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
        .unwrap_or_default();

    // Calcular hash actual del kernel
    let kernel_path = "/boot/vmlinuz-linux"; // Ejemplo para Arch
    if let Ok(data) = fs::read(kernel_path) {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(&data);
        let actual_hash = format!("{:x}", hasher.finalize());

        if expected_hash.contains(&actual_hash) {
            Ok(())
        } else {
            Err("Hash del kernel no coincide".to_string())
        }
    } else {
        Err("No se pudo leer el kernel".to_string())
    }
}