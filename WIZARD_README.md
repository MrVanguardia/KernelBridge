# 🧙 Asistente de Configuración Delta Force

## 🎯 ¿Qué es?

El **Asistente de Configuración** es una herramienta interactiva que te guía **paso a paso** para configurar Delta Force + ACE en Linux sin necesidad de ejecutar múltiples scripts manualmente.

**TODO se hace desde la GUI, con instrucciones en la terminal.**

---

## 🚀 Cómo usar

### 1. Lanzar la GUI en modo debug

```bash
cd ~/Documentos/PROYECTOS/kernelBridge
./kb_debug.sh
```

**O con el alias:**
```bash
kb-debug
```

### 2. En la GUI

1. Ve a la sección **"🎮 Juegos"**
2. Verás un recuadro grande: **"🎯 Delta Force + ACE Anti-Cheat"**
3. Click en el botón: **"🧙 Asistente de Configuración Completa"**

### 3. Sigue las instrucciones en la terminal

El asistente te guiará automáticamente a través de **5 pasos**:

```
╔═══════════════════════════════════════════════════════════════════════╗
║          🧙 ASISTENTE DE CONFIGURACIÓN DELTA FORCE + ACE             ║
╚═══════════════════════════════════════════════════════════════════════╝

✅ Drivers ACE (AntiCheatExpert)
✅ Steam Flatpak integration
✅ GE-Proton optimizations
✅ Optimizaciones AMD GPU
✅ GameMode + MangoHud
```

---

## 📋 Los 5 Pasos del Asistente

### Paso 1/5: Verificación del Sistema

**Automático** - El asistente verifica:
- ✅ Sistema operativo (Fedora Linux 43)
- ✅ Kernel version
- ✅ GPU (detecta AMD y configura optimizaciones RADV)
- ✅ Steam Flatpak
- ✅ Delta Force instalado
- ✅ GE-Proton
- ✅ Drivers ACE

**Salida ejemplo:**
```
📋 PASO 1/5: Verificación del Sistema

🔍 Sistema Operativo:
   Fedora Linux 43 (Workstation Edition)

🔍 Kernel:
   6.6.8-200.fc39.x86_64

🔍 GPU:
   AMD Radeon RX 6700 XT
   ✅ GPU AMD detectada - Se usarán optimizaciones RADV

🔍 Steam:
   ✅ Steam Flatpak detectado
   ✅ Delta Force instalado: Delta Force
   ✅ GE-Proton10-25 detectado

🔍 Drivers ACE:
   ✅ 4 drivers ACE encontrados
```

### Paso 2/5: Instalación de Herramientas

**Semi-automático** - Instala GameMode y MangoHud:
- Te pedirá la contraseña de sudo
- Instala automáticamente con `dnf`
- Si ya están instalados, lo detecta y continúa

**Salida ejemplo:**
```
📦 PASO 2/5: Instalación de Herramientas

Instalando GameMode y MangoHud para mejor rendimiento...

  Installing       : gamemode-1.7-1.fc39.x86_64
  Installing       : mangohud-0.7.1-1.fc39.x86_64

✅ GameMode y MangoHud instalados correctamente
```

### Paso 3/5: Configuración de Archivos

**Automático** - Ejecuta `fix_steam_flatpak.sh`:
- Copia el wrapper al sandbox de Steam
- Copia drivers ACE al sandbox
- Actualiza rutas para Steam Flatpak

**Salida ejemplo:**
```
📂 PASO 3/5: Configuración de Archivos para Steam

Ejecutando script de integración con Steam Flatpak...

[✓] Steam Flatpak detectado
[→] Creando directorios en Steam...
[→] Copiando wrapper script...
[→] Copiando drivers ACE...
[→] Actualizando rutas en el wrapper...
[✓] Wrapper configurado para Steam Flatpak

✅ Archivos copiados al sandbox de Steam correctamente
```

### Paso 4/5: Instrucciones para Steam

**Manual** - El asistente te muestra **exactamente** qué hacer en Steam:

```
⚙️  PASO 4/5: Instrucciones para Steam

════════════════════════════════════════════════════════════════
🎮 CONFIGURA STEAM AHORA
════════════════════════════════════════════════════════════════

Sigue estos pasos EN STEAM:

1️⃣  Abre Steam
2️⃣  Ve a tu Biblioteca
3️⃣  Click DERECHO en Delta Force → Propiedades

4️⃣  En la pestaña COMPATIBILIDAD:
    ☑️  Marca 'Forzar el uso de una herramienta de compatibilidad...'
    ☑️  Selecciona: GE-Proton10-25

5️⃣  En OPCIONES DE LANZAMIENTO, pega EXACTAMENTE esto:

┌────────────────────────────────────────────────────────────────┐
│ gamemoderun mangohud ~/.var/app/com.valvesoftware.Steam/data/scripts/steam_deltaforce_wrapper.sh %command% │
└────────────────────────────────────────────────────────────────┘

✅ Comando copiado al portapapeles (Ctrl+V para pegar)

6️⃣  Click Cerrar

════════════════════════════════════════════════════════════════
⏸️  PAUSA: Configura Steam ahora y luego vuelve aquí
════════════════════════════════════════════════════════════════

Presiona Enter cuando hayas terminado de configurar Steam...
```

**Importante:**
- El comando se copia automáticamente al portapapeles
- Solo necesitas hacer Ctrl+V en Steam
- El asistente **espera** a que presiones Enter

### Paso 5/5: Lanzamiento

