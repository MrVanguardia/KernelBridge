#!/bin/bash

################################################################################
# Fix automático para Steam Flatpak + GE-Proton + ACE
################################################################################
# Este script configura todo para que Delta Force funcione desde Steam
# usando GE-Proton con ACE habilitado.
################################################################################

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

clear
echo -e "${CYAN}╔════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║  Fix Steam Flatpak + GE-Proton + ACE                          ║${NC}"
echo -e "${CYAN}╚════════════════════════════════════════════════════════════════╝${NC}"
echo ""

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
STEAM_FLATPAK_DATA="${HOME}/.var/app/com.valvesoftware.Steam"

# Verificar que es Steam Flatpak
if [ ! -d "${STEAM_FLATPAK_DATA}" ]; then
    echo -e "${RED}[✗] Steam Flatpak no detectado${NC}"
    echo ""
    echo "Este script es para Steam Flatpak."
    echo "Si usas Steam nativo, el wrapper debería funcionar directamente."
    exit 1
fi

echo -e "${GREEN}[✓] Steam Flatpak detectado${NC}"

# Crear directorio para scripts dentro del sandbox
STEAM_SCRIPTS="${STEAM_FLATPAK_DATA}/data/scripts"
STEAM_ACE="${STEAM_FLATPAK_DATA}/data/Win64"

echo -e "${BLUE}[→] Creando directorios en Steam...${NC}"
mkdir -p "${STEAM_SCRIPTS}"
mkdir -p "${STEAM_ACE}"

# Copiar el wrapper actualizado
echo -e "${BLUE}[→] Copiando wrapper script...${NC}"
cp -v "${SCRIPT_DIR}/steam_deltaforce_wrapper.sh" "${STEAM_SCRIPTS}/"
chmod +x "${STEAM_SCRIPTS}/steam_deltaforce_wrapper.sh"

# Copiar drivers ACE
echo -e "${BLUE}[→] Copiando drivers ACE...${NC}"
cp -rv "${SCRIPT_DIR}/Win64/AntiCheatExpert" "${STEAM_ACE}/"

# Actualizar el wrapper para usar las rutas del sandbox
WRAPPER_SANDBOX="${STEAM_SCRIPTS}/steam_deltaforce_wrapper.sh"

echo -e "${BLUE}[→] Actualizando rutas en el wrapper...${NC}"

# Reemplazar la ruta de ACE_DIR para que apunte al sandbox
sed -i "s|WIN64_DIR=\"\${SCRIPT_DIR}/Win64\"|WIN64_DIR=\"${HOME}/.var/app/com.valvesoftware.Steam/data/Win64\"|g" "${WRAPPER_SANDBOX}"

echo -e "${GREEN}[✓] Wrapper configurado para Steam Flatpak${NC}"

# Verificar GE-Proton
COMPATTOOLS="${STEAM_FLATPAK_DATA}/.local/share/Steam/compatibilitytools.d"

if [ -d "${COMPATTOOLS}" ]; then
    GEPROTON=$(find "${COMPATTOOLS}" -maxdepth 1 -type d -name "GE-Proton*" 2>/dev/null | head -n1)
    if [ -n "${GEPROTON}" ]; then
        GEPROTON_VERSION=$(basename "${GEPROTON}")
        echo -e "${GREEN}[✓] ${GEPROTON_VERSION} detectado${NC}"
    else
        echo -e "${YELLOW}[!] GE-Proton no encontrado en compatibilitytools.d${NC}"
    fi
fi

# Buscar Delta Force
STEAM_COMMON="${STEAM_FLATPAK_DATA}/.local/share/Steam/steamapps/common"
DELTA_FORCE=$(find "${STEAM_COMMON}" -maxdepth 1 -type d -iname "*delta*force*" 2>/dev/null | head -n1)

if [ -n "${DELTA_FORCE}" ]; then
    echo -e "${GREEN}[✓] Delta Force encontrado${NC}"
else
    echo -e "${YELLOW}[!] Delta Force no encontrado (¿no instalado aún?)${NC}"
fi

echo ""
echo -e "${CYAN}╔════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║  CONFIGURACIÓN COMPLETADA                                     ║${NC}"
echo -e "${CYAN}╚════════════════════════════════════════════════════════════════╝${NC}"
echo ""

