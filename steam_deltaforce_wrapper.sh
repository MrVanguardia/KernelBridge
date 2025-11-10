#!/bin/bash

################################################################################
# Steam Delta Force Wrapper - Integración ACE con Steam
################################################################################
# Este script se ejecuta ANTES de que Steam lance Delta Force.
# Configura automáticamente el entorno ACE para que el anti-cheat funcione.
#
# USO:
# 1. En Steam, click derecho en Delta Force → Propiedades
# 2. En "Launch Options" pega:
#    /home/mrvanguardia/Documentos/PROYECTOS/kernelBridge/steam_deltaforce_wrapper.sh %command%
#
# El script configurará ACE y luego ejecutará el juego normalmente.
################################################################################

# NO usar set -e para evitar que falle por errores menores
# set -e

# Log file para diagnóstico
LOG_FILE="${HOME}/.cache/kernelbridge/steam_wrapper.log"
mkdir -p "$(dirname "${LOG_FILE}")"

# Función de logging
log() {
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] $*" | tee -a "${LOG_FILE}"
}

# Colores para mensajes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

log "═══════════════════════════════════════════════════════════════"
log "Steam Delta Force Wrapper iniciado"
log "Argumentos recibidos: $@"
log "═══════════════════════════════════════════════════════════════"

echo -e "${BLUE}╔═══════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║   Steam Delta Force - Configuración ACE Automática           ║${NC}"
echo -e "${BLUE}╚═══════════════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${YELLOW}Log completo: ${LOG_FILE}${NC}"
echo ""

# Directorios del proyecto
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WIN64_DIR="${SCRIPT_DIR}/Win64"
ACE_DIR="${WIN64_DIR}/AntiCheatExpert"

log "Script dir: ${SCRIPT_DIR}"
log "ACE dir: ${ACE_DIR}"

# Detectar Wine Prefix de Steam
# Steam Flatpak usa diferentes prefixes según la AppID
STEAM_FLATPAK_DATA="${HOME}/.var/app/com.valvesoftware.Steam/.local/share/Steam"
STEAM_NATIVE_DATA="${HOME}/.local/share/Steam"

# Intentar detectar el directorio de Steam
if [ -d "${STEAM_FLATPAK_DATA}" ]; then
    STEAM_DATA="${STEAM_FLATPAK_DATA}"
    echo -e "${GREEN}[✓] Steam Flatpak detectado${NC}"
    log "Steam Flatpak detectado: ${STEAM_DATA}"
elif [ -d "${STEAM_NATIVE_DATA}" ]; then
    STEAM_DATA="${STEAM_NATIVE_DATA}"
    echo -e "${GREEN}[✓] Steam nativo detectado${NC}"
    log "Steam nativo detectado: ${STEAM_DATA}"
else
    echo -e "${RED}[✗] No se encontró instalación de Steam${NC}"
    log "ERROR: Steam no encontrado"
    echo ""
    echo "Instalando Steam o verifica la ruta manualmente."
    log "Intentando ejecutar el juego de todas formas..."
    # No salir, intentar ejecutar el juego de todas formas
    exec "$@"
fi

# Buscar Delta Force en Steam
DELTA_FORCE_DIR=$(find "${STEAM_DATA}/steamapps/common" -maxdepth 1 -type d -iname "*delta*force*" 2>/dev/null | head -n1)

log "Buscando Delta Force en: ${STEAM_DATA}/steamapps/common"

if [ -z "${DELTA_FORCE_DIR}" ]; then
    echo -e "${RED}[✗] Delta Force no encontrado en Steam${NC}"
    log "ADVERTENCIA: Delta Force no encontrado, continuando de todas formas"
    echo "Asegúrate de que Delta Force esté instalado."
    # No salir, continuar
else
    echo -e "${GREEN}[✓] Delta Force encontrado: ${DELTA_FORCE_DIR}${NC}"
    log "Delta Force encontrado: ${DELTA_FORCE_DIR}"
fi

# Detectar Wine Prefix
# Steam usa compatdata/<AppID>/pfx para cada juego
COMPAT_DATA=$(find "${STEAM_DATA}/steamapps/compatdata" -maxdepth 2 -type d -name "pfx" 2>/dev/null | head -n1)

log "Buscando Wine Prefix en: ${STEAM_DATA}/steamapps/compatdata"

