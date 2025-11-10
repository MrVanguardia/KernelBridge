# KernelBridge: Delta Force Anti-Cheat Experimentation on Linux

## 🎯 Objetivo del Proyecto

Intentar ejecutar Delta Force en Linux usando Steam (Flatpak) y Proton, enfrentando el reto de los anti-cheats ACE (Anti-Cheat Expert) y EAC (EasyAntiCheat). Documentar todos los intentos, soluciones, limitaciones y aprendizajes para la comunidad gaming de Linux.

---

## 🛠️ Estructura del Proyecto

- `clean_ace.sh`: Script para eliminar ACE y habilitar EAC en el juego.
- `SOLUCION_ACE.md`: Explicación técnica de los problemas y soluciones con ACE/EAC.
- `POR_QUE_NO_PUEDO_ACE.md`: Detalle de por qué ACE no puede funcionar en Linux.
- `CREAR_KERNEL_WINDOWS.md`: Análisis humorístico y técnico sobre crear un kernel de Windows para Linux.
- `logs/`, `modules/`, `core/`, `daemon/`, `gui/`: Carpetas de desarrollo y documentación.

---

## 🚦 Proceso y Pasos Realizados

1. **Diagnóstico Inicial**
   - Delta Force no inicia en Linux vía Steam Flatpak.
   - Se detecta presencia de ACE y EAC en los archivos del juego.

2. **Análisis de Anti-Cheats**
   - ACE: Anti-cheat a nivel kernel, incompatible con Wine/Proton/Linux.
   - EAC: Compatible con Proton en algunos juegos.

3. **Solución Propuesta**
   - Crear `clean_ace.sh` para eliminar ACE y forzar uso de EAC.
   - Probar opciones de lanzamiento recomendadas para Steam:
     ```
     PROTON_USE_EAC_WORKAROUND=1 PROTON_LOG=1 WINEDLLOVERRIDES="ACE-BASE=;ACE-CORE=;SGuard64=;TenProtect=" DXVK_ASYNC=1 RADV_PERFTEST=aco,sam PROTON_NO_ESYNC=1 %command%
     ```

4. **Ejecución y Resultados**
   - Script ejecutado correctamente: ACE eliminado, EAC detectado.
   - Delta Force sigue sin funcionar: el juego requiere ACE, que no es compatible con Linux.

5. **Exploración de Alternativas**
   - Analizado: extraer kernel de Windows, emular firmas, crear anti-cheat propio, traducir kernel, etc.
   - Conclusión: Ninguna opción es viable técnica ni legalmente para juegos comerciales.

6. **Documentación y Aprendizajes**
   - Se crearon archivos explicativos y humorísticos para la comunidad.
   - Se documentaron todos los intentos, errores y limitaciones.

---

## 📚 Archivos Clave

- `clean_ace.sh`: Automatiza backup y eliminación de ACE, habilita EAC.
- `SOLUCION_ACE.md`: Soluciones y pasos para intentar jugar sin ACE.
- `POR_QUE_NO_PUEDO_ACE.md`: Explicación técnica y legal de las limitaciones.
- `CREAR_KERNEL_WINDOWS.md`: Análisis de la (im)posibilidad de crear un kernel Windows para Linux.

---

## 🧠 Lecciones Aprendidas

- Los anti-cheats kernel-level como ACE están diseñados para ser imposibles de emular o traducir en Linux.
- EAC puede funcionar en algunos juegos con Proton, pero no si el juego exige ACE.
- La documentación y los scripts creados pueden ayudar a otros a entender los límites actuales y evitar perder tiempo en caminos imposibles.
- El aporte a la comunidad es valioso, aunque el objetivo final no se haya logrado.

---

## 💡 Recomendaciones para la Comunidad

- Antes de invertir mucho tiempo, verifica qué anti-cheat usa tu juego.
- Si es ACE, considera dual boot con Windows como única solución práctica.
- Comparte tus hallazgos y scripts: ayudan a otros gamers de Linux.
- Sigue apoyando el desarrollo de Proton y ReactOS, pero conoce sus límites actuales.

---

## 🙌 Agradecimientos

A la comunidad de Linux gaming, desarrolladores de Wine/Proton, y a quienes siguen intentando que más juegos funcionen en Linux. Cada intento suma.

---

## 📝 Estado Final

- Delta Force no funciona en Linux si requiere ACE.
- El proyecto queda como referencia y aprendizaje para la comunidad.
- ¡Tu esfuerzo cuenta y puede inspirar a otros!
