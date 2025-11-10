# Ejemplos del NT Device Proxy

A continuación, ejemplos de respuestas en JSON (formato de una línea por respuesta). Los datos son ilustrativos y anonimizados.

## GET_PROCESS_LIST

```json
[
  { "pid": 1, "ppid": 0, "name": "init" },
  { "pid": 827, "ppid": 1, "name": "Xorg" },
  { "pid": 1452, "ppid": 827, "name": "gameclient" }
]
```

## GET_THREAD_LIST:<pid>

```json
[
  { "tid": 1452, "name": "main" },
  { "tid": 1453, "name": "render" }
]
```

## GET_MODULES:<pid>

```json
[
  { "path": "/lib/x86_64-linux-gnu/libc.so.6", "start": 140737488347136, "end": 140737489000000 },
  { "path": "/home/user/game/bin/gameclient.exe", "start": null, "end": null }
]
```

## GET_HANDLE_TABLE:<pid>

```json
[
  { "fd": 0, "target": "/dev/pts/0" },
  { "fd": 3, "target": "/home/user/.config/game/log.txt" }
]
```

## GET_PROCESS_MEMORY_MAP:<pid>

```json
[
  { "start": 94489280593920, "end": 94489280704512, "perms": "r-xp", "offset": 0, "dev": "08:01", "inode": 12345, "path": "/usr/bin/gameclient" },
  { "start": 94490000000000, "end": 94490000100000, "perms": "rw-p", "offset": 0, "dev": "00:00", "inode": 0, "path": null }
]
```

## GET_ATTESTATION

```json
{ "tpm_available": true, "ima_status": "enforcing", "evm_status": "enabled" }
```

## GET_ATTESTATION_EXT

```json
{
  "attestation": { "tpm_available": true, "ima_status": "enforcing", "evm_status": "enabled" },
  "pcrs": "sha256: 0 : 0000...\nsha256: 1 : 1111...\n..."
}
```

## CHECK_SANDBOX_VM

```json
{ "hypervisor_flag": false, "dmi_vendor": "LENOVO", "dmi_product": "20KH", "is_vm": false }
```

