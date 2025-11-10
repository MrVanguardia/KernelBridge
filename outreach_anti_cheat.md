# Brief técnico y outreach para propietarios de sistemas anti‑cheat

Este documento resume el estado actual de KernelBridge y propone líneas de colaboración con propietarios de sistemas anti‑cheat. Incluye una síntesis técnica y plantillas de contacto.

## Resumen ejecutivo
- Objetivo: jugar en Linux respetando las políticas de seguridad de cada proveedor.
- Enfoque: no emulamos ni evadimos anti‑cheats. Nos alineamos con los caminos soportados por cada proveedor en Linux (cuando existan).
- Complemento: para títulos compatibles, ofrecemos telemetría "NT-like" desde user‑space (sin hooks ni parches de kernel) para facilitar observabilidad y diagnóstico.

## Qué ya tenemos
- GUI + daemon con IPC seguro y cierre limpio.
- Biblioteca de juegos (Steam/Bottles/Lutris) y lanzamiento correcto bajo el runtime nativo.
- NT Device Proxy con IOCTLs:
  - Procesos, hilos, módulos, tabla de handles, mapa de memoria.
  - Atestación (TPM/IMA/EVM opcional).
  - Señales heurísticas no intrusivas: CHECK_PROCESS_SECURITY, CHECK_DEBUG, CHECK_SANDBOX_VM, CHECK_MULTICLIENT.

## Lo que pedimos a los proveedores
- Confirmar y documentar el camino soportado para ejecución en Linux (donde aplique) y condiciones necesarias para habilitar juego online.
- Opcional: exponer endpoints de verificación de salud para clientes Linux.
- Aceptar telemetría de diagnóstico opcional (no obligatoria) que ayude a los usuarios a detectar mala configuración.

## Puntos técnicos clave
- KernelBridge no inyecta código en procesos ni modifica binarios del proveedor.
- Toda la recolección de datos es desde /proc y herramientas estándar del sistema.
- La atestación aprovecha TPM y mecanismos de integridad del kernel (IMA/EVM) cuando están disponibles.
- Para juegos que requieren componentes de kernel no disponibles en Linux, seguiremos las recomendaciones oficiales del proveedor para ofrecer la mejor experiencia compatible sin comprometer la seguridad.

## Próximos pasos sugeridos
- Establecer un canal técnico para compartir requisitos de compatibilidad.
- Validar una lista de títulos y condiciones (Proton OK / VM requerida / no soportado).
- Intercambiar formatos mínimos de telemetría para diagnóstico (opt‑in del usuario).

## Plantillas de correo

### Español
Asunto: Colaboración para compatibilidad segura en Linux – Proyecto KernelBridge

Hola equipo de <Proveedor>,

Somos desarrolladores de KernelBridge, una capa de integración que permite gestionar juegos y telemetría en Linux sin evadir ni emular anti‑cheats. Para títulos compatibles con Proton/Wine, usamos el runtime nativo sin modificar binarios.

Nos gustaría confirmar las condiciones para habilitar el juego online en Linux y alinear requisitos técnicos. Adjuntamos un brief con nuestro enfoque y capacidades actuales (telemetría NT-like user‑space, atestación TPM/IMA/EVM).

¿Podemos coordinar una breve reunión técnica? También podemos compartir logs y trazas anónimas de diagnóstico (opt‑in) para acelerar la validación.

Gracias y quedamos atentos,
<Nombre>
<Organización/Proyecto>
<Contactos>

### English
Subject: Collaboration on secure Linux compatibility – KernelBridge project

Hello <Vendor> team,

We are the developers of KernelBridge, an integration layer that manages game launching and diagnostics on Linux without bypassing or emulating anti‑cheats. For Proton/Wine compatible titles, we use the native runtime without modifying binaries.

We’d like to confirm the conditions for enabling online play on Linux and align on technical requirements. Attached is a brief describing our approach and current capabilities (user‑space NT‑like telemetry, TPM/IMA/EVM attestation).

Could we schedule a short technical call? We can also share opt‑in anonymized diagnostic traces to speed up validation.

Best regards,
<Name>
<Org/Project>
<Contact>
