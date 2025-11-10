# KernelBridge – Documentación del sistema completo

Este documento explica qué es KernelBridge, cómo está organizado, qué hace cada módulo, cómo se comunican, los flujos típicos (scan → preflight → launch), el modelo de seguridad y cómo probar funciones clave como el NT Device Proxy.

## ¿Qué es KernelBridge?

KernelBridge permite ejecutar juegos en Linux respetando el entorno real de cada plataforma (Steam/Proton, Bottles, Lutris) y exponiendo una capa de telemetría “estilo NT” (NT Device Proxy) desde un daemon seguro. El objetivo es interoperar con anti-cheats usando el runtime/lanzador correcto, no emularlos ni evadirlos.

Beneficios:
- Lanza juegos con su runtime original (Proton/Steam, bottles de Bottles, runners de Lutris).
- Verifica requisitos mínimos (ej. TPM) antes de ejecutar (PREPARE_GAME).
- Ofrece telemetría NT-like (procesos, hilos, módulos, handles, memoria, atestación) para diagnóstico e integridad.
- Cierre limpio del daemon cuando cierra la GUI.

## Arquitectura general

- GUI (crate `gui/`, GTK4 + Relm4)
  - Lista juegos (Steam/Bottles/Lutris/Local), detecta launchers, lanza con el runtime correcto.
  - Botones de diagnóstico: “🧪 Probar NT Proxy” y consultas por PID (Hilos/Módulos/Handles/Memoria).
  - Envía IPC por socket UNIX al daemon: `PREPARE_GAME`, `NT_IOCTL:<CMD>`, etc.
- Daemon (crate `daemon/`)
  - Servicio long-running que escucha por UNIX socket en `/tmp/kernelbridge.sock` y gestiona los módulos.
  - Expone el NT Device Proxy (IOCTLs) y el gate de `PREPARE_GAME`.
  - Escribe `/tmp/kernelbridge-daemon.pid` para finalizarse limpiamente.
- Core (crate `core/`)
  - Tipos utilitarios y utilidades compartidas entre GUI y daemon.

Comunicación principal: socket UNIX (`/tmp/kernelbridge.sock`).

## Módulos del daemon y responsabilidades

- `tpm_manager` – Detección/estado del TPM y atestación básica. Se usa en `PREPARE_GAME` y en `GET_ATTESTATION`.
- `integrity_monitor` – Reporte de IMA/EVM/AppArmor/SELinux; datos se devuelven en `GET_ATTESTATION`.
- `anti_cheat_gateway` – Punto de integración/telemetría con sistemas anti-cheat (ganchos de monitoreo permitidos: eBPF/ptrace, etc.). No se usa para evasión.
- `game_launcher` – Lógica de lanzamiento de juegos (actualmente la GUI delega a lanzadores; este módulo sirve de base para automatizaciones futuras y pruebas).
- `event_broker` – Canalización de eventos entre módulos (para métricas y auditoría).
- `memory_auditor` – Utilidades de inspección segura de memoria (lectura/huellas; sin escritura ni inyección).
- `kernel_validator` – Validación del kernel frente a firma/huella esperada.
- `system_bridge_api` – Puntos de extensión para exponer capas del sistema al resto de módulos.
- `nt_device_proxy` – Traductor de IOCTLs lógicos NT → datos reales Linux (procesos, hilos, módulos/FDs/memoria, atestación).

## IPC: comandos soportados

Transporte: UNIX socket en `/tmp/kernelbridge.sock`.

- `PREPARE_GAME:<source>:<id>` → `OK: ...` o `ERROR: ...`
  - Gate de pre-requisitos (TPM disponible, políticas básicas). `source` ∈ {Steam,Bottles,Lutris,Local}.
- `NT_IOCTL:<CMD>` → JSON de una línea
  - Ver más en `docs/nt_device_proxy.md`. Comandos: `GET_PROCESS_LIST`, `GET_THREAD_LIST:<pid>`, `GET_MODULES:<pid>`, `GET_HANDLE_TABLE:<pid>`, `GET_PROCESS_MEMORY_MAP:<pid>`, `GET_ATTESTATION`, `GET_ATTESTATION_EXT`.
- Otros comandos del daemon (para utilidades internas o de pruebas):
  - `MONITOR_KERNEL:<pid>` – inicia monitores permitidos (eBPF/ptrace).
  - `REPORT_CHEAT:<game_id>:<details>`, `VAC_BAN:<id>:<reason>`, `QUERY_REPORT:<id>` – hooks de ejemplo.
  - `FILE_HASH:<path>`, `MEMORY_HASH:<pid>:<addr_hex>:<size>` – utilidades de hashing.
  - `SHUTDOWN` – finaliza daemon y limpia PID file.

Notas de framing:
- Las peticiones terminan en `\n`.
- El daemon responde una única vez por petición (ajustado para evitar respuestas duplicadas).

## Flujos típicos

1) Inicio y cierre
- `start.sh` arranca GUI y daemon; captura señales para enviar `SHUTDOWN` al cerrar.
- La GUI, al cerrar la ventana, intenta `SHUTDOWN` → si falla, lee `PID` y envía `SIGTERM` → último recurso `pkill`.

