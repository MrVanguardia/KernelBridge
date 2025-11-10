# KernelBridge: Bitácora, Guía y Manual Completo

## 🎯 Propósito y Visión

KernelBridge es un sistema experimental y plataforma de referencia para:
- Ejecutar juegos Windows en Linux con anti-cheats avanzados (ACE, EAC, etc.)
- Exponer telemetría y estructuras NT reales a anti-cheats desde Linux
- Integrar TPM, IMA/EVM, AppArmor/SELinux y monitoreo de integridad
- Automatizar la configuración, optimización y troubleshooting de juegos complejos
- Servir como base para investigación, desarrollo y documentación en la comunidad Linux gaming

---

## 🧭 Bitácora, Experimentos y Obstáculos: El Camino Real
---

## 🏗️ Arquitectura, Módulos y Límites Técnicos

### ¿Qué es KernelBridge?
Permite lanzar juegos Windows en Linux respetando el entorno real (Steam/Proton, Bottles, Lutris) y exponiendo telemetría “estilo NT” (NT Device Proxy) desde un daemon seguro. No emula ni evade anti-cheats: expone datos reales, no simulados.

### Componentes principales
- **GUI (GTK4 + Relm4):** Lista juegos, lanza con el runtime correcto, muestra logs, permite diagnóstico NT Proxy.
- **Daemon (Rust):** Servicio de fondo, gestiona módulos, expone NT Device Proxy, verifica TPM, ejecuta juegos en entornos aislados.
- **Core (Rust/C):** APIs NT híbridas, expone estructuras y syscalls NT usando datos reales de Linux.

### Módulos clave
- **tpm_manager:** Detección y atestación TPM.
- **integrity_monitor:** Estado IMA/EVM, AppArmor, SELinux, hashes y firmas.
- **anti_cheat_gateway:** Expone estructuras NT simuladas con datos reales, responde a anti-cheats sin ocultar el entorno.
- **game_launcher:** Lanza juegos tras verificar requisitos, prepara entorno seguro y reporta estado/logs.
- **event_broker, memory_auditor, kernel_validator, system_bridge_api:** Métricas, monitoreo, validación de kernel, extensiones de sistema.
- **nt_device_proxy:** Traduce IOCTLs NT a datos reales de Linux (procesos, hilos, módulos, memoria, handles, atestación TPM, sandbox/VM detection, etc.).

### Flujos y comunicación
- GUI ↔ Daemon: socket UNIX (`/tmp/kernelbridge.sock`).
- Daemon ↔ Core: FFI, sockets o llamadas directas.
- Módulos usan ptrace, process_vm_readv/writev, eBPF, TPM, IMA/EVM, AppArmor/SELinux.
- Cada juego corre en su propio namespace (PID, mount, net, etc.) para aislamiento y trazabilidad.

### Seguridad y límites reales
- Principio de menor privilegio: la GUI nunca inyecta ni modifica procesos, el daemon no evade anti-cheats.
- No hay simulación ni bypass: todo acceso y reporte es legítimo y verificable.
- Si el anti-cheat requiere drivers de kernel (ACE), no hay solución en Linux: ni emulación, ni ingeniería inversa, ni virtualización lo resuelven.

### Ejemplo de comandos y telemetría real
```bash
# Consultar procesos vía NT Device Proxy
printf 'NT_IOCTL:GET_PROCESS_LIST\n' | socat - UNIX-CONNECT:/tmp/kernelbridge.sock
# Consultar hilos de un PID
printf 'NT_IOCTL:GET_THREAD_LIST:1234\n' | socat - UNIX-CONNECT:/tmp/kernelbridge.sock
# Estado de atestación
printf 'NT_IOCTL:GET_ATTESTATION\n' | socat - UNIX-CONNECT:/tmp/kernelbridge.sock
```

### Estructuras NT simuladas (con datos reales)
- **EPROCESS:** PID, nombre, PPID, hilos, memoria, estado (de /proc)
- **ETHREAD:** TID, PID, estado (de /proc)
- **HANDLE_TABLE:** Handles abiertos (de /proc/[pid]/fd)
- **OBJECT_HEADER:** Tipo, handle_count, flags

