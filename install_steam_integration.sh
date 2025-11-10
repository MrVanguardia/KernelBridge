#!/bin/bash

################################################################################
# Instalador de Integración Steam + ACE
################################################################################
# Configura automáticamente Steam para que Delta Force use ACE.
#
# Este script:
# 1. Detecta Delta Force en tu biblioteca de Steam
# 2. Configura las Launch Options automáticamente
# 3. Verifica que todo esté listo
################################################################################

set -e

# Colores
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

clear
echo -e "${CYAN}╔════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║  Instalador de Integración Steam + ACE para Delta Force       ║${NC}"
echo -e "${CYAN}╚════════════════════════════════════════════════════════════════╝${NC}"
echo ""

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WRAPPER_SCRIPT="${SCRIPT_DIR}/steam_deltaforce_wrapper.sh"

# Verificar que el wrapper existe
if [ ! -f "${WRAPPER_SCRIPT}" ]; then
    echo -e "${RED}[✗] Error: No se encontró steam_deltaforce_wrapper.sh${NC}"
    echo "Ejecuta este script desde el directorio del proyecto kernelBridge"
    exit 1
fi

echo -e "${GREEN}[✓] Wrapper encontrado: ${WRAPPER_SCRIPT}${NC}"

# Detectar Steam
STEAM_FLATPAK="${HOME}/.var/app/com.valvesoftware.Steam/.local/share/Steam"
STEAM_NATIVE="${HOME}/.local/share/Steam"

if [ -d "${STEAM_FLATPAK}" ]; then
    STEAM_DIR="${STEAM_FLATPAK}"
    STEAM_TYPE="Flatpak"
    echo -e "${GREEN}[✓] Steam Flatpak detectado${NC}"
elif [ -d "${STEAM_NATIVE}" ]; then
    STEAM_DIR="${STEAM_NATIVE}"
    STEAM_TYPE="Nativo"
    echo -e "${GREEN}[✓] Steam nativo detectado${NC}"
else
    echo -e "${RED}[✗] Steam no encontrado${NC}"
    echo "Instala Steam primero."
    exit 1
fi

# Buscar Delta Force
echo ""
echo -e "${BLUE}[→] Buscando Delta Force en Steam...${NC}"
DELTA_FORCE_DIR=$(find "${STEAM_DIR}/steamapps/common" -maxdepth 1 -type d -iname "*delta*force*" 2>/dev/null | head -n1)

if [ -z "${DELTA_FORCE_DIR}" ]; then
    echo -e "${RED}[✗] Delta Force no encontrado en Steam${NC}"
    echo ""
    echo "Asegúrate de que Delta Force esté instalado en Steam."
    echo "Ve a tu biblioteca de Steam e instala Delta Force primero."
    exit 1
fi

echo -e "${GREEN}[✓] Delta Force encontrado:${NC} ${DELTA_FORCE_DIR}"

# Buscar el AppID de Delta Force
# El AppID está en el archivo appmanifest
APPMANIFEST=$(find "${STEAM_DIR}/steamapps" -maxdepth 1 -name "appmanifest_*.acf" -exec grep -l "Delta Force" {} \; 2>/dev/null | head -n1)

if [ -z "${APPMANIFEST}" ]; then
    echo -e "${YELLOW}[!] No se pudo detectar el AppID automáticamente${NC}"
    APP_ID="unknown"
else
    APP_ID=$(basename "${APPMANIFEST}" | sed 's/appmanifest_//; s/.acf//')
    echo -e "${GREEN}[✓] AppID detectado: ${APP_ID}${NC}"
fi

# Verificar ACE drivers
ACE_DIR="${SCRIPT_DIR}/Win64/AntiCheatExpert"
if [ ! -d "${ACE_DIR}" ]; then
    echo -e "${RED}[✗] Directorio ACE no encontrado: ${ACE_DIR}${NC}"
    exit 1
fi

ACE_COUNT=$(find "${ACE_DIR}" -name "*.sys" 2>/dev/null | wc -l)
echo -e "${GREEN}[✓] Drivers ACE encontrados: ${ACE_COUNT} archivos${NC}"

# Mostrar instrucciones
echo ""
echo -e "${CYAN}╔════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║  INSTRUCCIONES DE CONFIGURACIÓN                               ║${NC}"
echo -e "${CYAN}╚════════════════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${YELLOW}Para que Delta Force use ACE desde Steam, debes configurar las${NC}"
echo -e "${YELLOW}Launch Options manualmente:${NC}"
echo ""
echo -e "${BLUE}1.${NC} Abre Steam"
echo -e "${BLUE}2.${NC} Ve a tu Biblioteca"
echo -e "${BLUE}3.${NC} Click derecho en ${GREEN}Delta Force${NC}"
echo -e "${BLUE}4.${NC} Selecciona ${GREEN}Propiedades${NC}"
echo -e "${BLUE}5.${NC} En la sección ${GREEN}OPCIONES DE LANZAMIENTO${NC} (Launch Options)"
echo -e "${BLUE}6.${NC} Pega exactamente esta línea:"
echo ""
echo -e "${GREEN}┌────────────────────────────────────────────────────────────────┐${NC}"
echo -e "${GREEN}│${NC} ${WRAPPER_SCRIPT} %command% ${GREEN}│${NC}"
echo -e "${GREEN}└────────────────────────────────────────────────────────────────┘${NC}"
echo ""

