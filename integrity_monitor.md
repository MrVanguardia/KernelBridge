# Integrity Monitor

Reporta el estado real del sistema para validación por anti-cheat.

## Funciones
- Supervisa módulos y archivos críticos.
- Genera informes de integridad (hashes, firmas, timestamps).
- Usa IMA/EVM, AppArmor, SELinux y TPM para medición y aislamiento.

## Ejemplo de reporte
- Estado IMA: /sys/kernel/security/ima/status
- Estado EVM: /sys/kernel/security/evm/status
- Estado AppArmor: aa-status
- Estado SELinux: getenforce
- Estado TPM: tpm2_pcrread

## Tecnologías
- Rust
- IMA/EVM, AppArmor, SELinux, TPM
