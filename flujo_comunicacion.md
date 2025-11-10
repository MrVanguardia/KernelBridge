# Flujo de Comunicación KernelBridge

Este documento describe cómo interactúan los principales componentes del sistema KernelBridge: GUI, Daemon y Core.

## Arquitectura General

```
┌─────────────┐      ┌─────────────┐      ┌─────────────┐
│   GUI (GTK) │◄───►│   Daemon    │◄───►│   Core      │
└─────────────┘      └─────────────┘      └─────────────┘
```

## 1. GUI (GTK 4 + libadwaita)
- Interfaz visual para el usuario.
- Envía comandos y solicitudes al daemon (por socket UNIX o D-Bus).
- Recibe respuestas y actualiza el estado (ej: TPM, módulos, logs).

## 2. Daemon (Rust)
- Servicio en segundo plano.
- Recibe comandos de la GUI.
- Gestiona módulos, verifica TPM, lanza juegos, comunica con el core.
- IPC soportado: Sockets UNIX (`/tmp/kernelbridge.sock`) y D-Bus (opcional).

## 3. Core (C/Rust)
- Implementa APIs híbridas NT y acceso a Linux.
- Expone funciones para el daemon mediante FFI, sockets o llamadas directas.

---

## Ejemplo de Flujo: Verificación de TPM

1. **GUI** inicia y solicita estado de TPM al daemon.
2. **Daemon** ejecuta `tpm2_getrandom` y responde a la GUI con el resultado.
3. **GUI** muestra el estado real del TPM al usuario.

---

## Ejemplo de Flujo: Lanzar Juego

1. **GUI** envía comando "lanzar juego" al daemon.
2. **Daemon** verifica TPM y módulos requeridos.
3. **Daemon** comunica con el core para preparar entorno NT-compatible.
4. **Daemon** lanza el juego y reporta estado a la GUI.

---

## IPC: Sockets UNIX y D-Bus
- Los comandos y respuestas se envían como mensajes JSON o texto plano.
- Ejemplo de mensaje:
  ```json
  { "cmd": "check_tpm" }
  ```
- El daemon responde:
  ```json
  { "tpm_status": "TPM detectado y operativo" }
  ```

---

## Seguridad
- La comunicación puede cifrarse si se requiere.
- El daemon valida permisos antes de ejecutar comandos críticos.

---

## Extensiones Futuras
- El core puede exponer APIs adicionales para anti-cheat y monitor de integridad.
- El daemon puede registrar eventos y logs para auditoría.

---

## Referencias
- [Relm4](https://github.com/Relm4/Relm4)
- [tpm2-tools](https://github.com/tpm2-software/tpm2-tools)
- [D-Bus IPC](https://dbus.freedesktop.org/doc/dbus-tutorial.html)
