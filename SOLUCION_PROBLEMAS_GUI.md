# 🔧 Solución de Problemas - Delta Force GUI

## ❌ Error: "No se encontró quick_start_deltaforce.sh"

Este error ocurre cuando la GUI no puede encontrar el script de lanzamiento.

### ✅ Solución Rápida

**Ejecuta SIEMPRE desde el directorio del proyecto:**

```bash
cd ~/Documentos/PROYECTOS/kernelBridge
./start_gui_deltaforce.sh
```

**NO ejecutes así:**
```bash
# ❌ INCORRECTO - no funcionará
~/Documentos/PROYECTOS/kernelBridge/start_gui_deltaforce.sh

# ❌ INCORRECTO - no funcionará
./gui/target/release/kernelbridge-gui
```

---

## 🎯 Forma Correcta (Paso a Paso)

### 1. Abre una terminal

### 2. Ve al directorio del proyecto:
```bash
cd ~/Documentos/PROYECTOS/kernelBridge
```

### 3. Verifica que estés en el lugar correcto:
```bash
pwd
# Debe mostrar: /home/tu_usuario/Documentos/PROYECTOS/kernelBridge

ls quick_start_deltaforce.sh
# Debe mostrar: quick_start_deltaforce.sh
```

### 4. Ejecuta el script de inicio:
```bash
./start_gui_deltaforce.sh
```

### 5. En la GUI:
- Click en "🎮 Juegos"
- Click en "🎯 Lanzar Delta Force (Quick Start)"

---

## 🔍 Verificar Ubicación de Archivos

Asegúrate de que estos archivos existan:

```bash
cd ~/Documentos/PROYECTOS/kernelBridge

# Verificar scripts
ls -la quick_start_deltaforce.sh
ls -la start_gui_deltaforce.sh
ls -la launch_deltaforce.sh

# Verificar GUI compilada
ls -la gui/target/release/kernelbridge-gui

# Verificar drivers ACE
ls -la Win64/AntiCheatExpert/
```

Si alguno falta, recompila:

```bash
# Recompilar GUI
cd gui
cargo build --release
cd ..

# Hacer scripts ejecutables
chmod +x *.sh
```

---

## 🛠️ Si Sigue sin Funcionar

### Opción A: Usar Quick Start Directo

En lugar de la GUI, usa el script directo:

```bash
cd ~/Documentos/PROYECTOS/kernelBridge
./quick_start_deltaforce.sh
```

Esto **SIEMPRE funciona** y no depende de la GUI.

### Opción B: Verificar Sistema

```bash
cd ~/Documentos/PROYECTOS/kernelBridge
./verify_deltaforce.sh
```

Esto te dirá exactamente qué falta.

---

## 📊 Logs de la GUI

Si quieres ver qué está buscando exactamente la GUI, mira los logs:

1. Abre la GUI: `./start_gui_deltaforce.sh`
2. Ve a "🧠 KernelBridge" 
3. Los logs mostrarán todas las rutas donde buscó

Deberías ver algo como:
```
[DELTA FORCE] Buscando en: /home/usuario/Documentos/PROYECTOS/kernelBridge/quick_start_deltaforce.sh
[DELTA FORCE] ✅ Encontrado: ...
```

---

## 💡 Recordatorio Importante

**SIEMPRE ejecuta desde el directorio del proyecto:**

```bash
# 1. Ir al directorio
cd ~/Documentos/PROYECTOS/kernelBridge

# 2. Verificar ubicación
pwd

# 3. Ejecutar
./start_gui_deltaforce.sh
```

**Esto evita el 99% de los problemas.**

---

## 🚀 Alternativa: Crear Lanzador de Escritorio

Si quieres hacer doble click desde el escritorio:

```bash
# Crear archivo .desktop
cat > ~/.local/share/applications/deltaforce.desktop << 'EOF'
[Desktop Entry]
Name=Delta Force (KernelBridge)
Exec=bash -c 'cd ~/Documentos/PROYECTOS/kernelBridge && ./start_gui_deltaforce.sh'
Icon=applications-games
Type=Application
Categories=Game;
Terminal=true
EOF

# Hacerlo ejecutable
chmod +x ~/.local/share/applications/deltaforce.desktop
```

Ahora aparecerá en tu menú de aplicaciones como "Delta Force (KernelBridge)".

---

## ✅ Resumen

1. **SIEMPRE** ejecuta desde `~/Documentos/PROYECTOS/kernelBridge`
2. Usa `./start_gui_deltaforce.sh` para la GUI
3. O usa `./quick_start_deltaforce.sh` directo (más confiable)
4. Verifica con `./verify_deltaforce.sh` si hay problemas

**¡Eso es todo!** 🎮