if [ -z "${COMPAT_DATA}" ]; then
    echo -e "${YELLOW}[!] No se encontró Wine Prefix existente${NC}"
    log "ADVERTENCIA: Wine Prefix no encontrado"
    # Steam creará el prefix automáticamente, no necesitamos crearlo aquí
    echo -e "${YELLOW}[!] Steam creará el Wine Prefix automáticamente${NC}"
    WINE_PREFIX=""
else
    WINE_PREFIX="${COMPAT_DATA}"
    echo -e "${GREEN}[✓] Wine Prefix: ${WINE_PREFIX}${NC}"
    log "Wine Prefix encontrado: ${WINE_PREFIX}"
fi

# Verificar drivers ACE
if [ ! -d "${ACE_DIR}" ]; then
    echo -e "${RED}[✗] Directorio ACE no encontrado: ${ACE_DIR}${NC}"
    log "ERROR: Directorio ACE no encontrado: ${ACE_DIR}"
    echo "Verifica que los drivers ACE estén en Win64/AntiCheatExpert/"
    echo -e "${YELLOW}[!] Continuando sin ACE...${NC}"
    log "Continuando sin ACE, ejecutando juego directamente"
    exec "$@"
fi

ACE_DRIVERS=$(find "${ACE_DIR}" -type f -name "*.sys" 2>/dev/null | wc -l)
if [ ${ACE_DRIVERS} -eq 0 ]; then
    echo -e "${RED}[✗] No se encontraron drivers ACE (.sys)${NC}"
    log "ERROR: No se encontraron drivers ACE"
    echo -e "${YELLOW}[!] Continuando sin ACE...${NC}"
    exec "$@"
fi

echo -e "${GREEN}[✓] Encontrados ${ACE_DRIVERS} drivers ACE${NC}"
log "Drivers ACE encontrados: ${ACE_DRIVERS}"

# Configurar Wine Prefix con ACE
echo ""
echo -e "${BLUE}[→] Configurando Wine Prefix con ACE...${NC}"
log "Configurando Wine Prefix con ACE"

