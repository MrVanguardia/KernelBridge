# 🧙 Cómo Usar el Asistente de Delta Force

## 🎯 Inicio Rápido

### 1. Lanzar la GUI con logs

```bash
cd ~/Documentos/PROYECTOS/kernelBridge
./kb_debug.sh
```

### 2. Hacer click en el botón del asistente

En la GUI, sección **"🎮 Juegos"**:
- Click en **"🧙 Asistente de Configuración Completa"**

### 3. Seguir las instrucciones en la terminal

El asistente te guiará automáticamente por **5 pasos**.

---

## 📋 Qué hace el asistente

### ✅ PASO 1: Verificación del Sistema

**Detecta automáticamente:**
- Sistema operativo y kernel
- GPU (AMD/NVIDIA/Intel)
- Steam Flatpak
- Delta Force instalado
- GE-Proton
- Drivers ACE

**Si algo falta, te dice cómo instalarlo:**
```
⚠️  Delta Force NO detectado
💡 Instala Delta Force desde Steam primero:
   1. Abre Steam
   2. Busca 'Delta Force'
   3. Instala el juego
   4. Vuelve a ejecutar este asistente
```

### ✅ PASO 2: Instalación de Herramientas

**Instala automáticamente:**
- GameMode (optimización CPU/GPU)
- MangoHud (overlay FPS)

**Te pedirá la contraseña de sudo** (solo una vez).

Si ya están instalados, lo detecta y continúa.

### ✅ PASO 3: Configuración de Steam

**Dos opciones:**

**A) Automática** (si encuentra el script):
- Ejecuta `fix_steam_flatpak.sh`
- Copia archivos al sandbox de Steam
- Configura rutas automáticamente

**B) Manual** (si no encuentra el script):
```
📋 CONFIGURACIÓN MANUAL:
════════════════════════════════════════════════════════════════

Ejecuta estos comandos en otra terminal:

cd ~/Documentos/PROYECTOS/kernelBridge
./fix_steam_flatpak.sh

════════════════════════════════════════════════════════════════

⏸️  Presiona Enter cuando hayas ejecutado el script...
```

**El asistente ESPERA** a que presiones Enter.

### ✅ PASO 4: Configurar Steam (Manual)

Te muestra **EXACTAMENTE** qué hacer:

```
1️⃣  Abre Steam
2️⃣  Ve a tu Biblioteca
3️⃣  Click DERECHO en Delta Force → Propiedades

4️⃣  En COMPATIBILIDAD:
    ☑️  Marca 'Forzar herramienta de compatibilidad...'
    ☑️  Selecciona: GE-Proton10-25

5️⃣  En OPCIONES DE LANZAMIENTO, pega:

┌────────────────────────────────────────────────────────────────┐
│ gamemoderun mangohud ~/.var/app/... %command% │
└────────────────────────────────────────────────────────────────┘

✅ Comando copiado al portapapeles (Ctrl+V)
```

**El comando se copia automáticamente** - solo haz Ctrl+V en Steam.

Presiona Enter cuando termines.

### ✅ PASO 5: Listo para Jugar

Te confirma que todo está configurado:
```
✅ GameMode (rendimiento máximo)
✅ MangoHud (overlay FPS)
✅ RADV + ACO (optimizaciones AMD)
✅ DXVK Async (sin stuttering)
✅ ACE configurado automáticamente
```

---

## ⚠️ Casos Especiales

### 🔴 Delta Force NO instalado

**El asistente te lo detectará:**
```
🔍 Steam:
   ✅ Steam Flatpak detectado
   ⚠️  Delta Force NO detectado
   💡 Instala Delta Force desde Steam primero
```

**Qué hacer:**
1. Abre Steam
2. Busca "Delta Force"
3. Instala el juego
4. **Vuelve a ejecutar el asistente** (click de nuevo en el botón)

### 🔴 GE-Proton NO instalado

**El asistente te lo detectará:**
```
🔍 Steam:
   ✅ Steam Flatpak detectado
   ⚠️  GE-Proton no detectado
   💡 Instala GE-Proton con ProtonUp-Qt
```

