# Módulos Adicionales Recomendados

## Event Broker
Servicio ligero que registra eventos del sistema (cambios en /proc, logs de auditd) y los envía en tiempo real al Anti-Cheat Gateway.

## Memory Auditor
Monitorea regiones de memoria del juego usando ptrace y reporta accesos no autorizados al Integrity Monitor.

## Kernel Validator
Valida hashes del kernel y módulos críticos mediante TPM, asegurando integridad del sistema base.

## System Bridge API
Capa de comunicación entre el daemon y los juegos, permitiendo que los juegos accedan a APIs NT híbridas de forma segura.