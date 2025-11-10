# 🐛 Modo Debug - Ver todos los logs

## 🎯 ¿Para qué sirve?

El **modo debug** te permite ver **TODOS los logs** de la GUI y Delta Force directamente en la terminal donde ejecutas el programa.

**Útil para:**
- ✅ Ver errores en tiempo real
- ✅ Diagnosticar por qué Delta Force no inicia
- ✅ Seguir el progreso de la configuración ACE
- ✅ Ver logs de Steam/Proton/Wine
- ✅ Debugging general

---

## 🚀 Cómo usar

### Opción 1: Script de debug (Recomendado)

```bash
cd ~/Documentos/PROYECTOS/kernelBridge
./kb_debug.sh
```

**Esto muestra:**
- ✅ Información del sistema (GPU, kernel, Mesa)
- ✅ Verificación de componentes
- ✅ Detección de Steam y Delta Force
- ✅ Logs en tiempo real con colores
- ✅ Timestamps en cada mensaje

### Opción 2: GUI normal con logs

```bash
cd ~/Documentos/PROYECTOS/kernelBridge
./start_gui_deltaforce.sh
```

### Opción 3: Con alias (después de instalar)

```bash
# Instalar aliases primero
./install_aliases.sh
source ~/.bashrc

# Luego simplemente:
kb-debug
```

---

## 📊 Tipos de mensajes

El modo debug colorea los mensajes automáticamente:

| Color | Tipo | Ejemplo |
|-------|------|---------|
| 🔴 **Rojo** | Errores | `ERROR: No se encontró archivo` |
| 🟡 **Amarillo** | Advertencias | `WARNING: Wine prefix no existe` |
| 🟢 **Verde** | Éxito | `✅ Script completado exitosamente` |
| 🔵 **Azul** | Info general | `[INFO] Iniciando compilación` |
| 🟣 **Magenta** | Steam/Proton | `Steam Flatpak detectado` |
| 🔷 **Cyan** | Delta Force | `[DELTA FORCE] Configurando ACE` |

---

## 🔍 Ejemplo de salida

```
╔═══════════════════════════════════════════════════════════════════════╗
║                    KernelBridge - Modo Debug                          ║
╚═══════════════════════════════════════════════════════════════════════╝

[13:45:23] ╔════════════════════════════════════════════════════════════════╗
[13:45:23] ║           DELTA FORCE - LANZAMIENTO INICIADO                  ║
[13:45:23] ╚════════════════════════════════════════════════════════════════╝

[13:45:23] [DELTA FORCE] 🔍 Buscando en: /home/user/Documentos/PROYECTOS/kernelBridge/quick_start_deltaforce.sh
[13:45:23] [DELTA FORCE] ✅ Script encontrado: quick_start_deltaforce.sh
[13:45:23] [DELTA FORCE] 📂 Directorio: /home/user/Documentos/PROYECTOS/kernelBridge
[13:45:23] [DELTA FORCE] 🚀 Ejecutando script...

════════════════════════════════════════════════════════════════
[13:45:24] [✓] Steam Flatpak detectado
[13:45:24] [✓] Delta Force encontrado: /home/user/.var/app/com.valvesoftware.Steam/.local/share/Steam/steamapps/common/Delta Force
[13:45:24] [→] Configurando Wine Prefix con ACE...
[13:45:25] [→] Copiando drivers ACE...
[13:45:25] ACE-BASE.sys -> /home/user/.var/app/com.valvesoftware.Steam/.local/share/Steam/steamapps/compatdata/0/pfx/drive_c/windows/system32/drivers/ACE-BASE.sys
[13:45:25] [✓] Claves de registro ACE agregadas
[13:45:26] [✓] Configuración ACE completada

[13:45:27] Iniciando Delta Force...
════════════════════════════════════════════════════════════════
[13:45:28] [DELTA FORCE] ✅ Script completado exitosamente
```

---

## 🛠️ Debugging Común

### ❌ Error: "No se encontró quick_start_deltaforce.sh"

**Causa:** Ejecutando desde directorio incorrecto

