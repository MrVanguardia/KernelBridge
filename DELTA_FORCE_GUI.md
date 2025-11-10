# 🎮 Lanzar Delta Force desde la GUI

## ✅ ¡Nueva Función Agregada!

Ahora puedes lanzar Delta Force directamente desde la interfaz gráfica de KernelBridge.

---

## 🚀 Cómo Usar

### Método 1: Script de Inicio Rápido

```bash
cd ~/Documentos/PROYECTOS/kernelBridge
./start_gui_deltaforce.sh
```

### Método 2: GUI Directa

```bash
cd ~/Documentos/PROYECTOS/kernelBridge
./gui/target/release/kernelbridge-gui
```

---

## 📖 Pasos en la GUI

1. **Abre la GUI** con cualquiera de los métodos de arriba

2. **Ve a la sección "🎮 Juegos"** (en el menú lateral izquierdo)

3. **Busca el botón azul**: 
   ```
   🎯 Lanzar Delta Force (Quick Start)
   ```

4. **Click en el botón**

5. **¡El juego se lanzará automáticamente!**

---

## 🔍 Qué Hace el Botón

Cuando haces click en "🎯 Lanzar Delta Force (Quick Start)", el sistema:

1. ✅ Ejecuta `quick_start_deltaforce.sh` automáticamente
2. ✅ Detecta Steam (Flatpak o nativo)
3. ✅ Encuentra Delta Force instalado
4. ✅ Configura drivers ACE
5. ✅ Crea Wine prefix si es necesario
6. ✅ Configura registro de Windows
7. ✅ Lanza Delta Force

**Todo automático, sin comandos manuales.**

---

## 📊 Logs y Progreso

Los mensajes aparecerán en la sección **"🧠 KernelBridge"** de la GUI:

```
[DELTA FORCE] Lanzando quick start...
[DELTA FORCE] Ejecutando: /home/.../quick_start_deltaforce.sh
[DELTA FORCE] Script lanzado. Revisa la ventana del terminal...
```

**Nota**: El script se ejecuta en una ventana de terminal aparte, así que verás:
- Mensajes en la GUI (confirmación)
- Ventana de terminal (progreso detallado)

---

## 🎯 Alternativas

Si el botón de la GUI no funciona, siempre puedes usar:

### Opción A: Script directo
```bash
./quick_start_deltaforce.sh
```

### Opción B: Verificar primero
```bash
./verify_deltaforce.sh
./quick_start_deltaforce.sh
```

---

## 🐛 Solución de Problemas

### "No se encontró quick_start_deltaforce.sh"

**Solución:**
```bash
# Asegúrate de estar en el directorio correcto
cd ~/Documentos/PROYECTOS/kernelBridge
ls -la quick_start_deltaforce.sh

# Si no existe, está en la ubicación correcta
pwd
```

### El botón no aparece

**Solución:**
```bash
# Recompilar GUI
cd gui
cargo build --release

# Verificar que se compiló
ls -la target/release/kernelbridge-gui
```

### Nada pasa al hacer click

**Solución:**

1. Revisa la sección "🧠 KernelBridge" en la GUI para ver logs
2. Verifica que el script sea ejecutable:
   ```bash
   chmod +x quick_start_deltaforce.sh
   ```
3. Prueba ejecutarlo manualmente:
   ```bash
   ./quick_start_deltaforce.sh
   ```

---

## 📸 Captura de Pantalla

Así se ve el botón en la GUI:

```
╔══════════════════════════════════════════════════════╗
║ 🎮 Biblioteca de Juegos                             ║
║                                                      ║
║ ✅ 5 compatibles | 📊 10 total                       ║
║                                                      ║
║ [ 🔍 Escanear ]                                     ║
║                                                      ║
║ [ 🎯 Lanzar Delta Force (Quick Start) ] ← AQUÍ     ║
║                                                      ║
║ 🚀 Launchers detectados                             ║
║ ...                                                  ║
╚══════════════════════════════════════════════════════╝
```

El botón **🎯 Lanzar Delta Force** está destacado en **azul** (estilo "suggested-action").

---

## ✨ Ventajas de Usar la GUI

✅ **Visual**: No necesitas terminal
✅ **Logs integrados**: Ves mensajes en tiempo real
✅ **Todo en uno**: Escanear juegos + lanzar Delta Force
✅ **Fácil**: Un solo click

---

## 🎮 ¡A Jugar!

Ahora tienes 3 formas de lanzar Delta Force:

1. **GUI** (más visual): `./start_gui_deltaforce.sh`
2. **Script directo** (más rápido): `./quick_start_deltaforce.sh`
3. **Steam + Proton-GE** (más automático): Desde Steam

**¡Elige la que prefieras y disfruta!** 🔥

---

*Última actualización: 10 de noviembre de 2025*