**Informativo** - Te dice que todo está listo:

```
🚀 PASO 5/5: Lanzamiento de Delta Force

════════════════════════════════════════════════════════════════
TODO LISTO PARA JUGAR
════════════════════════════════════════════════════════════════

Ahora puedes:
  1️⃣  Ir a Steam → Delta Force → JUGAR
  2️⃣  El wrapper configurará ACE automáticamente
  3️⃣  Verás los logs aquí en esta terminal

Optimizaciones activas:
  ✅ GameMode (rendimiento máximo de CPU/GPU)
  ✅ MangoHud (overlay de FPS y estadísticas)
  ✅ RADV + ACO (optimizaciones AMD)
  ✅ DXVK Async (sin stuttering)
  ✅ ACE configurado automáticamente

════════════════════════════════════════════════════════════════
⚠️  IMPORTANTE:
════════════════════════════════════════════════════════════════
  • Modo Campaña/Offline: Debería funcionar perfectamente
  • Modo Multijugador: Puede funcionar, riesgo de detección ACE
  • NO uses en cuentas principales (riesgo de baneo)
```

---

## ✅ Después de Completar el Asistente

### Lanzar Delta Force

**Opción 1: Desde Steam (Recomendado)**
1. Abre Steam
2. Click en Delta Force
3. Click JUGAR
4. Verás los logs en la terminal de la GUI

**Opción 2: Desde la GUI**
- Click en **"⚡ Lanzar Delta Force (Quick Start)"**

**Opción 3: Desde terminal**
```bash
deltaforce
```

---

## 🔧 Ventajas del Asistente

| Característica | Sin Asistente | Con Asistente |
|---------------|---------------|---------------|
| **Scripts a ejecutar** | 5+ scripts | 1 botón |
| **Configuración manual** | Mucha | Solo Steam |
| **Errores de ruta** | Frecuentes | Ninguno |
| **Documentación a leer** | 6+ archivos MD | Guía en pantalla |
| **Tiempo de configuración** | 30-60 min | 5-10 min |
| **Verifica el sistema** | Manual | Automático |
| **Copia al clipboard** | Manual | Automático |
| **Optimizaciones** | Manual | Automático |

---

## 🐛 Troubleshooting

### ❌ "No se encontró fix_steam_flatpak.sh"

**Causa:** Ejecutando desde directorio incorrecto

**Solución:**
```bash
cd ~/Documentos/PROYECTOS/kernelBridge
./kb_debug.sh
```

### ❌ "Steam Flatpak no detectado"

**Causa:** Steam no instalado o es versión nativa

**Solución:**
```bash
# Instalar Steam Flatpak
flatpak install flathub com.valvesoftware.Steam
```

### ❌ "Delta Force no encontrado"

**Causa:** No instalado desde Steam

**Solución:**
1. Abre Steam
2. Instala Delta Force
3. Vuelve a ejecutar el asistente

### ❌ "Instalación de GameMode falló"

**Causa:** Cancelaste la contraseña de sudo

**Solución:**
- No es crítico, puedes continuar
- O reinstala manualmente: `sudo dnf install gamemode mangohud`

---

## 📊 Comparación: Manual vs Asistente

### Configuración Manual (Método anterior)

```bash
# Paso 1
./fix_steam_flatpak.sh

# Paso 2
./install_amd_tools.sh
# (ingresa contraseña)

# Paso 3
# Leer STEAM_GEPROTON_LISTO.md
# Copiar comando manualmente
# Configurar Steam manualmente

# Paso 4
# Leer AMD_OPTIMIZATIONS.md
# Ajustar configuración

# Paso 5
./verify_deltaforce.sh
```

**Tiempo estimado:** 30-60 minutos

### Con el Asistente

```bash
kb-debug
# Click en "🧙 Asistente de Configuración Completa"
# Seguir instrucciones en pantalla
```

**Tiempo estimado:** 5-10 minutos

---

## 🎯 Resultado Final

Al completar el asistente tendrás:

✅ **Wrapper instalado** en Steam Flatpak sandbox  
✅ **25 archivos ACE** disponibles  
✅ **GE-Proton** configurado  
✅ **GameMode** instalado  
✅ **MangoHud** instalado  
✅ **Optimizaciones AMD** activadas  
✅ **Steam Launch Options** configuradas  
✅ **Todo funcionando** automáticamente  

**Solo queda jugar! 🎮**

---

## 📝 Documentación Adicional

Si quieres personalizar o entender más:

- `AMD_OPTIMIZATIONS.md` - Optimizaciones específicas de GPU
- `STEAM_GEPROTON_LISTO.md` - Configuración manual de Steam
- `DEBUG_MODE.md` - Logs y debugging avanzado
- `SOLUCION_STEAM_NO_INICIA.md` - Solución de problemas

---

## 💡 Comandos Útiles Después

```bash
# Ver logs del wrapper
deltaforce-logs

# Limpiar cache de shaders (si hay problemas)
deltaforce-clean

# Relanzar GUI con debug
kb-debug

# Lanzar Delta Force directo
deltaforce
```

---

## 🎉 ¡Listo!

El asistente hace **TODO el trabajo pesado** por ti. Solo necesitas:

1. **Ejecutar la GUI** con `kb-debug`
2. **Click un botón** (Asistente de Configuración)
3. **Seguir instrucciones** en pantalla
4. **Configurar Steam** (una sola vez)
5. **JUGAR** 🎮

**Más fácil imposible!** 🧙✨
