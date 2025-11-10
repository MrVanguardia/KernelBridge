# Estructuras NT simuladas

Generadas dinámicamente con datos reales del sistema Linux.

## EPROCESS
- PID, nombre, PPID, número de hilos, tiempo de inicio.
- Fuente: /proc/[pid]/stat

## HANDLE_TABLE
- Lista de handles abiertos por proceso.
- Fuente: /proc/[pid]/fd

## OBJECT_HEADER
- Tipo de objeto, número de handles, flags.

## ETHREAD
- TID, PID propietario, estado del hilo.
- Fuente: /proc/[pid]/task/[tid]/status

## Uso
Estas estructuras se exponen al anti-cheat en formato NT-compatible, pero contienen datos reales y verificables de Linux.