**Solución:**
```bash
cd ~/Documentos/PROYECTOS/kernelBridge
./kb_debug.sh
```

### ❌ Error: "Steam no detectado"

**Verás en los logs:**
```
⚠️  Steam no detectado
```

**Solución:**
- Verifica que Steam está instalado: `flatpak list | grep -i steam`
- Si es Steam nativo, verifica: `ls ~/.local/share/Steam`

### ❌ Error: "Delta Force no encontrado"

**Verás en los logs:**
```
⚠️  Delta Force no detectado (¿no instalado?)
```

**Solución:**
- Instala Delta Force desde Steam primero
- Verifica: `find ~/.var/app/com.valvesoftware.Steam -name "*Delta*Force*"`

### ❌ Error al compilar GUI

**Verás en los logs:**
```
ERROR: could not compile `kernelbridge-gui`
```

**Solución:**
```bash
cd ~/Documentos/PROYECTOS/kernelBridge/gui
cargo clean
cargo build --release
```

---

## 📝 Ver logs anteriores

### Logs de la GUI
Los logs se guardan automáticamente en:
```bash
cat ~/.cache/kernelbridge/steam_wrapper.log
```

O con el alias:
```bash
deltaforce-logs
```

### Logs de Steam
```bash
cat ~/.var/app/com.valvesoftware.Steam/.local/share/Steam/logs/console_log.txt
```

### Logs de Proton/Wine
```bash
# Último log de Proton
ls -lt ~/.var/app/com.valvesoftware.Steam/.local/share/Steam/steamapps/compatdata/*/pfx/drive_c/users/steamuser/Temp/*.log | head -1
```

---

## 🎯 Comandos útiles con alias

Después de `./install_aliases.sh`:

| Comando | Función |
|---------|---------|
| `kb-debug` | Lanzar GUI con debug completo |
| `deltaforce` | Inicio rápido sin GUI |
| `deltaforce-gui` | GUI con logs simples |
| `deltaforce-logs` | Ver logs del wrapper |
| `deltaforce-clean` | Limpiar cache de shaders |
| `deltaforce-verify` | Verificar sistema |

---

## 🔧 Variables de entorno de debug

El script configura automáticamente:

```bash
RUST_BACKTRACE=full     # Stack traces completos
RUST_LOG=debug          # Logging detallado de Rust
```

Si quieres aún MÁS detalles, ejecuta manualmente:

```bash
RUST_LOG=trace RUST_BACKTRACE=full ./gui/target/release/kernelbridge-gui
```

---

## 📊 Guardar logs en archivo

Para guardar todos los logs en un archivo:

```bash
./kb_debug.sh 2>&1 | tee ~/deltaforce_debug.log
```

Luego puedes revisar:
```bash
cat ~/deltaforce_debug.log
```

O compartir el archivo para soporte.

---

## 🎓 Interpretar logs

### Logs normales (OK)

```
[13:45:23] [DELTA FORCE] ✅ Script encontrado
[13:45:24] [✓] Steam Flatpak detectado
[13:45:25] [✓] Configuración ACE completada
```

### Logs de advertencia (Revisar)

```
[13:45:23] ⚠️  GE-Proton no detectado
[13:45:24] [!] Wine Prefix no disponible aún
```

**Acción:** Usualmente no es crítico, el sistema se autocorrige.

### Logs de error (Acción requerida)

```
[13:45:23] ❌ ERROR: No se encontró archivo
[13:45:24] ERROR: Steam no detectado
```

**Acción:** Lee el error y sigue las soluciones de arriba.

---

## ✅ Resumen

**Para debugging completo:**
```bash
cd ~/Documentos/PROYECTOS/kernelBridge
./kb_debug.sh
```

**Características:**
- ✅ Logs en tiempo real con colores
- ✅ Timestamps en cada mensaje
- ✅ Información del sistema
- ✅ Verificación automática de componentes
- ✅ Fácil de interpretar

**Mantén la terminal abierta mientras usas Delta Force para ver todo lo que sucede!** 🐛🔍
