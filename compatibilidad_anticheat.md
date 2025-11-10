# Compatibilidad con sistemas anti‑cheat

KernelBridge está diseñado para interoperar con sistemas anti‑cheat modernos sin depender de marcas específicas. El enfoque prioriza el cumplimiento y la observabilidad legítima desde espacio de usuario.

## Estructuras NT Válidas
- **EPROCESS**: PID, nombre, PPID, threads, start_time, UID, GID, memory_usage, state (desde /proc/[pid]/stat y /proc/[pid]/status)
- **ETHREAD**: TID, PID propietario, estado (desde /proc/[pid]/task/[tid]/status)
- **HANDLE_TABLE**: Lista de handles abiertos (desde /proc/[pid]/fd)
- **OBJECT_HEADER**: Tipo de objeto, handle_count, flags

Generadas dinámicamente con datos reales del sistema Linux.

## Acceso Legítimo a Memoria y Contexto
- Usando ptrace, process_vm_readv/writev, eBPF según privilegios del daemon.
- Evitando interferencias del usuario o capas de sandbox no autorizadas.

## Asegurando Integridad mediante TPM
- Validación del kernel y daemon firmados usando TPM.
- Verificación de políticas IMA/EVM activas.
- Secure Boot detection.

## Syscalls NT Críticas Reproducidas
- NtReadVirtualMemory / NtWriteVirtualMemory: Acceso a memoria usando process_vm_*.
- NtQuerySystemInformation: Información del sistema desde /proc.
- ZwQueryInformationProcess, ZwOpenProcess, etc.: Traducidas a llamadas Linux.

## Reportes de Integridad Legibles
- Estructuras de firma comparables con hashes esperados.
- Uso de SHA256 y PCR extendidos para medición de arranque y módulos.

## Aislamiento y Sandbox de Ejecución
- Cada juego corre en su propio namespace (PID, mount, UTS, IPC, net) controlado por el daemon.
- Impide interferencias externas y asegura trazabilidad.

## Tecnologías Usadas
- APIs híbridas: Rust, C, eBPF, kprobes
- TPM e integridad: tpm2-tools, libtpm, IMA/EVM, Secure Boot
- Seguridad: AppArmor, SELinux, namespaces
- Kernel Interfaces: /proc, /sys, ptrace, auditd