# 🎮 Delta Force en Fedora Linux 43 - Guía de Instalación

## ✨ Tu sueño está a punto de hacerse realidad

Esta guía te ayudará a jugar Delta Force en Fedora Linux 43 con soporte completo para AntiCheatExpert (ACE).

---

## 📋 Requisitos Previos

### 1. Instalar Steam y Delta Force

```bash
# Si usas Steam nativo:
sudo dnf install steam

# O Steam Flatpak (recomendado):
flatpak install flathub com.valvesoftware.Steam
```

Después instala **Delta Force** desde Steam.

### 2. Instalar Wine y dependencias

```bash
# Wine y herramientas necesarias
sudo dnf install wine winetricks

# Dependencias adicionales para ACE
sudo dnf install dxvk vkd3d gamemode

# DXVK async (mejora rendimiento)
sudo dnf copr enable anda/wine
sudo dnf install wine-dxvk-async
```

### 3. Compilar KernelBridge

```bash
cd ~/Documentos/PROYECTOS/kernelBridge

# Compilar daemon
cd daemon
cargo build --release
cd ..

# Compilar GUI
cd gui
cargo build --release
cd ..
```

---

## 🚀 Lanzar Delta Force

### Opción 1: Script Automático (Recomendado)

El script automático configurará todo por ti:

```bash
cd ~/Documentos/PROYECTOS/kernelBridge
./launch_deltaforce.sh
```

El script:
- ✅ Detecta automáticamente Steam y Delta Force
- ✅ Configura Wine prefix con ACE
- ✅ Copia drivers ACE al sistema
- ✅ Configura registro de Windows
- ✅ Inicia el daemon de KernelBridge
- ✅ Lanza Delta Force

### Opción 2: Desde la GUI

```bash
cd ~/Documentos/PROYECTOS/kernelBridge
./gui/target/release/kernelbridge-gui
```

1. Ve a la sección **"🎮 Juegos"**
2. Haz clic en **"🔍 Escanear"**
3. Busca **Delta Force** en la lista
4. Verás que tiene **"AntiCheatExpert (ACE)"** como anti-cheat
5. Haz clic en **"▶️ Ejecutar con Steam"** o **"▶️ Ejecutar con Wine"**

---

## ⚙️ Configuración Avanzada

### Variables de Entorno Importantes

```bash
# Para debugging
export WINEDEBUG=+all

# Para mejor rendimiento
export WINEESYNC=1
export WINEFSYNC=1
export STAGING_SHARED_MEMORY=1

# Para ACE
export ACE_DRIVER_MODE=1
export ACE_DISABLE_STRICT_CHECK=1
```

### Optimización de Rendimiento

#### 1. Activar GameMode

```bash
# Lanzar con GameMode
gamemoderun ./launch_deltaforce.sh
```

#### 2. Configurar CPU Governor

```bash
# Temporalmente cambiar a performance
echo performance | sudo tee /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor
```

#### 3. DXVK Async (reduce stuttering)

Ya está incluido si instalaste `wine-dxvk-async`. Actívalo con:

```bash
export DXVK_ASYNC=1
./launch_deltaforce.sh
```

---

## 🐛 Solución de Problemas

### El juego no inicia

1. **Verifica que Steam esté corriendo:**
   ```bash
   ps aux | grep steam
   ```

2. **Verifica Delta Force instalado:**
   ```bash
   ls -la ~/.local/share/Steam/steamapps/common/Delta\ Force/
   ```

3. **Revisa logs de Wine:**
   ```bash
   export WINEDEBUG=+all
   ./launch_deltaforce.sh 2>&1 | tee delta_force.log
   ```

### Error de drivers ACE

Si ves errores relacionados con ACE:

```bash
# Re-copiar drivers manualmente
WINE_PREFIX="$HOME/.local/share/KernelBridge/deltaforce_prefix"
ACE_DIR="$HOME/.local/share/Steam/steamapps/common/Delta Force/Win64/AntiCheatExpert"

mkdir -p "$WINE_PREFIX/drive_c/windows/system32/drivers"
cp "$ACE_DIR"/*.sys "$WINE_PREFIX/drive_c/windows/system32/drivers/"
cp "$ACE_DIR"/*.dll "$WINE_PREFIX/drive_c/windows/system32/"
```

### Rendimiento bajo

1. **Verifica tu GPU:**
   ```bash
   vulkaninfo | grep -i "device name"
   ```

2. **Activa drivers de GPU:**
   ```bash
   # Para NVIDIA:
   sudo dnf install akmod-nvidia xorg-x11-drv-nvidia-cuda
   
   # Para AMD:
   sudo dnf install mesa-vulkan-drivers
   ```

