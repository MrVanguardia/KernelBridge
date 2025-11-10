# NT Device Proxy

Capa en el daemon que traduce IOCTLs lógicos “estilo NT” a datos reales de Linux. Expone información de procesos, hilos, módulos (mapeos), "handles" (FDs), memoria y atestación del sistema.

## Transporte

- UNIX socket: `/tmp/kernelbridge.sock`
- Petición: `NT_IOCTL:<COMANDO>\n`
- Respuesta: una única línea JSON (UTF-8). En caso de error: `{ "error": "..." }`

Ejemplo con `socat`:

```sh
# Lista de procesos
printf 'NT_IOCTL:GET_PROCESS_LIST\n' | socat - UNIX-CONNECT:/tmp/kernelbridge.sock

# Hilos de un PID
printf 'NT_IOCTL:GET_THREAD_LIST:1234\n' | socat - UNIX-CONNECT:/tmp/kernelbridge.sock
```

## Comandos

### GET_PROCESS_LIST

Devuelve `Array` de procesos accesibles.

Elemento:
- `pid` (int)
- `ppid` (int)
- `name` (string)

```json
[
  { "pid": 1, "ppid": 0, "name": "systemd" },
  { "pid": 123, "ppid": 1, "name": "Xorg" }
]
```

### GET_THREAD_LIST:<pid>

Devuelve `Array` con hilos del proceso.

Elemento:
- `tid` (int)
- `name` (string|null)

```json
[
  { "tid": 1234, "name": "Worker-1" },
  { "tid": 1235, "name": null }
]
```

### GET_MODULES:<pid>

Devuelve `Array` de módulos/archivos mapeados (derivados de `/proc/<pid>/maps`).

Elemento:
- `path` (string) — ruta absoluta del archivo
- `start` (u64|null) — dirección inicio
- `end` (u64|null) — dirección fin

```json
[
  { "path": "/usr/lib/libc.so.6", "start": 140735000000, "end": 140735012000 },
  { "path": "/home/user/bin/game", "start": 4194304, "end": 5242880 }
]
```

### GET_HANDLE_TABLE:<pid>

Devuelve `Array` de handles estilo NT; en Linux corresponde a descriptores de archivo.

Elemento:
- `fd` (int)
- `target` (string) — destino del enlace en `/proc/<pid>/fd/<fd>`

```json
[
  { "fd": 0, "target": "/dev/pts/1" },
  { "fd": 3, "target": "socket:[12345]" }
]
```

### GET_PROCESS_MEMORY_MAP:<pid>

Devuelve `Array` de regiones del mapa de memoria del proceso (`/proc/<pid>/maps`).

Elemento:
- `start` (u64)
- `end` (u64)
- `perms` (string) — ej. "r-xp"
- `offset` (u64)
- `dev` (string)
- `inode` (u64)
- `path` (string|null)

```json
[
  { "start": 4194304, "end": 5242880, "perms": "r-xp", "offset": 0, "dev": "08:01", "inode": 123, "path": "/home/user/bin/game" }
]
```

### GET_ATTESTATION

Estado base de atestación del host.

```json
{ "tpm_available": true, "ima_status": "enforcing", "evm_status": "enabled" }
```

### GET_ATTESTATION_EXT

Versión extendida (puede incluir PCRs u otros campos según disponibilidad de `tpm2-tools`).

```json
{
  "attestation": { "tpm_available": true, "ima_status": "enforcing", "evm_status": "enabled" },
  "pcrs": "sha256:\n  0 : 000000...\n  1 : abcdef...\n" 
}
```

### CHECK_PROCESS_SECURITY:<pid>

Heurísticas de seguridad no intrusivas sobre un proceso.

Respuesta (ejemplo):

```json
{
  "pid": 1234,
  "has_wx_pages": false,
  "has_anon_exec": true,
  "has_memfd_exec": false,
  "is_debugged": false,
  "tracer_pid": 0,
  "suspicious_regions": ["[anon:exec]"]
}
```

### CHECK_DEBUG:<pid>

Devuelve `TracerPid` e inferencia de depuración.

```json
{ "pid": 1234, "tracer_pid": 0, "is_debugged": false }
```

### CHECK_SANDBOX_VM

Señales de virtualización/sandbox (CPU flag “hypervisor” y DMI vendor/product).

```json
{ "hypervisor_flag": true, "dmi_vendor": "QEMU", "dmi_product": "Standard PC", "is_vm": true }
```

### CHECK_MULTICLIENT:<name>

Cuenta procesos cuyo `comm` contiene `<name>`.

```json
{ "name": "game.exe", "count": 2 }
```

## Errores

- Respuesta con `{ "error": "mensaje" }` en caso de parámetros faltantes (por ejemplo, PID inválido) o comando desconocido.

## Seguridad y permisos

- Consultar `/proc/<pid>` de procesos de otro usuario puede requerir permisos adicionales.
- La atestación extendida requiere `tpm2-tools` y acceso al TPM.
- No se exponen operaciones de escritura ni inyección: este proxy es de sólo lectura para telemetría.

## Relación con la GUI

La GUI usa el botón “🧪 Probar NT Proxy” y controles por PID para invocar estos IOCTLs y mostrar resúmenes en los logs. Para inspecciones profundas se recomienda agregar una vista de tabla y exportar JSON (planeado en el roadmap).
