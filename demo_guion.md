# Guion de demo rápida (GUI + Daemon)

Objetivo: mostrar en 2–3 minutos el flujo principal y la telemetría NT‑like.

## Preparación

1) Compilar (opcional; si ya tienes binarios en `target/release`, salta este paso):

```sh
cd gui && cargo build --release
cd ../daemon && cargo build --release
```

2) Iniciar componentes:

```sh
./start.sh
```

## Demo (GUI)

1) Vista “🧠 KernelBridge”
   - Clic en “🧪 Probar NT Proxy” → debe registrar resumen de procesos y atestación.
   - Clic en “🌐 Diagnóstico de red” (opcional) para validar conectividad a tiendas/servicios.
   - Clic en “🧾 Exportar diagnóstico” → genera `~/KernelBridgeDiagnostics/diag_<timestamp>.json` con:
     - GET_PROCESS_LIST
     - GET_ATTESTATION(_EXT)
     - CHECK_SANDBOX_VM

2) Vista “🎮 Juegos”
   - “🔍 Escanear” → lista juegos detectados (Steam/Bottles/Lutris/Local).
   - “🚀 Launchers detectados” → abrir Steam/Bottles/Lutris según disponibilidad.

## Notas

- Si la GUI corre elevada, re‑ejecuta lanzadores bajo el usuario real (HOME/XDG/DBus correctos).
- La telemetría es de solo lectura y no modifica procesos del sistema.