### APIs NT implementadas
- ZwQueryInformationProcess, KeAttachProcess, ObReferenceObjectByHandle, PsLookupProcessByProcessId, ZwOpenProcess, NtReadVirtualMemory/NtWriteVirtualMemory (traducidas a /proc, ptrace, process_vm_readv/writev)

### Integridad y TPM
- Validación de kernel y binarios críticos usando TPM, IMA/EVM, Secure Boot.
- Si no hay TPM, juegos que lo requieren no se lanzan.

### Roadmap y estado
- Infraestructura base, APIs NT híbridas, integración anti-cheat, launcher seguro, GUI avanzada, empaquetado Flatpak/AppImage: **completado**.
- Módulos adicionales, métricas, validación de kernel, integración con launchers, documentación y scripts: **completado**.

### Límites y retos reales
- **ACE (AntiCheatExpert):** Incompatible con Linux/Proton por requerir drivers kernel-level, firmas digitales y validaciones imposibles de emular. Ningún bypass, emulación o traducción es viable técnica ni legalmente.
- **EAC (EasyAntiCheat):** Compatible con Proton en modo permisivo. Scripts y opciones de lanzamiento automatizan su uso cuando es posible.
- **Virtualización:** ACE detecta VMs, rendimiento pobre, no es solución real.
- **Ingeniería inversa:** Ilegal, ofuscación extrema, riesgo legal real.
- **Dual boot:** Única solución real para juegos con ACE obligatorio.

---

## 🧭 Bitácora, Experimentos y Obstáculos: El Camino Real

### 1. El Sueño: Jugar Delta Force con ACE en Linux

El objetivo era claro: lograr que Delta Force, con su anti-cheat ACE (kernel-level), funcionara en Linux usando Steam Flatpak y Proton. El proceso fue una mezcla de ilusión, creatividad, frustración y aprendizaje.

---

### 2. Primeros Intentos: Emulación y Scripts

- Se intentó emular drivers de Windows (.sys) en Wine: imposible, Wine solo ejecuta código de userspace, nunca drivers de kernel.
- Se crearon scripts para limpiar ACE y forzar EAC, con la esperanza de que el juego aceptara solo EasyAntiCheat (EAC), que sí funciona en Linux.
- Se automatizó todo: backup de drivers, limpieza de registro, configuración de launch options, integración con Steam Flatpak.

**Resultado:** EAC funciona en algunos juegos, pero Delta Force exige ACE sí o sí.

---

### 3. Creatividad Desbordada: ¿Y si hago mi propio kernel de Windows?

Se exploró la idea (absurda pero honesta) de crear un kernel de Windows open source para Linux:

- 20 años de desarrollo, cientos de millones de dólares, miles de funciones y estructuras NT, HAL, subsistemas, drivers, QA, legal, patentes…
- Incluso si se lograra, ACE detectaría que no es el kernel original (firma digital, hash, comunicación con servidores Tencent) y banearía igual.

**Moraleja:** A veces la solución más complicada no es la mejor. Dual boot con Windows toma 2 horas y funciona 100%.

---

### 4. Ingenio, Frustración y Realidad Técnica

- Se intentó pensar en módulos de kernel Linux que emularan ACE: imposible, código cerrado, verificación de integridad, comunicación cifrada con servidores.
- Se consideró la ingeniería inversa: ilegal, ofuscación extrema, años de trabajo, riesgo legal real.
- Se pensó en virtualización con GPU passthrough: ACE detecta virtualización, rendimiento pobre, más fácil dual boot.
- Se intentó “fingir” el heartbeat de ACE: protocolo desconocido, cifrado, ban instantáneo.

**Resultado:** Ningún bypass, emulación o traducción es viable técnica ni legalmente para ACE.

---

### 5. Lo que SÍ funciona y el aporte real

- Scripts para limpiar ACE y habilitar EAC (clean_ace.sh)
- Automatización de launch options, integración Flatpak, optimizaciones AMD
- Documentación exhaustiva de cada intento, error y aprendizaje
- Reflexión honesta: los límites de la compatibilidad anti-cheat en Linux

---

### 6. Moraleja para la Comunidad

> “A veces la solución más complicada no es la mejor. Y a veces, ni siquiera funciona.”

