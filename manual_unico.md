# KernelBridge – Manual único (consolidado)

Fecha: 31/10/2025

Este documento consolida toda la documentación técnica y funcional del proyecto en un único archivo. Incluye arquitectura, módulos, flujos, IPC, compatibilidad anti-cheat, estructuras NT, lanzador de juegos, monitor de integridad, TPM, y el NT Device Proxy con comandos y ejemplos.

## Actualizaciones 31/10/2025

- NT Device Proxy: comandos añadidos de diagnóstico no intrusivo: CHECK_PROCESS_SECURITY, CHECK_DEBUG, CHECK_SANDBOX_VM, CHECK_MULTICLIENT.

## Índice

1. Visión general del sistema (sistema_completo)
2. Organización de módulos (organizacion_modulos)
3. Flujo de comunicación (flujo_comunicacion)
4. Compatibilidad anti-cheat (compatibilidad_anticheat)
5. APIs híbridas NT (apis_hibridas_nt)
6. Estructuras NT simuladas (nt_structs)
7. Anti-Cheat Gateway (anti_cheat_gateway)
8. Game Launcher seguro (game_launcher)
9. Integrity Monitor (integrity_monitor)
10. TPM Manager (tpm_manager)
11. NT Device Proxy (nt_device_proxy)
12. Módulos adicionales (modulos_adicionales)
13. Roadmap completado (roadmap_completado)

---

## 1) Visión general del sistema

[Fuente: docs/sistema_completo.md]

{{INICIO}}

KernelBridge – Documentación del sistema completo

Este documento explica qué es KernelBridge, cómo está organizado, qué hace cada módulo, cómo se comunican, los flujos típicos (scan → preflight → launch), el modelo de seguridad y cómo probar funciones clave como el NT Device Proxy.

¿Que es KernelBridge?

KernelBridge permite ejecutar juegos en Linux respetando el entorno real de cada plataforma (Steam/Proton, Bottles, Lutris) y exponiendo una capa de telemetría “estilo NT” (NT Device Proxy) desde un daemon seguro. El objetivo es interoperar con anti-cheats usando el runtime/lanzador correcto, no emularlos ni evadirlos.

Beneficios:
- Lanza juegos con su runtime original (Proton/Steam, bottles de Bottles, runners de Lutris).
- Verifica requisitos mínimos (ej. TPM) antes de ejecutar (PREPARE_GAME).
- Ofrece telemetría NT-like (procesos, hilos, módulos, handles, memoria, atestación) para diagnóstico e integridad.
- Cierre limpio del daemon cuando cierra la GUI.

Arquitectura general, módulos, IPC, flujos, modelo de seguridad, uso GUI/terminal, configuración, requisitos, roadmap y troubleshooting se incluyen como parte de este manual y se expanden en las secciones siguientes.

{{FIN}}

---

## 2) Organización de módulos

[Fuente: docs/organizacion_modulos.md]

# Organización de módulos y flujo de KernelBridge

## Estructura de carpetas

- gui/                # Interfaz gráfica GNOME (GTK 4 + libadwaita)
- daemon/             # Servicio en segundo plano, IPC y control de módulos
- core/               # APIs híbridas NT, acceso a Linux, estructuras NT
- modules/
    - tpm_manager.rs      # Detección y gestión real de TPM
    - integrity_monitor.rs# Monitor de integridad y reporte
    - anti_cheat_gateway.rs # Exposición de estructuras NT a anti-cheats
    - game_launcher.rs    # Lanzador seguro de juegos
    - event_broker.rs     # Registro de eventos del sistema
    - memory_auditor.rs   # Monitoreo de memoria de juegos
    - kernel_validator.rs # Validación de hashes del kernel con TPM
    - system_bridge_api.rs# Comunicación daemon-juegos
- docs/               # Documentación técnica y de arquitectura
- config.toml         # Configuraciones
- logs/               # Logs del sistema
- flatpak/            # Empaquetado seguro
- README.md           # Guía de instalación y uso

## Flujo general

