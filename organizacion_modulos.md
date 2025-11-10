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

## Referencias
- Ver archivos en docs/ para detalles de cada módulo.
