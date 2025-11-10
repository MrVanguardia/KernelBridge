# Anti-Cheat Gateway

Este módulo permite que los anti-cheats lean estructuras NT simuladas pero basadas en datos reales del sistema Linux.

## Funciones
- Expone EPROCESS, ETHREAD, HANDLE_TABLE, OBJECT_HEADER, etc. en formato NT-compatible.
- Traduce consultas del anti-cheat a llamadas nativas de Linux (sin ocultar el entorno).
- Provee acceso legítimo a memoria y contexto usando ptrace, eBPF, process_vm_readv/writev.
- Reporta integridad y estado del sistema usando TPM Manager e Integrity Monitor.

## Seguridad y compatibilidad
- No realiza evasión ni manipulación del anti-cheat.
- Cada juego corre en su propio namespace controlado por el daemon.
- El gateway valida y registra todos los accesos y eventos relevantes.

## Tecnologías
- Rust, C
- Sockets UNIX, D-Bus
- ptrace, eBPF, TPM, IMA/EVM

## Ejemplo de flujo
1. Anti-cheat solicita EPROCESS de un PID.
2. Gateway consulta /proc/[pid]/stat y genera estructura NT.
3. Anti-cheat recibe datos reales en formato esperado.
