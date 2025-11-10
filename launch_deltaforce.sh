#!/bin/bash
# Script de lanzamiento para Delta Force en Linux con AntiCheatExpert (ACE)
# Compatible con Fedora Linux 43

set -e

# Colores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}╔═══════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║  KernelBridge - Delta Force Launcher (ACE Compatible)   ║${NC}"
echo -e "${BLUE}╚═══════════════════════════════════════════════════════════╝${NC}"
echo ""

# Detectar Steam y Delta Force
STEAM_DIR=""
DELTA_FORCE_DIR=""

# Buscar Steam
for steam_candidate in \
    "$HOME/.local/share/Steam" \
    "$HOME/.steam/steam" \
    "$HOME/.var/app/com.valvesoftware.Steam/.local/share/Steam"; do
    if [ -d "$steam_candidate" ]; then
        STEAM_DIR="$steam_candidate"
        echo -e "${GREEN}✓${NC} Steam encontrado: $STEAM_DIR"
        break
    fi
done

if [ -z "$STEAM_DIR" ]; then
    echo -e "${RED}✗${NC} No se encontró Steam. Instala Steam primero."
    exit 1
fi

# Buscar Delta Force en Steam
for library in "$STEAM_DIR/steamapps" "$STEAM_DIR"/steamapps/common/*/steamapps; do
    df_candidate="$library/common/Delta Force"
    if [ -d "$df_candidate" ]; then
        DELTA_FORCE_DIR="$df_candidate"
        echo -e "${GREEN}✓${NC} Delta Force encontrado: $DELTA_FORCE_DIR"
        break
    fi
done

if [ -z "$DELTA_FORCE_DIR" ]; then
    echo -e "${RED}✗${NC} No se encontró Delta Force en Steam."
    echo -e "${YELLOW}Asegúrate de tener Delta Force instalado desde Steam.${NC}"
    exit 1
fi

# Verificar AntiCheatExpert
# Primero buscar en el juego instalado
ACE_DIR="$DELTA_FORCE_DIR/Win64/AntiCheatExpert"
if [ ! -d "$ACE_DIR" ]; then
    # Si no está en el juego, usar los del proyecto KernelBridge
    SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
    ACE_DIR="$SCRIPT_DIR/Win64/AntiCheatExpert"
    if [ ! -d "$ACE_DIR" ]; then
        echo -e "${RED}✗${NC} No se encontró AntiCheatExpert"
        echo -e "${YELLOW}Buscando en ubicaciones alternativas...${NC}"
        # Buscar en Game/DeltaForce
        if [ -d "$DELTA_FORCE_DIR/Game/DeltaForce/Win64/AntiCheatExpert" ]; then
            ACE_DIR="$DELTA_FORCE_DIR/Game/DeltaForce/Win64/AntiCheatExpert"
        elif [ -d "$DELTA_FORCE_DIR/Game/Win64/AntiCheatExpert" ]; then
            ACE_DIR="$DELTA_FORCE_DIR/Game/Win64/AntiCheatExpert"
        else
            echo -e "${RED}No se pudo encontrar AntiCheatExpert en ninguna ubicación${NC}"
            exit 1
        fi
    fi
fi

echo -e "${GREEN}✓${NC} AntiCheatExpert detectado: $ACE_DIR"

# Verificar drivers ACE
ACE_DRIVERS=(
    "ACE-BASE.sys"
    "ACE-BOOT.sys"
    "ACE-CORE.sys"
)

DRIVERS_OK=true
for driver in "${ACE_DRIVERS[@]}"; do
    if [ -f "$ACE_DIR/$driver" ]; then
        echo -e "${GREEN}  ✓${NC} $driver"
    else
        echo -e "${RED}  ✗${NC} $driver (falta)"
        DRIVERS_OK=false
    fi
done

if [ "$DRIVERS_OK" = false ]; then
    echo -e "${YELLOW}Advertencia: Algunos drivers ACE faltan, el juego puede no iniciar correctamente.${NC}"
fi

# Configurar Wine prefix para Delta Force
WINE_PREFIX="$HOME/.local/share/KernelBridge/deltaforce_prefix"
mkdir -p "$WINE_PREFIX"

echo ""
echo -e "${BLUE}Configurando entorno Wine...${NC}"

# Variables de entorno críticas para ACE
export WINEPREFIX="$WINE_PREFIX"
export WINEARCH="win64"
export WINE_CPU_TOPOLOGY="8:8"
export STAGING_SHARED_MEMORY=1
export ACE_DRIVER_MODE=1
export ACE_DISABLE_STRICT_CHECK=1

# Deshabilitar comprobaciones de virtualización
export DXVK_NVAPI_DRIVER_VERSION=53141
export DXVK_NVAPIHACK=0

# Variables para mejor compatibilidad
export WINE_DISABLE_WRITE_WATCH=1
export WINEESYNC=1
export WINEFSYNC=1

# DXVK y VKD3D
export DXVK_HUD=0
export DXVK_LOG_LEVEL=none
export VKD3D_DEBUG=none

# Inicializar Wine prefix si es necesario
if [ ! -d "$WINE_PREFIX/drive_c" ]; then
    echo -e "${YELLOW}Inicializando Wine prefix (primera vez)...${NC}"
    wineboot --init
    echo -e "${GREEN}✓${NC} Wine prefix inicializado"
fi

# Configurar drivers ACE en el prefix
SYSTEM32="$WINE_PREFIX/drive_c/windows/system32/drivers"
mkdir -p "$SYSTEM32"

echo -e "${BLUE}Copiando drivers ACE al prefix de Wine...${NC}"
for driver in "$ACE_DIR"/*.sys; do
    if [ -f "$driver" ]; then
        driver_name=$(basename "$driver")
        cp "$driver" "$SYSTEM32/$driver_name"
        echo -e "${GREEN}  ✓${NC} $driver_name"
    fi
done

# Copiar DLLs adicionales de ACE
for dll in "$ACE_DIR"/*.dll; do
    if [ -f "$dll" ]; then
        dll_name=$(basename "$dll")
        cp "$dll" "$WINE_PREFIX/drive_c/windows/system32/$dll_name"
        echo -e "${GREEN}  ✓${NC} $dll_name"
    fi
done

# Crear claves de registro para ACE
echo -e "${BLUE}Configurando registro de Windows...${NC}"
cat > /tmp/ace_registry.reg << 'EOF'
Windows Registry Editor Version 5.00

[HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\ACE-BASE]
"Type"=dword:00000001
"Start"=dword:00000000
"ErrorControl"=dword:00000001
"ImagePath"="\\SystemRoot\\System32\\drivers\\ACE-BASE.sys"
"DisplayName"="ACE Anti-Cheat Base Driver"

[HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\ACE-BOOT]
"Type"=dword:00000001
"Start"=dword:00000000
"ErrorControl"=dword:00000001
"ImagePath"="\\SystemRoot\\System32\\drivers\\ACE-BOOT.sys"
"DisplayName"="ACE Anti-Cheat Boot Driver"

[HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\ACE-CORE]
"Type"=dword:00000001
"Start"=dword:00000000
"ErrorControl"=dword:00000001
"ImagePath"="\\SystemRoot\\System32\\drivers\\ACE-CORE.sys"
"DisplayName"="ACE Anti-Cheat Core Driver"

[HKEY_LOCAL_MACHINE\SOFTWARE\Tencent\ACE]
"InstallPath"="C:\\Program Files\\AntiCheatExpert"
"Version"="1.0.0"
"Enabled"=dword:00000001
EOF

wine regedit /tmp/ace_registry.reg 2>/dev/null
echo -e "${GREEN}✓${NC} Registro configurado"

# Iniciar daemon de KernelBridge si no está corriendo
if [ ! -S /tmp/kernelbridge.sock ]; then
    echo -e "${YELLOW}Iniciando daemon de KernelBridge...${NC}"
    if [ -x "$(pwd)/daemon/target/release/kernelbridge-daemon" ]; then
        "$(pwd)/daemon/target/release/kernelbridge-daemon" &
        sleep 2
        echo -e "${GREEN}✓${NC} Daemon iniciado"
    elif [ -x "/usr/bin/kernelbridge-daemon" ]; then
        /usr/bin/kernelbridge-daemon &
        sleep 2
        echo -e "${GREEN}✓${NC} Daemon iniciado"
    else
        echo -e "${YELLOW}⚠${NC} No se encontró el daemon, continuando sin él..."
    fi
fi

# Preparar ACE con el daemon
if [ -S /tmp/kernelbridge.sock ]; then
    echo -e "${BLUE}Configurando ACE con KernelBridge...${NC}"
    echo "SETUP_ACE:$DELTA_FORCE_DIR|$WINE_PREFIX" | nc -U /tmp/kernelbridge.sock || true
fi

echo ""
echo -e "${GREEN}═══════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}  Configuración completada. Lanzando Delta Force...${NC}"
echo -e "${GREEN}═══════════════════════════════════════════════════════════${NC}"
echo ""

# Buscar ejecutable de Delta Force
DELTA_EXE=""
for exe_candidate in \
    "$DELTA_FORCE_DIR/DeltaForce.exe" \
    "$DELTA_FORCE_DIR/Game/DeltaForce.exe" \
    "$DELTA_FORCE_DIR/Binaries/Win64/DeltaForce.exe"; do
    if [ -f "$exe_candidate" ]; then
        DELTA_EXE="$exe_candidate"
        break
    fi
done

if [ -z "$DELTA_EXE" ]; then
    echo -e "${RED}✗${NC} No se encontró el ejecutable de Delta Force"
    exit 1
fi

echo -e "${BLUE}Ejecutable:${NC} $DELTA_EXE"
echo ""

# Cambiar al directorio del juego
cd "$(dirname "$DELTA_EXE")"

# Lanzar con Wine
echo -e "${YELLOW}Iniciando Delta Force...${NC}"
echo -e "${YELLOW}(Esto puede tomar unos momentos)${NC}"
echo ""

wine "$DELTA_EXE" "$@"

echo ""
echo -e "${GREEN}Delta Force se ha cerrado.${NC}"