# Solo configurar si tenemos Wine Prefix
if [ -n "${WINE_PREFIX}" ] && [ -d "${WINE_PREFIX}" ]; then
    # Crear directorios necesarios en Wine Prefix
    WINE_SYSTEM32="${WINE_PREFIX}/drive_c/windows/system32"
    WINE_DRIVERS="${WINE_PREFIX}/drive_c/windows/system32/drivers"

    mkdir -p "${WINE_SYSTEM32}" 2>/dev/null || true
    mkdir -p "${WINE_DRIVERS}" 2>/dev/null || true

    # Copiar drivers ACE al Wine Prefix
    echo -e "${BLUE}[→] Copiando drivers ACE...${NC}"
    log "Copiando drivers ACE a ${WINE_DRIVERS}"
    cp -v "${ACE_DIR}"/*.sys "${WINE_DRIVERS}/" 2>/dev/null || true

    # Crear entradas de registro para ACE
    WINE_REGISTRY="${WINE_PREFIX}/system.reg"

    # Función para agregar claves de registro
    add_registry_keys() {
        local REG_FILE="${WINE_PREFIX}/user.reg"
        
        # Asegurar que el archivo existe
        touch "${REG_FILE}" 2>/dev/null || return 0
        
        # Agregar claves para ACE (si no existen)
        if ! grep -q "AntiCheatExpert" "${REG_FILE}" 2>/dev/null; then
            cat >> "${REG_FILE}" << 'EOF' || true

[Software\\Tencent\\AntiCheatExpert]
"InstallPath"="C:\\windows\\system32\\drivers"
"Version"="1.0.0"
"Enabled"=dword:00000001

[System\\CurrentControlSet\\Services\\ACE-BASE]
"Type"=dword:00000001
"Start"=dword:00000000
"ErrorControl"=dword:00000001
"ImagePath"="system32\\drivers\\ACE-BASE.sys"
"DisplayName"="ACE Anti-Cheat Base Driver"

[System\\CurrentControlSet\\Services\\ACE-BOOT]
"Type"=dword:00000001
"Start"=dword:00000000
"ErrorControl"=dword:00000001
"ImagePath"="system32\\drivers\\ACE-BOOT.sys"
"DisplayName"="ACE Anti-Cheat Boot Driver"

[System\\CurrentControlSet\\Services\\ACE-CORE]
"Type"=dword:00000001
"Start"=dword:00000002
"ErrorControl"=dword:00000001
"ImagePath"="system32\\drivers\\ACE-CORE.sys"
"DisplayName"="ACE Anti-Cheat Core Driver"

EOF
            echo -e "${GREEN}[✓] Claves de registro ACE agregadas${NC}"
            log "Claves de registro ACE agregadas"
        else
            echo -e "${YELLOW}[!] Claves de registro ACE ya existen${NC}"
            log "Claves de registro ACE ya existen"
        fi
    }

    add_registry_keys
    
    # Configurar WINEPREFIX
    export WINEPREFIX="${WINE_PREFIX}"
    log "WINEPREFIX configurado: ${WINE_PREFIX}"
else
    echo -e "${YELLOW}[!] Wine Prefix no disponible aún${NC}"
    echo -e "${YELLOW}[!] Steam lo configurará en el primer inicio${NC}"
    log "Wine Prefix no disponible, Steam lo creará"
fi

# Variables de entorno para mejorar compatibilidad
# Solo configurar WINEPREFIX si está disponible
if [ -n "${WINE_PREFIX}" ]; then
    export WINEPREFIX="${WINE_PREFIX}"
fi

export DXVK_HUD=0
export WINE_LARGE_ADDRESS_AWARE=1
export STAGING_SHARED_MEMORY=1

# ════════════════════════════════════════════════════════════
# OPTIMIZACIONES AMD GPU (RADV + Mesa)
# ════════════════════════════════════════════════════════════

# Mesa multithreading
export mesa_glthread=true
export MESA_LOADER_DRIVER_OVERRIDE=radv

# RADV optimizations (AMD open-source driver)
export RADV_PERFTEST=aco,sam,rt,nggc
export RADV_DEBUG=novrsflatshading

# VKD3D optimizations
export VKD3D_CONFIG=dxr11,dxr
export VKD3D_SHADER_CACHE_PATH="${HOME}/.cache/vkd3d"
export VKD3D_DEBUG=none

# DXVK optimizations
export DXVK_ASYNC=1
export DXVK_STATE_CACHE_PATH="${HOME}/.cache/dxvk"
export DXVK_LOG_LEVEL=none

# Mesa shader cache
export MESA_SHADER_CACHE_DIR="${HOME}/.cache/mesa_shader_cache"
export MESA_DISK_CACHE_SINGLE_FILE=true

# AMD GPU performance
export AMD_VULKAN_ICD=RADV
export AMD_DEBUG=nohyperz,nofmask

# Disable vsync para mejor rendimiento
export __GL_SYNC_TO_VBLANK=0
export vblank_mode=0

# Proton-GE específico
export PROTON_USE_WINED3D=0
export PROTON_NO_ESYNC=0
export PROTON_NO_FSYNC=0
export PROTON_FORCE_LARGE_ADDRESS_AWARE=1
export PROTON_ENABLE_NVAPI=0
export PROTON_HIDE_NVIDIA_GPU=1
export PROTON_USE_DXVK=1
export PROTON_USE_VKD3D=1

log "Variables de entorno AMD configuradas"
log "RADV: aco,sam,rt,nggc | DXVK_ASYNC: 1 | Mesa glthread: enabled"

echo ""
echo -e "${GREEN}╔═══════════════════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║   Configuración ACE completada - Iniciando Delta Force       ║${NC}"
echo -e "${GREEN}╚═══════════════════════════════════════════════════════════════╝${NC}"
echo ""
if [ -n "${WINE_PREFIX}" ]; then
    echo -e "${BLUE}Wine Prefix:${NC} ${WINE_PREFIX}"
fi
echo -e "${BLUE}ACE Drivers:${NC} ${ACE_DRIVERS} archivos copiados"
echo -e "${BLUE}Comando Steam:${NC} $@"
echo ""
echo -e "${YELLOW}Logs guardados en: ${LOG_FILE}${NC}"
echo ""

log "Ejecutando comando: $@"
log "═══════════════════════════════════════════════════════════════"

# Ejecutar el comando original de Steam (el juego)
# Steam pasa el comando completo como argumentos
exec "$@"
