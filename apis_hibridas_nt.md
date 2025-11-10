# APIs Híbridas NT

Este módulo implementa funciones internas del Kernel de Windows (NT) adaptadas a Linux, usando Rust y C.

## Funciones implementadas
- ZwQueryInformationProcess: Consulta información real de procesos usando /proc/[pid]/stat.
- KeAttachProcess: Valida y prepara cambio de contexto (namespace) usando /proc/[pid]/ns/mnt.
- ObReferenceObjectByHandle: Valida y obtiene el recurso de un handle (fd) usando /proc/[pid]/fd.
- PsLookupProcessByProcessId: Verifica existencia de proceso por PID.
- ZwOpenProcess: Abre un handle real al proceso usando /proc/[pid]/mem.
- NtReadVirtualMemory / NtWriteVirtualMemory: Acceso legítimo a memoria de procesos usando process_vm_readv/writev.

## Tecnologías
- Rust, C
- /proc, ptrace, process_vm_readv/writev
- eBPF, kprobes (futuro)

## Uso
Estas funciones interceptan llamadas NT y las traducen a equivalentes nativos de Linux, devolviendo estructuras NT simuladas pero con datos reales.