**Lección:**
- Si tu juego exige ACE, la única solución real es dual boot con Windows.
- Si acepta EAC, puedes jugar en Linux con Proton.
- Documenta tus experimentos, comparte tus scripts y ayuda a la comunidad a no perder tiempo en caminos imposibles.

---

---

## 🧠 Reflexión Final, Moraleja y Créditos

### Aprendizajes y límites reales
- No todo es posible en Linux: los anti-cheats kernel-level como ACE están diseñados para ser imposibles de emular, traducir o bypassear sin colaboración oficial.
- La documentación honesta de cada intento, error y obstáculo ahorra tiempo y frustración a otros.
- El dual boot sigue siendo la única solución real para juegos con ACE obligatorio.
- La comunidad avanza cuando se comparten tanto los éxitos como los fracasos.

### Moraleja para futuros usuarios y desarrolladores
> “No pierdas meses en lo imposible. Documenta, comparte, y ayuda a que otros no repitan los mismos errores. El conocimiento colectivo es más valioso que cualquier hack temporal.”
- A quienes leen esto buscando una respuesta real, aunque sea dura: aquí está, sin adornos.

### Motivación
Este proyecto existe para que la próxima persona que intente lo mismo tenga una referencia completa, sincera y útil. Si logras avanzar un paso más, documenta y comparte. Así se construye comunidad.

---


**¡Tu esfuerzo y curiosidad pueden inspirar a toda la comunidad!**

---

## 🧑‍💻 Reflexión Sincera y Aprendizajes Reales

Este proyecto fue, sobre todo, un ejercicio de honestidad y humildad técnica. Aprendí que, aunque la pasión y la curiosidad pueden llevarte lejos, hay límites técnicos, legales y prácticos que no se pueden forzar. Intentar hacer funcionar anti-cheats kernel-level en Linux no solo es frustrante, sino que te enseña a valorar el trabajo de los demás, a respetar las reglas del juego y a aceptar que no todo es posible, por más que lo intentes.

Fracasar en el intento no es perder el tiempo: es aprender de verdad. Documentar cada error, cada obstáculo y cada límite es la mejor forma de ayudar a otros y de crecer como desarrollador y como persona. Si este README sirve para que alguien más no repita los mismos errores, o para que una empresa entienda el lado humano y técnico de la comunidad, entonces todo el esfuerzo habrá valido la pena.

No busco reconocimiento ni problemas, solo dejar constancia de lo que es posible y lo que no, y de que la honestidad técnica es el mayor aporte que podemos hacer.

---


## 🛡️ Nota Legal, Descargo y Reflexión Personal

Este proyecto es únicamente con fines educativos, de documentación y experimentación técnica. No promueve, facilita ni incentiva el bypass, la evasión, la ingeniería inversa ni la vulneración de sistemas de seguridad, anti-cheat o software propietario. Toda la información, scripts y ejemplos aquí presentados están destinados a la interoperabilidad legítima, la investigación de compatibilidad y la transparencia técnica en entornos Linux.

**No se debe usar este proyecto para infringir términos de servicio, licencias de software, ni para actividades que violen la ley o los acuerdos de usuario de juegos o plataformas.**

El autor y los colaboradores no se hacen responsables por el uso indebido de la información o el software aquí publicado. Cada usuario es responsable de cumplir con la legislación local y los términos de los servicios y productos que utilice.

**Este experimento nació de la frustración y la curiosidad técnica, no del ánimo de lucro ni de perjudicar a nadie. Respeto profundamente el trabajo de los desarrolladores de juegos y anti-cheat, y reconozco los enormes retos de la seguridad informática. Si alguna empresa, publisher o desarrollador tiene dudas, inquietudes o considera que algo aquí puede causar un problema, por favor comuníquese antes de tomar cualquier acción. Estoy dispuesto a dialogar y aclarar cualquier malentendido.**

Este proyecto también es un testimonio de los límites, frustraciones y aprendizajes de intentar algo difícil en Linux. No busco conflictos, solo compartir lo aprendido para que otros no pierdan tiempo ni se metan en problemas.

---

*Última actualización: 10 de noviembre de 2025*