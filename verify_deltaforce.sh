#!/bin/bash
# Script de verificación del sistema Delta Force
# Comprueba que todo esté listo para jugar

echo "╔═══════════════════════════════════════════════════════════╗"
echo "║     KernelBridge - Verificación de Delta Force           ║"
echo "╚═══════════════════════════════════════════════════════════╝"
echo ""

ALL_OK=true

# Verificar dependencias
echo "📦 Verificando dependencias..."
DEPS=(wine winetricks cargo rustc)
for dep in "${DEPS[@]}"; do
    if command -v "$dep" &> /dev/null; then
        echo "  ✅ $dep"
    else
        echo "  ❌ $dep (falta)"
        ALL_OK=false
    fi
done
echo ""

# Verificar librerías opcionales pero recomendadas
echo "🔧 Verificando herramientas opcionales..."
OPT_DEPS=(gamemode gamemoderun mangohud)
for dep in "${OPT_DEPS[@]}"; do
    if command -v "$dep" &> /dev/null; then
        echo "  ✅ $dep"
    else
        echo "  ⚠️  $dep (opcional, mejora rendimiento)"
    fi
done
echo ""

# Verificar Steam
echo "🎮 Verificando Steam..."
STEAM_FOUND=false
for steam_dir in \
    "$HOME/.local/share/Steam" \
    "$HOME/.steam/steam" \
    "$HOME/.var/app/com.valvesoftware.Steam/.local/share/Steam"; do
    if [ -d "$steam_dir" ]; then
        echo "  ✅ Steam encontrado: $steam_dir"
        STEAM_FOUND=true
        STEAM_DIR="$steam_dir"
        break
    fi
done

if [ "$STEAM_FOUND" = false ]; then
    echo "  ❌ Steam no encontrado"
    ALL_OK=false
else
    # Verificar Delta Force
    echo ""
    echo "🎯 Verificando Delta Force..."
    DF_FOUND=false
    for library in "$STEAM_DIR/steamapps" "$STEAM_DIR"/steamapps/libraryfolders.vdf; do
        df_candidate="$STEAM_DIR/steamapps/common/Delta Force"
        if [ -d "$df_candidate" ]; then
            echo "  ✅ Delta Force encontrado: $df_candidate"
            DF_FOUND=true
            DF_DIR="$df_candidate"
            
            # Verificar ACE
            if [ -d "$DF_DIR/Win64/AntiCheatExpert" ]; then
                echo "  ✅ AntiCheatExpert detectado"
                
                # Verificar drivers
                ACE_DIR="$DF_DIR/Win64/AntiCheatExpert"
                DRIVERS=("ACE-BASE.sys" "ACE-BOOT.sys" "ACE-CORE.sys")
                DRIVERS_OK=true
                for driver in "${DRIVERS[@]}"; do
                    if [ -f "$ACE_DIR/$driver" ]; then
                        echo "    ✅ $driver"
                    else
                        echo "    ❌ $driver (falta)"
                        DRIVERS_OK=false
                    fi
                done
                
                if [ "$DRIVERS_OK" = false ]; then
                    ALL_OK=false
                fi
            else
                echo "  ❌ AntiCheatExpert no encontrado"
                ALL_OK=false
            fi
            break
        fi
    done
    
    if [ "$DF_FOUND" = false ]; then
        echo "  ⚠️  Delta Force no encontrado en Steam"
        echo "     Instálalo desde Steam antes de jugar"
    fi
fi
echo ""

# Verificar compilación de KernelBridge
echo "🛠️  Verificando compilación..."
if [ -f "daemon/target/release/kernelbridge-daemon" ]; then
    echo "  ✅ Daemon compilado"
else
    echo "  ⚠️  Daemon no compilado (se compilará automáticamente)"
fi

if [ -f "gui/target/release/kernelbridge-gui" ]; then
    echo "  ✅ GUI compilada"
else
    echo "  ⚠️  GUI no compilada (se compilará automáticamente)"
fi
echo ""

# Verificar scripts
echo "📜 Verificando scripts..."
SCRIPTS=("launch_deltaforce.sh" "quick_start_deltaforce.sh")
for script in "${SCRIPTS[@]}"; do
    if [ -x "$script" ]; then
        echo "  ✅ $script"
    elif [ -f "$script" ]; then
        echo "  ⚠️  $script (no ejecutable, arreglando...)"
        chmod +x "$script"
        echo "    ✅ Permisos corregidos"
    else
        echo "  ❌ $script (falta)"
        ALL_OK=false
    fi
done
echo ""

# Verificar documentación
echo "📚 Verificando documentación..."
DOCS=("DELTA_FORCE_README.md" "docs/delta_force_guia.md" "deltaforce.conf")
for doc in "${DOCS[@]}"; do
    if [ -f "$doc" ]; then
        echo "  ✅ $doc"
    else
        echo "  ❌ $doc (falta)"
        ALL_OK=false
    fi
done
echo ""

# Verificar configuración del sistema
echo "⚙️  Verificando configuración del sistema..."

# File limits
FILE_MAX=$(cat /proc/sys/fs/file-max)
if [ "$FILE_MAX" -ge 524288 ]; then
    echo "  ✅ Límite de archivos: $FILE_MAX"
else
    echo "  ⚠️  Límite de archivos bajo: $FILE_MAX (recomendado >= 524288)"
    echo "     Para aumentar: echo 'fs.file-max = 2097152' | sudo tee -a /etc/sysctl.conf"
fi

# Vulkan
if command -v vulkaninfo &> /dev/null; then
    GPU=$(vulkaninfo 2>/dev/null | grep "deviceName" | head -1 | cut -d'=' -f2 | xargs)
    if [ -n "$GPU" ]; then
        echo "  ✅ Vulkan disponible: $GPU"
    else
        echo "  ⚠️  Vulkan disponible pero GPU no detectada"
    fi
else
    echo "  ⚠️  vulkaninfo no encontrado (instala vulkan-tools)"
fi
echo ""

# Resumen final
echo "════════════════════════════════════════════════════════════"
if [ "$ALL_OK" = true ]; then
    echo "✅ ¡Todo listo para jugar Delta Force!"
    echo ""
    echo "Para lanzar el juego, ejecuta:"
    echo "  ./quick_start_deltaforce.sh"
    echo ""
    echo "O consulta la guía completa:"
    echo "  cat DELTA_FORCE_README.md"
else
    echo "⚠️  Algunas comprobaciones fallaron"
    echo ""
    echo "Revisa los errores arriba e instala las dependencias faltantes."
    echo ""
    echo "Para instalar dependencias en Fedora:"
    echo "  sudo dnf install wine winetricks dxvk vkd3d gamemode cargo rust"
fi
echo "════════════════════════════════════════════════════════════"
