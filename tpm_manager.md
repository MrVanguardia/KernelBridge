# TPM Manager

Detecta y gestiona el chip TPM 2.0 real usando tpm2-tools y libtpm.

## Funciones
- Detecta la presencia de TPM en /dev/tpm0.
- Obtiene versión y propiedades del TPM con tpm2_getcap.
- Lee PCRs y estado con tpm2_pcrread.

## Seguridad
- Si no existe TPM, bloquea automáticamente los juegos que lo requieren.
- Puede integrarse con IMA/EVM para medir la integridad del sistema.

## Tecnologías
- tpm2-tools, libtpm
- Rust
