# KernelBridge – One‑pager para proveedores

Resumen técnico para evaluar compatibilidad en Linux sin emulación ni evasión.

- Objetivo: permitir jugar en Linux respetando políticas de seguridad.
- Enfoque: nada de inyección ni bypass; telemetría de solo lectura desde un daemon seguro.
- Integración: GUI + daemon con IPC por socket UNIX.
- Lanzamiento: usa el runtime nativo (Steam/Proton, Bottles, Lutris, Wine local) bajo la sesión correcta del usuario.
- NT Device Proxy: expone IOCTLs lógicos NT‑like con datos reales de /proc.
- Atestación: TPM e integridad (IMA/EVM) cuando están disponibles.
- Heurísticas no intrusivas: CHECK_PROCESS_SECURITY, CHECK_DEBUG, CHECK_SANDBOX_VM, CHECK_MULTICLIENT.
- Privacidad: todo opt‑in para compartición; datos permanecen locales salvo consentimiento explícito.

Arquitectura (simplificada):

```mermaid
flowchart LR
GUI[GUI (GTK4/Relm4)] -- UNIX socket --> Daemon[Daemon]
Daemon --> NTProxy[NT Device Proxy]
Daemon --> TPM[TPM/IMA/EVM]
Daemon --> Modules[Módulos: integrity, launcher, auditor]
```

Valores de seguridad:
- Mínimo privilegio y sin escritura en memoria de procesos.
- Sin parches de kernel ni ganchos de evasión.
- Rutas de lanzamiento compatibles y verificables.

Qué necesitamos de ustedes:
- Condiciones/requisitos para habilitar juego online en Linux (si existen).
- Pautas sobre telemetría mínima aceptable para diagnóstico (opt‑in).