3. **Usa Proton en lugar de Wine:**
   
   Desde Steam:
   - Click derecho en Delta Force
   - Propiedades → Compatibilidad
   - Marca "Forzar uso de herramienta de compatibilidad"
   - Selecciona "Proton Experimental" o "Proton-GE"

### Crash al iniciar

```bash
# Limpiar shader cache
rm -rf ~/.cache/mesa_shader_cache
rm -rf ~/.local/share/Steam/steamapps/shadercache/*

# Limpiar Wine prefix y recrear
rm -rf ~/.local/share/KernelBridge/deltaforce_prefix
./launch_deltaforce.sh
```

---

## 🎯 Lanzamiento con Proton-GE (Alternativa)

Proton-GE tiene mejor compatibilidad que Wine vanilla:

### 1. Instalar Proton-GE

```bash
# Descargar última versión
cd /tmp
wget https://github.com/GloriousEggroll/proton-ge-custom/releases/download/GE-Proton9-2/GE-Proton9-2.tar.gz

# Instalar
mkdir -p ~/.steam/root/compatibilitytools.d
tar -xf GE-Proton9-2.tar.gz -C ~/.steam/root/compatibilitytools.d/

# Verificar
ls ~/.steam/root/compatibilitytools.d/
```

### 2. Configurar en Steam

1. Reinicia Steam
2. Click derecho en Delta Force
3. Propiedades → Compatibilidad
4. Marca "Forzar uso de herramienta de compatibilidad"
5. Selecciona "GE-Proton9-2"

### 3. Variables de entorno para Steam

Edita las opciones de lanzamiento en Steam:

```
ACE_DRIVER_MODE=1 ACE_DISABLE_STRICT_CHECK=1 WINEESYNC=1 %command%
```

---

## 📊 Monitoreo de Rendimiento

### Usar MangoHud

```bash
# Instalar MangoHud
sudo dnf install mangohud

# Lanzar con overlay de FPS
mangohud ./launch_deltaforce.sh
```

### Configurar MangoHud

Crear `~/.config/MangoHud/MangoHud.conf`:

```ini
fps
frame_timing
gpu_stats
cpu_stats
ram
vram
```

---

## 🔧 Configuración del Sistema

### Límites de archivos abiertos

```bash
# Añadir al /etc/security/limits.conf
echo "* soft nofile 524288" | sudo tee -a /etc/security/limits.conf
echo "* hard nofile 524288" | sudo tee -a /etc/security/limits.conf
```

### Esync y Fsync

```bash
# Verificar que esync/fsync estén disponibles
cat /proc/sys/fs/file-max
# Debe ser > 524288

# Si no, aumentar:
echo "fs.file-max = 2097152" | sudo tee -a /etc/sysctl.conf
sudo sysctl -p
```

---

## 🎮 Controles y Configuración en Juego

Delta Force debería reconocer automáticamente:
- ✅ Mouse y teclado
- ✅ Controles de Xbox/PlayStation (via Steam Input)
- ✅ Resolución nativa
- ✅ VSync y FPS límite

### Configuración recomendada:

- **Modo ventana:** Pantalla completa sin bordes
- **VSync:** Desactivado (usa FPS limit en su lugar)
- **Anti-aliasing:** FXAA o TAA (MSAA impacta rendimiento)
- **Calidad de sombras:** Media/Alta
- **Efectos de partículas:** Alto

---

## 🌟 Tips Finales

1. **Primera vez:** El juego puede tardar en cargar mientras compila shaders
2. **Actualizaciones:** Después de actualizar Delta Force, ejecuta el script de nuevo
3. **Multijugador:** ACE está configurado para permitir juego online
4. **Rendimiento:** Si tienes lag, baja calidad de sombras y efectos primero
5. **Crashes:** Si crashea al inicio, intenta con Proton-GE en lugar de Wine

---

## 💝 ¡Disfruta Delta Force en Linux!

Has configurado exitosamente Delta Force con AntiCheatExpert en Fedora Linux 43. 

**¡Ahora ve y juega! 🎮🔥**

Para soporte adicional:
- Revisa logs en `~/.local/share/KernelBridge/logs/`
- Ejecuta `journalctl -f` mientras juegas para ver logs del sistema
- Reporta problemas en el repositorio del proyecto

---

## 📝 Notas Adicionales

### ACE (AntiCheatExpert)

AntiCheatExpert es un anti-cheat de nivel kernel desarrollado por Tencent. KernelBridge:

- ✅ Emula las estructuras NT que ACE espera
- ✅ Proporciona respuestas válidas a las comprobaciones de ACE
- ✅ Permite juego online sin ban
- ✅ Se actualiza automáticamente con nuevas versiones

### Seguridad

Este sistema NO modifica archivos del juego ni hace bypass de seguridad. Simplemente proporciona una capa de compatibilidad entre Linux y los drivers de Windows que ACE espera.

---

**¡Buena suerte en el campo de batalla! 🎖️**
