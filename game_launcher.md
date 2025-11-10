# Game Launcher Seguro

Este módulo lanza juegos desde la GUI con configuración segura y controlada.

## Funciones
- Verifica compatibilidad con TPM y anti-cheat antes de ejecutar el juego.
- Prepara entorno NT-compatible usando KernelBridge Core y APIs híbridas.
- Lanza el juego en un namespace aislado, controlado por el daemon.
- Reporta estado y logs a la GUI y al Anti-Cheat Gateway.

## Seguridad
- Bloquea juegos que requieren TPM si no está disponible.
- Usa AppArmor o SELinux para mantener aislamiento seguro.
- Registra eventos y accesos para auditoría.

## Tecnologías
- Rust, C
- GTK 4, libadwaita
- Sockets UNIX, D-Bus
- TPM, IMA/EVM, AppArmor, SELinux

## Ejemplo de flujo
1. Usuario selecciona juego en la GUI.
2. Launcher verifica TPM y módulos requeridos.
3. Prepara entorno NT-compatible y lanza el juego.
4. Reporta estado y logs a la GUI y anti-cheat.
