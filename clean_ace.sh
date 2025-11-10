#!/bin/bash

# Script para limpiar ACE y probar con solo EasyAntiCheat

PFXDIR="$HOME/.var/app/com.valvesoftware.Steam/.local/share/Steam/steamapps/compatdata/2507950/pfx"
GAME_DIR="$HOME/.var/app/com.valvesoftware.Steam/.local/share/Steam/steamapps/common/Delta Force"

echo "🎮 Delta Force ACE Cleaner & EAC Enabler"
echo "========================================"
echo ""

# Verificar que el juego existe
if [ ! -d "$GAME_DIR" ]; then
    echo "❌ Delta Force no encontrado en: $GAME_DIR"
    exit 1
fi

echo "✅ Delta Force encontrado"
echo ""

# Verificar Wine Prefix
if [ ! -d "$PFXDIR" ]; then
    echo "❌ Wine Prefix no encontrado: $PFXDIR"
    exit 1
fi

echo "✅ Wine Prefix encontrado"
echo ""

# Backup de drivers ACE
echo "📦 Haciendo backup de drivers ACE..."
BACKUP_DIR="$HOME/.cache/kernelbridge/ace_backup_$(date +%Y%m%d_%H%M%S)"
mkdir -p "$BACKUP_DIR"

if [ -d "$PFXDIR/drive_c/windows/system32/drivers" ]; then
    find "$PFXDIR/drive_c/windows/system32/drivers" -name "ACE-*.sys" -exec cp {} "$BACKUP_DIR/" \; 2>/dev/null
    echo "✅ Backup guardado en: $BACKUP_DIR"
fi
echo ""

# Eliminar drivers ACE del Wine Prefix
echo "🧹 Eliminando drivers ACE del Wine Prefix..."
rm -f "$PFXDIR/drive_c/windows/system32/drivers/ACE-"*.sys 2>/dev/null
rm -f "$PFXDIR/drive_c/windows/syswow64/drivers/ACE-"*.sys 2>/dev/null
echo "✅ Drivers ACE eliminados"
echo ""

# Eliminar servicios ACE del registro
echo "🗑️  Eliminando servicios ACE del registro de Wine..."
export WINEPREFIX="$PFXDIR"
flatpak run --command=wine com.valvesoftware.Steam reg delete "HKLM\\System\\CurrentControlSet\\Services\\ACE-BASE" /f 2>/dev/null
flatpak run --command=wine com.valvesoftware.Steam reg delete "HKLM\\System\\CurrentControlSet\\Services\\ACE-CORE" /f 2>/dev/null
flatpak run --command=wine com.valvesoftware.Steam reg delete "HKLM\\System\\CurrentControlSet\\Services\\ACE-BOOT" /f 2>/dev/null
flatpak run --command=wine com.valvesoftware.Steam reg delete "HKLM\\System\\CurrentControlSet\\Services\\SGuard64" /f 2>/dev/null
echo "✅ Servicios ACE eliminados del registro"
echo ""

# Verificar EasyAntiCheat
echo "🔍 Verificando EasyAntiCheat..."
if find "$GAME_DIR" -iname "*easyanticheat*" -o -iname "*eac*" | grep -q .; then
    echo "✅ EasyAntiCheat encontrado en el juego"
    find "$GAME_DIR" -iname "*easyanticheat*" -o -iname "*eac*" | head -5
else
    echo "⚠️  EasyAntiCheat no detectado claramente"
fi
echo ""

# Crear archivo de configuración para Steam
LAUNCH_OPTIONS="PROTON_USE_EAC_WORKAROUND=1 PROTON_LOG=1 WINEDLLOVERRIDES=\"ACE-BASE=;ACE-CORE=;SGuard64=;TenProtect=\" DXVK_ASYNC=1 RADV_PERFTEST=aco,sam PROTON_NO_ESYNC=1 %command%"

echo "📋 Opciones de lanzamiento recomendadas para Steam:"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "$LAUNCH_OPTIONS"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Copiar al portapapeles si wl-copy está disponible
if command -v wl-copy &> /dev/null; then
    echo "$LAUNCH_OPTIONS" | wl-copy
    echo "✅ Opciones copiadas al portapapeles (Wayland)"
elif command -v xclip &> /dev/null; then
    echo "$LAUNCH_OPTIONS" | xclip -selection clipboard
    echo "✅ Opciones copiadas al portapapeles (X11)"
fi
echo ""

echo "📝 Instrucciones:"
echo "1. Abre Steam"
echo "2. Click derecho en Delta Force → Propiedades"
echo "3. En 'OPCIONES DE LANZAMIENTO', pega el comando de arriba"
echo "4. Asegúrate de usar GE-Proton (no Proton normal)"
echo "5. Lanza el juego"
echo ""

echo "🔧 Troubleshooting:"
echo "- Si no inicia: Prueba agregando -offline al final del comando"
echo "- Si crashea: Prueba con PROTON_USE_D3D11=1"
echo "- Si ban: Restaura drivers ACE desde $BACKUP_DIR"
echo ""

echo "✅ Limpieza completada!"
echo ""
echo "⚠️  ADVERTENCIA: Deshabilitar ACE puede resultar en ban."
echo "   Úsalo bajo tu propio riesgo para pruebas."
