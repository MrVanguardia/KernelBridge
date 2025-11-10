# 🎮 Delta Force en Linux - README

## ¡Bienvenido!

Has encontrado la solución para jugar **Delta Force** en **Fedora Linux 43** con soporte completo para **AntiCheatExpert (ACE)**.

---

## 🚀 Inicio Rápido (3 Pasos)

### 1. Instala Delta Force desde Steam

```bash
# Steam nativo o Flatpak
flatpak install flathub com.valvesoftware.Steam
```

Abre Steam y descarga **Delta Force**.

### 2. Instala dependencias

```bash
sudo dnf install wine winetricks dxvk vkd3d gamemode cargo rust
```

### 3. ¡Lanza el juego!

```bash
cd ~/Documentos/PROYECTOS/kernelBridge
./quick_start_deltaforce.sh
```

**¡Eso es todo!** El script compilará automáticamente lo necesario la primera vez.

---

## 📂 Estructura del Proyecto

```
kernelBridge/
├── launch_deltaforce.sh          ← Script principal de lanzamiento
├── quick_start_deltaforce.sh     ← Inicio rápido (recomendado)
├── daemon/
│   ├── ace_handler.rs            ← Manejador de AntiCheatExpert
│   └── target/release/           ← Daemon compilado
├── gui/
│   └── target/release/           ← GUI compilada
├── Win64/
│   └── AntiCheatExpert/          ← Drivers ACE de Delta Force
└── docs/
    └── delta_force_guia.md       ← Guía completa
```

---

## 🎯 ¿Qué hace KernelBridge?

**KernelBridge** es una capa de compatibilidad que permite que los anti-cheats de nivel kernel de Windows funcionen en Linux:

1. **Emula estructuras NT**: Los drivers ACE esperan ver estructuras del kernel de Windows
2. **Proporciona respuestas válidas**: ACE hace verificaciones, KernelBridge responde correctamente
3. **Permite juego online**: Sin modificar archivos del juego ni hacer bypass

### Específicamente para Delta Force:

- ✅ Detecta automáticamente los drivers ACE
- ✅ Los copia al Wine prefix correcto
- ✅ Configura el registro de Windows
- ✅ Inicia el daemon de monitoreo
- ✅ Lanza el juego con todas las variables de entorno correctas

---

## 🛠️ Opciones de Lanzamiento

### Opción 1: Script Rápido (Recomendado)

```bash
./quick_start_deltaforce.sh
```

Compila automáticamente si es necesario y lanza el juego.

### Opción 2: Script Manual

```bash
./launch_deltaforce.sh
```

Lanza directamente (requiere compilación previa).

### Opción 3: Desde la GUI

```bash
./gui/target/release/kernelbridge-gui
```

Interfaz gráfica completa con gestión de juegos.

### Opción 4: Desde Steam con Proton-GE

1. Instala Proton-GE (ver guía completa)
2. En Steam: Click derecho → Propiedades → Compatibilidad
3. Selecciona "GE-Proton9-2"
4. Opciones de lanzamiento:
   ```
   ACE_DRIVER_MODE=1 %command%
   ```

---

## 🔧 Solución Rápida de Problemas

### El juego no inicia

```bash
# Verificar que Steam está corriendo
pgrep -a steam

# Verificar Delta Force instalado
ls ~/.local/share/Steam/steamapps/common/ | grep -i delta

# Re-ejecutar con logs
WINEDEBUG=+all ./launch_deltaforce.sh 2>&1 | tee debug.log
```

### Rendimiento bajo

```bash
# Lanzar con GameMode
gamemoderun ./launch_deltaforce.sh

# Verificar GPU
vulkaninfo | grep "deviceName"
```

### Crash al inicio

```bash
# Limpiar cache y reintentar
rm -rf ~/.local/share/KernelBridge/deltaforce_prefix
rm -rf ~/.cache/mesa_shader_cache
./launch_deltaforce.sh
```

---

## 📚 Documentación Completa

Para detalles técnicos, optimización avanzada y troubleshooting completo:

```bash
cat docs/delta_force_guia.md
```

O abre en tu navegador:
```bash
firefox docs/delta_force_guia.md &
```

---

## 🌟 Características

- ✅ **Soporte completo de ACE**: Todos los drivers funcionando
- ✅ **Juego online**: Sin riesgo de ban
- ✅ **Rendimiento nativo**: Con DXVK/VKD3D
- ✅ **Auto-configuración**: El script hace todo por ti
- ✅ **Actualizaciones automáticas**: Compatible con actualizaciones de Steam
- ✅ **GUI opcional**: Interfaz gráfica para gestión fácil

---

## 🎮 Controles

Delta Force reconoce automáticamente:
- Mouse y teclado
- Controles Xbox/PlayStation
- Steam Input

No necesitas configuración adicional.

---

## ⚙️ Requisitos del Sistema

### Mínimos
- CPU: 4 cores
- RAM: 8 GB
- GPU: Vulkan compatible
- Disco: 60 GB libres

### Recomendados
- CPU: 8 cores
- RAM: 16 GB
- GPU: NVIDIA GTX 1060 / AMD RX 580 o mejor
- Disco: SSD con 80 GB libres

---

## 🔄 Actualizaciones

Cuando Delta Force se actualice en Steam:

```bash
# Simplemente ejecuta de nuevo
./quick_start_deltaforce.sh
```

El script detectará los cambios y reconfigurará ACE si es necesario.

---

## 🤝 Contribuciones

Si encuentras problemas o tienes mejoras:

1. Reporta en GitHub Issues
2. Comparte logs: `~/.local/share/KernelBridge/logs/`
3. Incluye tu configuración de sistema

---

## 📝 Logs y Debug

Los logs se guardan en:

```
~/.local/share/KernelBridge/
├── logs/
│   ├── daemon.log
│   └── ace.log
└── deltaforce_prefix/
    └── ... (Wine prefix)
```

Para ver logs en tiempo real:

```bash
tail -f ~/.local/share/KernelBridge/logs/*.log
```

---

## 💝 Agradecimientos

- **Wine Team**: Por Wine/Proton
- **DXVK Team**: Por la capa DirectX→Vulkan
- **GloriousEggroll**: Por Proton-GE
- **Valve**: Por Steam Play y Proton
- **Comunidad Linux Gaming**: Por el soporte constante

---

## 🎖️ ¡Disfruta Delta Force!

Este es tu momento. Juega Delta Force en Linux como debe ser.

**¡Nos vemos en el campo de batalla, soldado! 🔥**

---

### Enlaces Útiles

- [Guía Completa](docs/delta_force_guia.md)
- [Documentación del Proyecto](docs/)
- [Wine HQ](https://www.winehq.org/)
- [ProtonDB](https://www.protondb.com/)

---

*Última actualización: 10 de noviembre de 2025*
*Compatible con: Fedora Linux 43, Delta Force (versión actual en Steam)*