1. El usuario gestiona todo desde la GUI.
2. La GUI envía comandos al daemon (sockets UNIX/D-Bus).
3. El daemon controla los módulos (TPM, integridad, APIs NT, launcher, event broker, etc.).
4. El core implementa las APIs NT y expone estructuras NT simuladas con datos reales.
5. El Anti-Cheat Gateway traduce y expone datos NT al anti-cheat.
6. El Game Launcher ejecuta juegos en entorno seguro y reporta estado.
7. Event Broker registra eventos en tiempo real.
8. Memory Auditor monitorea accesos a memoria.
9. Kernel Validator asegura integridad del kernel.
10. System Bridge API facilita comunicación con juegos.

## Seguridad
- TPM real, IMA/EVM, AppArmor, SELinux, namespaces.
- No hay simulación ni evasión: todo acceso y reporte es legítimo y verificable.

---

## 3) Flujo de comunicación

[Fuente: docs/flujo_comunicacion.md]

(Figuras de interacción GUI ↔ Daemon ↔ Core, IPC por UNIX socket, ejemplos de mensajes JSON, seguridad, y extensiones)

---

## 4) Compatibilidad anti-cheat

[Fuente: docs/compatibilidad_anticheat.md]

Compatibilidad con sistemas anti‑cheat modernos sin mención de marcas específicas. Se exponen estructuras NT válidas (EPROCESS, ETHREAD, HANDLE_TABLE, OBJECT_HEADER) con datos reales. Acceso legítimo a memoria/contexto (ptrace, process_vm_*), integridad vía TPM/IMA/EVM.

---

## 5) APIs híbridas NT

[Fuente: docs/apis_hibridas_nt.md]

Funciones implementadas (ejemplos): ZwQueryInformationProcess, KeAttachProcess, ObReferenceObjectByHandle, PsLookupProcessByProcessId, ZwOpenProcess, NtReadVirtualMemory/NtWriteVirtualMemory; traducidas a /proc, ptrace, process_vm_*.

---

## 6) Estructuras NT simuladas

[Fuente: docs/nt_structs.md]

EPROCESS, HANDLE_TABLE, OBJECT_HEADER, ETHREAD a partir de /proc.

---

## 7) Anti-Cheat Gateway

[Fuente: docs/anti_cheat_gateway.md]

Traducción de estructuras NT sobre datos reales; integra ptrace/eBPF. Seguridad: no evasión, validación y registro de accesos.

---

## 8) Game Launcher seguro

[Fuente: docs/game_launcher.md]

Preflight TPM/anti-cheat; ejecución en namespace aislado; reporting a GUI/gateway.

---

## 9) Integrity Monitor

[Fuente: docs/integrity_monitor.md]

Estado IMA/EVM/AppArmor/SELinux/TPM; reportes y tecnologías usadas.

---

## 10) TPM Manager

[Fuente: docs/tpm_manager.md]

Detección y gestión de TPM 2.0 con tpm2-tools/libtpm; seguridad y funciones.

---

## 11) NT Device Proxy

[Fuente: docs/nt_device_proxy.md]

Transporte, comandos `GET_PROCESS_LIST`, `GET_THREAD_LIST:<pid>`, `GET_MODULES:<pid>`, `GET_HANDLE_TABLE:<pid>`, `GET_PROCESS_MEMORY_MAP:<pid>`, `GET_ATTESTATION`, `GET_ATTESTATION_EXT`; ejemplos JSON; permisos y relación con GUI.

Comandos añadidos recientemente:
- `CHECK_PROCESS_SECURITY:<pid>`: heurísticas de seguridad (W+X, ejecuciones anónimas/memfd, tracer_pid, regiones sospechosas).
- `CHECK_DEBUG:<pid>`: lee TracerPid e infiere depuración.
- `CHECK_SANDBOX_VM`: señales de virtualización (flag hypervisor, DMI vendor/product).
- `CHECK_MULTICLIENT:<name>`: conteo de procesos coincidentes por nombre.

---

## 12) Módulos adicionales

[Fuente: docs/modulos_adicionales.md]

Event Broker, Memory Auditor, Kernel Validator, System Bridge API.

---

## 13) Roadmap completado

[Fuente: docs/roadmap_completado.md]

Fases alcanzadas y estado de producción.

---

## Notas finales

Este manual unificado resume y reúne la documentación del proyecto para consulta rápida. Para detalles actualizados por módulo, consultar los archivos individuales en `docs/`.
