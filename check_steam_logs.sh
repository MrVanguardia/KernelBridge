#!/bin/bash

################################################################################
# Verificador de Logs de Steam + Wrapper
################################################################################
# Este script muestra los logs para diagnosticar por qué Delta Force no inicia
################################################################################

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

clear
echo -e "${CYAN}╔════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║  Diagnóstico de Steam + Delta Force + ACE                     ║${NC}"
echo -e "${CYAN}╚════════════════════════════════════════════════════════════════╝${NC}"
echo ""

# Verificar wrapper
WRAPPER="${HOME}/Documentos/PROYECTOS/kernelBridge/steam_deltaforce_wrapper.sh"

if [ ! -f "${WRAPPER}" ]; then
    echo -e "${RED}[✗] Wrapper no encontrado: ${WRAPPER}${NC}"
    exit 1
fi

echo -e "${GREEN}[✓] Wrapper encontrado${NC}"

# Verificar permisos
if [ -x "${WRAPPER}" ]; then
    echo -e "${GREEN}[✓] Wrapper es ejecutable${NC}"
else
    echo -e "${RED}[✗] Wrapper NO es ejecutable${NC}"
    echo "Ejecuta: chmod +x ${WRAPPER}"
fi

echo ""

# Mostrar log del wrapper si existe
WRAPPER_LOG="${HOME}/.cache/kernelbridge/steam_wrapper.log"

if [ -f "${WRAPPER_LOG}" ]; then
    echo -e "${BLUE}════════════════════════════════════════════════════════════════${NC}"
    echo -e "${BLUE}Log del Wrapper (últimas 50 líneas):${NC}"
    echo -e "${BLUE}════════════════════════════════════════════════════════════════${NC}"
    tail -50 "${WRAPPER_LOG}"
    echo ""
else
    echo -e "${YELLOW}[!] No hay log del wrapper aún${NC}"
    echo "El wrapper no se ha ejecutado o hubo un error antes de crear el log."
    echo ""
fi

# Buscar Steam
STEAM_FLATPAK="${HOME}/.var/app/com.valvesoftware.Steam/.local/share/Steam"
STEAM_NATIVE="${HOME}/.local/share/Steam"

if [ -d "${STEAM_FLATPAK}" ]; then
    STEAM_DIR="${STEAM_FLATPAK}"
    echo -e "${GREEN}[✓] Steam Flatpak: ${STEAM_DIR}${NC}"
elif [ -d "${STEAM_NATIVE}" ]; then
    STEAM_DIR="${STEAM_NATIVE}"
    echo -e "${GREEN}[✓] Steam Nativo: ${STEAM_DIR}${NC}"
else
    echo -e "${RED}[✗] Steam no encontrado${NC}"
    exit 1
fi

# Verificar Delta Force
DELTA_FORCE=$(find "${STEAM_DIR}/steamapps/common" -maxdepth 1 -type d -iname "*delta*force*" 2>/dev/null | head -n1)

if [ -n "${DELTA_FORCE}" ]; then
    echo -e "${GREEN}[✓] Delta Force: ${DELTA_FORCE}${NC}"
else
    echo -e "${RED}[✗] Delta Force no encontrado en Steam${NC}"
fi

echo ""

# Buscar logs de Steam
STEAM_LOGS="${STEAM_DIR}/logs"

if [ -d "${STEAM_LOGS}" ]; then
    echo -e "${BLUE}════════════════════════════════════════════════════════════════${NC}"
    echo -e "${BLUE}Logs de Steam (últimos archivos modificados):${NC}"
    echo -e "${BLUE}════════════════════════════════════════════════════════════════${NC}"
    
    # Mostrar los 3 archivos de log más recientes
    find "${STEAM_LOGS}" -type f -name "*.txt" -o -name "*.log" | xargs ls -lt | head -5
    
    echo ""
    echo -e "${YELLOW}Para ver un log específico:${NC}"
    echo "cat ${STEAM_LOGS}/<nombre_del_archivo>"
    echo ""
fi

# Verificar compatdata (Wine Prefix)
COMPATDATA="${STEAM_DIR}/steamapps/compatdata"

if [ -d "${COMPATDATA}" ]; then
    echo -e "${BLUE}════════════════════════════════════════════════════════════════${NC}"
    echo -e "${BLUE}Wine Prefixes disponibles (compatdata):${NC}"
    echo -e "${BLUE}════════════════════════════════════════════════════════════════${NC}"
    
    find "${COMPATDATA}" -maxdepth 2 -type d -name "pfx" | while read prefix; do
        appid=$(echo "$prefix" | grep -oP 'compatdata/\K[0-9]+')
        echo -e "${GREEN}AppID ${appid}:${NC} $prefix"
    done
    echo ""
fi

# Verificar ACE drivers
ACE_DIR="${HOME}/Documentos/PROYECTOS/kernelBridge/Win64/AntiCheatExpert"

if [ -d "${ACE_DIR}" ]; then
    ACE_COUNT=$(find "${ACE_DIR}" -name "*.sys" | wc -l)
    echo -e "${GREEN}[✓] Drivers ACE: ${ACE_COUNT} archivos${NC}"
else
    echo -e "${RED}[✗] Directorio ACE no encontrado${NC}"
fi

echo ""
echo -e "${CYAN}════════════════════════════════════════════════════════════════${NC}"
echo -e "${CYAN}VERIFICACIÓN DE LAUNCH OPTIONS${NC}"
echo -e "${CYAN}════════════════════════════════════════════════════════════════${NC}"
echo ""
echo "Para verificar que las Launch Options están configuradas:"
echo ""
echo "1. Abre Steam"
echo "2. Click derecho en Delta Force → Propiedades"
echo "3. Verifica que en OPCIONES DE LANZAMIENTO esté:"
echo ""
echo -e "${GREEN}${WRAPPER} %command%${NC}"
echo ""
echo -e "${YELLOW}IMPORTANTE: Debe incluir '%command%' al final${NC}"
echo ""

echo -e "${CYAN}════════════════════════════════════════════════════════════════${NC}"
echo -e "${CYAN}PASOS PARA DIAGNOSTICAR${NC}"
echo -e "${CYAN}════════════════════════════════════════════════════════════════${NC}"
echo ""
echo "1. Verifica que las Launch Options estén correctas en Steam"
echo "2. Intenta ejecutar Delta Force desde Steam"
echo "3. Vuelve a ejecutar este script para ver los logs actualizados"
echo "4. Si el wrapper log está vacío, el problema está en Steam"
echo "5. Si el wrapper log tiene errores, corregirlos primero"
echo ""

echo -e "${BLUE}════════════════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}PRUEBA MANUAL DEL WRAPPER${NC}"
echo -e "${BLUE}════════════════════════════════════════════════════════════════${NC}"
echo ""
echo "Para probar el wrapper manualmente (sin Steam):"
echo ""
echo -e "${GREEN}${WRAPPER} echo \"Test ejecutado\"${NC}"
echo ""
echo "Esto debería mostrar logs y ejecutar el comando de prueba."
echo ""
