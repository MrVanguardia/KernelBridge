# 🎮 DELTA FORCE EN LINUX - GUÍA RÁPIDA

## ¡3 Pasos Para Jugar!

### Paso 1: Verificar el Sistema
```bash
cd ~/Documentos/PROYECTOS/kernelBridge
./verify_deltaforce.sh
```

Si todo está ✅, continúa al Paso 2.

Si hay ❌:
```bash
# Instalar dependencias que falten
sudo dnf install wine winetricks dxvk vkd3d gamemode cargo rust
```

### Paso 2: Preparar Delta Force

Asegúrate de tener Delta Force instalado en Steam. Si no:
```bash
# Abrir Steam
steam

# O si usas Flatpak:
flatpak run com.valvesoftware.Steam
```

Descarga **Delta Force** desde Steam.

### Paso 3: ¡JUGAR!

```bash
./quick_start_deltaforce.sh
```

**¡Eso es todo!**

---

## 📋 Checklist Rápido

- ✅ Steam instalado
- ✅ Delta Force descargado en Steam
- ✅ Wine instalado (`wine --version`)
- ✅ Drivers de GPU actualizados
- ✅ Al menos 8GB RAM y 60GB espacio libre

---

## 🎯 Opciones de Lanzamiento

### A) Script Automático (Recomendado)
```bash
./quick_start_deltaforce.sh
```
Compila y lanza todo automáticamente.

### B) Con GameMode (Mejor Rendimiento)
```bash
gamemoderun ./launch_deltaforce.sh
```

### C) Con MangoHud (Ver FPS)
```bash
mangohud ./launch_deltaforce.sh
```

### D) Con Ambos
```bash
gamemoderun mangohud ./launch_deltaforce.sh
```

### E) Desde la GUI
```bash
./gui/target/release/kernelbridge-gui
```
Ve a **Juegos → Escanear → Delta Force → Ejecutar**

---

## ⚡ Optimización Rápida

### Mejor Rendimiento
```bash
# Antes de jugar:
echo performance | sudo tee /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor

# Después de jugar:
echo powersave | sudo tee /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor
```

### Variables de Entorno Útiles
```bash
# Añadir a tu ~/.bashrc o ejecutar antes de jugar:
export WINEESYNC=1
export WINEFSYNC=1  
export STAGING_SHARED_MEMORY=1
export DXVK_ASYNC=1
```

---

## 🐛 Soluciones Rápidas

### Problema: "No se encuentra Delta Force"
**Solución:**
```bash
# Verificar instalación
ls ~/.local/share/Steam/steamapps/common/ | grep -i delta
# O para Steam Flatpak:
ls ~/.var/app/com.valvesoftware.Steam/.local/share/Steam/steamapps/common/ | grep -i delta
```

### Problema: "Rendimiento bajo / FPS bajo"
**Soluciones:**
1. Cerrar aplicaciones en segundo plano
2. Usar GameMode: `gamemoderun ./launch_deltaforce.sh`
3. Bajar calidad gráfica en opciones del juego
4. Verificar GPU: `vulkaninfo | grep deviceName`

### Problema: "Crash al iniciar"
**Soluciones:**
```bash
# 1. Limpiar cache
rm -rf ~/.cache/mesa_shader_cache
rm -rf ~/.local/share/KernelBridge/deltaforce_prefix

# 2. Reintentar
./launch_deltaforce.sh

# 3. Con logs completos
WINEDEBUG=+all ./launch_deltaforce.sh 2>&1 | tee delta_debug.log
```

### Problema: "No se conecta online"
**Solución:**
- Verifica tu conexión a internet
- Desactiva VPN si tienes
- Asegúrate de que Steam esté online

---

## 📊 Ver Logs

```bash
# Logs del sistema
journalctl -f

# Logs de KernelBridge
tail -f ~/.local/share/KernelBridge/logs/*.log

# Logs de Wine (durante ejecución)
WINEDEBUG=+all ./launch_deltaforce.sh
```

---

## 🎮 Controles

Delta Force detecta automáticamente:
- ✅ Mouse y teclado (plug & play)
- ✅ Controles Xbox
- ✅ Controles PlayStation (via Steam Input)

No necesitas configurar nada.

---

## 💡 Tips Pro

1. **Primera vez**: El juego tardará en cargar mientras compila shaders (es normal)
2. **Actualizaciones**: Después de actualizar Delta Force, simplemente ejecuta el script de nuevo
3. **Multi-monitor**: Usa modo ventana sin bordes para mejor experiencia
4. **Audio**: Si no hay audio, verifica que PulseAudio/PipeWire estén corriendo

---

## 📚 Más Información

- **Guía Completa**: `cat DELTA_FORCE_README.md`
- **Documentación Técnica**: `cat docs/delta_force_guia.md`
- **Configuración Avanzada**: Edita `deltaforce.conf`

---

## 🆘 Soporte

Si tienes problemas:

1. Ejecuta verificación: `./verify_deltaforce.sh`
2. Revisa logs: `~/.local/share/KernelBridge/logs/`
3. Busca en los docs: `docs/`
4. Reporta issue con logs completos

---

## ✨ ¡A Jugar!

```
╔═══════════════════════════════════════════════════════════╗
║                                                           ║
║     🎮 ¡Disfruta Delta Force en Linux! 🎮                ║
║                                                           ║
║     Tu sueño se hizo realidad.                           ║
║     Nos vemos en el campo de batalla, soldado! 🔥        ║
║                                                           ║
╚═══════════════════════════════════════════════════════════╝
```

**¡GLHF! (Good Luck, Have Fun!)** 🎖️