echo -e "${GREEN}✓ Archivos copiados al sandbox de Steam${NC}"
echo -e "${GREEN}✓ Wrapper actualizado con rutas correctas${NC}"
echo -e "${GREEN}✓ Drivers ACE disponibles para Steam${NC}"
echo ""

echo -e "${CYAN}════════════════════════════════════════════════════════════════${NC}"
echo -e "${CYAN}SIGUIENTE PASO: Configurar Steam${NC}"
echo -e "${CYAN}════════════════════════════════════════════════════════════════${NC}"
echo ""

echo -e "${YELLOW}1. Abre Steam${NC}"
echo -e "${YELLOW}2. Ve a tu Biblioteca${NC}"
echo -e "${YELLOW}3. Click derecho en Delta Force → Propiedades${NC}"
echo ""

echo -e "${YELLOW}4. En COMPATIBILIDAD:${NC}"
echo -e "   ${GREEN}✓ Marca: Forzar el uso de una herramienta específica...${NC}"
echo -e "   ${GREEN}✓ Selecciona: GE-Proton10-25${NC}"
echo ""

echo -e "${YELLOW}5. En OPCIONES DE LANZAMIENTO, pega EXACTAMENTE:${NC}"
echo ""
echo -e "${GREEN}┌────────────────────────────────────────────────────────────────┐${NC}"
echo -e "${GREEN}│${NC} ~/.var/app/com.valvesoftware.Steam/data/scripts/steam_deltaforce_wrapper.sh %command% ${GREEN}│${NC}"
echo -e "${GREEN}└────────────────────────────────────────────────────────────────┘${NC}"
echo ""

# Copiar al clipboard
LAUNCH_CMD="~/.var/app/com.valvesoftware.Steam/data/scripts/steam_deltaforce_wrapper.sh %command%"

if command -v xclip &> /dev/null; then
    echo "${LAUNCH_CMD}" | xclip -selection clipboard
    echo -e "${GREEN}[✓] Comando copiado al portapapeles (Ctrl+V para pegar)${NC}"
elif command -v wl-copy &> /dev/null; then
    echo "${LAUNCH_CMD}" | wl-copy
    echo -e "${GREEN}[✓] Comando copiado al portapapeles (Ctrl+V para pegar)${NC}"
fi

echo ""
echo -e "${YELLOW}6. Click Cerrar${NC}"
echo -e "${YELLOW}7. Lanza Delta Force desde Steam normalmente${NC}"
echo ""

echo -e "${CYAN}════════════════════════════════════════════════════════════════${NC}"
echo -e "${CYAN}VERIFICACIÓN${NC}"
echo -e "${CYAN}════════════════════════════════════════════════════════════════${NC}"
echo ""

echo "Después de intentar ejecutar Delta Force, verifica los logs:"
echo ""
echo -e "${BLUE}cat ~/.cache/kernelbridge/steam_wrapper.log${NC}"
echo ""

echo -e "${CYAN}════════════════════════════════════════════════════════════════${NC}"
echo -e "${CYAN}OPTIMIZACIONES ADICIONALES PARA GE-PROTON${NC}"
echo -e "${CYAN}════════════════════════════════════════════════════════════════${NC}"
echo ""

echo "GE-Proton ya incluye optimizaciones, pero puedes agregar más:"
echo ""
echo "En Launch Options (ANTES del wrapper):"
echo ""
echo -e "${GREEN}PROTON_ENABLE_NVAPI=1 DXVK_ASYNC=1 ~/.var/app/com.valvesoftware.Steam/data/scripts/steam_deltaforce_wrapper.sh %command%${NC}"
echo ""

echo -e "${YELLOW}Esto habilita:${NC}"
echo "  • NVAPI: Mejor soporte NVIDIA"
echo "  • DXVK_ASYNC: Compilación asíncrona de shaders (mejor rendimiento)"
echo ""

echo -e "${CYAN}════════════════════════════════════════════════════════════════${NC}"
echo ""
echo -e "${GREEN}¡Todo listo! Ahora configura Steam y prueba Delta Force.${NC}"
echo ""
echo -e "${BLUE}Logs: ~/.cache/kernelbridge/steam_wrapper.log${NC}"
echo -e "${BLUE}Ayuda: cat ~/Documentos/PROYECTOS/kernelBridge/SOLUCION_STEAM_NO_INICIA.md${NC}"
echo ""