2) Escaneo de juegos (GUI)
- Se lanza en hilo aparte; no bloquea UI.
- Fuentes:
  - Steam (nativo y Flatpak): appmanifests y `libraryfolders.vdf`.
  - Bottles (nativo/Flatpak): busca `Program Files` y `Program Files (x86)` dentro de cada bottle.
  - Lutris: parseo básico de `.yml/.yaml` para `game.name` y `game.slug`.
  - Local: `~/Games` y rutas auxiliares (por ejemplo, EA App dentro de `~/Games`).
- Si no hay juegos, se muestran “Launchers detectados” (Steam/Bottles/Lutris y Wine) para abrir los entornos.

3) Lanzamiento seguro
- GUI ejecuta `PREPARE_GAME:<source>:<id>` → si el daemon devuelve `OK`, lanza con el runtime correcto:
  - Steam: `steam -applaunch <appid>` o `flatpak run com.valvesoftware.Steam -applaunch <appid>`.
  - Bottles: `flatpak run com.usebottles.bottles run -b <bottle> -e <exe>` o `bottles-cli` nativo.
  - Lutris: `lutris:rungame/<slug>` vía `lutris` o `flatpak run net.lutris.Lutris`.
  - Local: `wine <path>` con `cwd` = directorio del juego.
- Si la GUI corre como root, re-ejecuta los lanzadores bajo `SUDO_USER` y fija `HOME`, `XDG_RUNTIME_DIR`, `DBUS_SESSION_BUS_ADDRESS` para evitar problemas con DBus/Flatpak.

4) Diagnóstico con NT Device Proxy (GUI)
- Botón “🧪 Probar NT Proxy”: hace `GET_PROCESS_LIST` y `GET_ATTESTATION` y loguea resúmenes.
- Controles por PID:
  - `GET_THREAD_LIST:<pid>`, `GET_MODULES:<pid>`, `GET_HANDLE_TABLE:<pid>`, `GET_PROCESS_MEMORY_MAP:<pid>` y logs de resumen.

## Modelo de seguridad (resumen)

- Principio de menor privilegio:
  - La GUI no inyecta ni modifica procesos; sólo lectura/telemetría vía daemon.
  - El daemon no escribe en memoria de procesos ni realiza acciones de evasión de anti-cheat.
- Sesión de usuario correcta al lanzar:
  - Si la GUI corre con privilegios elevados, delega al usuario real (SUDO_USER) con variables de entorno correctas (HOME/XDG/DBus) para no “romper” Steam Flatpak o Lutris.
- Preflight (PREPARE_GAME):
  - Verifica disponibilidad de TPM; extensible a IMA/EVM y políticas por juego (p. ej., marcar qué juegos requieren atestación fuerte).
- Accesos /proc:
  - Consultar `/proc/<pid>` de otros usuarios puede requerir permisos; manejar errores y limitar consultas si es necesario.

## Uso desde la GUI (resumen)

- “🔍 Escanear”: detecta juegos; no bloquea UI.
- “🚀 Launchers detectados”: abre Steam/Bottles/Lutris/Wine para que el usuario gestione juegos.
- Botón “🧪 Probar NT Proxy”: prueba IOCTLs básicos.
- Campo “PID” + botones: consultas profundas del NT Device Proxy.
- Logs muestran la actividad (scan, preflight, IOCTLs, etc.).

## Uso del NT Device Proxy desde terminal

Consulta el documento `docs/nt_device_proxy.md` para ejemplos con `socat` y estructura JSON.

## Configuración

- `config.toml`: rutas y ajustes (placeholder actual). Se recomienda añadir:
  - Políticas PREPARE_GAME (requisitos de TPM/IMA por juego o por fuente).
  - Timeouts y límites de payload para IOCTLs (mapeos grandes).

## Requisitos

- Linux x86_64.
- Rust toolchain (cargo >= 1.70 recomendado).
- GTK4 + libadwaita para la GUI.
- `tpm2-tools` y acceso a `/sys/class/tpm/tpm0` para atestación extendida.

## Roadmap y extensiones propuestas

- GUI: vista en tabla con paginación y “Exportar JSON” para resultados de IOCTL; filtros por nombre/path/pid.
- Protocolo: opción CBOR/binario para mayor rendimiento y compresión de respuestas grandes.
- Seguridad: políticas PREPARE_GAME basadas en IMA/EVM y listas de juegos con requisitos; verificación de firma del kernel y de binarios críticos.
- Fuentes: integrar launchers/tiendas adicionales si es requerido.
- Daemon: caché de lecturas de `/proc` con invalidación rápida para reducir overhead en consultas frecuentes.

## Solución de problemas

- GUI no compila por GTK4: instala paquetes dev de GTK4/libadwaita de tu distribución.
- Steam/Lutris Flatpak no lanzan al correr como root: asegúrate de tener `SUDO_USER`; la GUI re-ejecuta con HOME/XDG/DBus de ese usuario.
- “Sin respuesta del daemon”: verifica proceso activo y existencia de `/tmp/kernelbridge.sock`; elimina sockets viejos; relanza.
- `GET_MODULES`/`GET_PROCESS_MEMORY_MAP` vacíos o con errores en PIDs de otros usuarios: permisos de `/proc` pueden restringir acceso.