# Copiar al clipboard si xclip está disponible
if command -v xclip &> /dev/null; then
    echo "${WRAPPER_SCRIPT} %command%" | xclip -selection clipboard
    echo -e "${GREEN}[✓] Comando copiado al portapapeles${NC}"
    echo -e "Puedes pegarlo directamente con ${CYAN}Ctrl+V${NC}"
    echo ""
elif command -v wl-copy &> /dev/null; then
    echo "${WRAPPER_SCRIPT} %command%" | wl-copy
    echo -e "${GREEN}[✓] Comando copiado al portapapeles (Wayland)${NC}"
    echo -e "Puedes pegarlo directamente con ${CYAN}Ctrl+V${NC}"
    echo ""
fi

echo -e "${BLUE}7.${NC} Click ${GREEN}Cerrar${NC}"
echo -e "${BLUE}8.${NC} Lanza Delta Force normalmente desde Steam"
echo ""

echo -e "${CYAN}╔════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║  ¿QUÉ SUCEDERÁ?                                                ║${NC}"
echo -e "${CYAN}╚════════════════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "Cuando hagas click en ${GREEN}JUGAR${NC} en Steam:"
echo ""
echo -e "  ${BLUE}→${NC} Steam ejecutará el wrapper script primero"
echo -e "  ${BLUE}→${NC} El wrapper configurará ACE automáticamente"
echo -e "  ${BLUE}→${NC} Se copiarán los drivers ACE al Wine Prefix"
echo -e "  ${BLUE}→${NC} Se crearán las claves de registro necesarias"
echo -e "  ${BLUE}→${NC} Delta Force se iniciará con ACE funcionando"
echo ""

echo -e "${YELLOW}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${YELLOW}NOTA IMPORTANTE:${NC}"
echo -e "${YELLOW}═══════════════════════════════════════════════════════════════${NC}"
echo ""
echo -e "ACE es un anti-cheat ${RED}kernel-level${NC} diseñado para Windows."
echo -e "En Linux con Wine/Proton:"
echo ""
echo -e "  ${GREEN}✓${NC} Modo campaña/offline: ${GREEN}Debería funcionar${NC}"
echo -e "  ${YELLOW}?${NC} Modo multijugador: ${YELLOW}Puede funcionar o detectar Wine${NC}"
echo -e "  ${RED}✗${NC} Riesgo de ban: ${RED}Posible si ACE detecta el entorno${NC}"
echo ""
echo -e "Recomendación: ${CYAN}Prueba primero en modo offline/campaña${NC}"
echo ""

# Crear un script de desinstalación
UNINSTALL_SCRIPT="${SCRIPT_DIR}/uninstall_steam_integration.sh"
cat > "${UNINSTALL_SCRIPT}" << 'UNINSTALL_EOF'
#!/bin/bash
echo "Para desinstalar la integración Steam + ACE:"
echo ""
echo "1. Abre Steam"
echo "2. Click derecho en Delta Force → Propiedades"
echo "3. Borra el contenido de OPCIONES DE LANZAMIENTO"
echo "4. Click Cerrar"
echo ""
echo "Delta Force volverá a ejecutarse normalmente sin ACE."
UNINSTALL_EOF

chmod +x "${UNINSTALL_SCRIPT}"

echo -e "${GREEN}[✓] Script de desinstalación creado: ${UNINSTALL_SCRIPT}${NC}"
echo ""

echo -e "${CYAN}╔════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║  RESUMEN                                                       ║${NC}"
echo -e "${CYAN}╚════════════════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${GREEN}Steam:${NC} ${STEAM_TYPE} (${STEAM_DIR})"
echo -e "${GREEN}Delta Force:${NC} Instalado (${DELTA_FORCE_DIR})"
if [ "${APP_ID}" != "unknown" ]; then
    echo -e "${GREEN}AppID:${NC} ${APP_ID}"
fi
echo -e "${GREEN}ACE Drivers:${NC} ${ACE_COUNT} archivos listos"
echo -e "${GREEN}Wrapper:${NC} ${WRAPPER_SCRIPT}"
echo ""

echo -e "${BLUE}¿Ya configuraste las Launch Options en Steam?${NC}"
echo -e "Si ya lo hiciste, simplemente lanza Delta Force desde Steam."
echo ""
echo -e "${GREEN}¡Disfruta jugando Delta Force en Linux con ACE! 🎮${NC}"
echo ""