**Qué hacer:**
```bash
# Instalar ProtonUp-Qt
flatpak install flathub net.davidotek.pupgui2

# Abrir ProtonUp-Qt
flatpak run net.davidotek.pupgui2

# En la app:
# 1. Click "Add version"
# 2. Selecciona "GE-Proton" (no Luxtorpeda, no Boxtron)
# 3. Instala la versión más reciente
```

Luego **vuelve a ejecutar el asistente**.

### 🔴 Directorio ACE NO encontrado

**El asistente te lo detectará:**
```
🔍 Drivers ACE:
   ⚠️  Directorio ACE no encontrado
   💡 Busca la carpeta Win64/AntiCheatExpert
```

**Qué hacer:**

Los drivers ACE vienen **con Delta Force**. Después de instalar el juego:

```bash
# Buscar drivers ACE
find ~/.var/app/com.valvesoftware.Steam -name "ACE*.sys" -o -name "*AntiCheat*"

# Copiar al proyecto (ejemplo)
mkdir -p ~/Documentos/PROYECTOS/kernelBridge/Win64/AntiCheatExpert
cp -r <ruta_donde_encontraste_ACE>/* ~/Documentos/PROYECTOS/kernelBridge/Win64/AntiCheatExpert/
```

Luego **vuelve a ejecutar el asistente**.

### 🔴 Steam Flatpak NO instalado

```bash
flatpak install flathub com.valvesoftware.Steam
```

---

## 🎮 Después del Asistente

### Lanzar Delta Force

**Opción 1: Desde Steam (Recomendado)**
1. Abre Steam
2. Delta Force → JUGAR
3. Verás los logs en la terminal de la GUI

**Opción 2: Desde la GUI**
- Click en **"⚡ Lanzar Delta Force (Quick Start)"**

**Opción 3: Desde terminal**
```bash
deltaforce
```

---

## 🔧 Si algo sale mal

### El asistente se detuvo en Paso 3

**Causa:** No encontró `fix_steam_flatpak.sh`

**Solución:**

Abre **otra terminal** y ejecuta:
```bash
cd ~/Documentos/PROYECTOS/kernelBridge
./fix_steam_flatpak.sh
```

Luego vuelve a la terminal del asistente y **presiona Enter**.

### El asistente se detuvo en Paso 4

**Normal** - Está esperando que configures Steam.

1. Sigue las instrucciones mostradas
2. Configura Steam
3. Vuelve a la terminal del asistente
4. **Presiona Enter**

### Quiero volver a ejecutar el asistente

**Simplemente:**
1. Ve a la GUI
2. Sección **"🎮 Juegos"**
3. Click de nuevo en **"🧙 Asistente de Configuración Completa"**

El asistente se puede ejecutar **cuantas veces quieras**.

---

## 📊 Resumen Visual

```
┌─────────────────────────────────────────────────────────────┐
│                                                             │
│  1. Ejecutar: ./kb_debug.sh                                │
│                                                             │
│  2. En GUI → Juegos → Click botón del asistente           │
│                                                             │
│  3. Seguir instrucciones en terminal:                      │
│     ├─ PASO 1: Verificación (automático)                  │
│     ├─ PASO 2: Instalar herramientas (automático)         │
│     ├─ PASO 3: Copiar archivos (automático/manual)        │
│     ├─ PASO 4: Configurar Steam (manual - espera Enter)   │
│     └─ PASO 5: ¡Listo! (informativo)                      │
│                                                             │
│  4. Ir a Steam → Delta Force → JUGAR                       │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

---

## ✅ Ventajas del Asistente

| Sin Asistente | Con Asistente |
|--------------|---------------|
| Ejecutar 5+ scripts | 1 click |
| Leer 6+ documentos | Instrucciones en pantalla |
| Configurar manualmente | Automático + guía |
| 30-60 minutos | 5-10 minutos |
| Fácil equivocarse | A prueba de errores |

---

## 💡 Comandos Útiles Después

```bash
# Ver logs
deltaforce-logs

# Limpiar cache
deltaforce-clean

# Relanzar GUI
kb-debug

# Jugar
deltaforce
```

---

## 🎉 ¡Eso es todo!

**Un solo click, seguir instrucciones en pantalla, y listo para jugar.**

Mucho más fácil que ejecutar scripts manualmente. 🚀
