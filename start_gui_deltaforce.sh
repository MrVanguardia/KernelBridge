#!/bin/bash
################################################################################
# KernelBridge GUI - Modo Debug con Logs Visibles
################################################################################

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

clear
echo -e "${CYAN}╔════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║   KernelBridge GUI - Modo Debug con Logs Completos            ║${NC}"
echo -e "${CYAN}╚════════════════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${YELLOW}🚀 Iniciando KernelBridge GUI con soporte de Delta Force${NC}"
echo ""
echo -e "${BLUE}📋 Instrucciones:${NC}"
echo "  1. La GUI se abrirá en una ventana nueva"
echo "  2. Ve a la sección '🎮 Juegos'"
echo "  3. Click en '🎯 Lanzar Delta Force (Quick Start)'"
echo -e "  4. ${GREEN}TODOS los logs aparecerán AQUÍ en esta terminal${NC}"
echo ""
echo -e "${YELLOW}⚠️  IMPORTANTE: NO cierres esta terminal mientras uses Delta Force${NC}"
echo ""

# Cambiar al directorio del proyecto
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$SCRIPT_DIR"

echo -e "${BLUE}📂 Directorio del proyecto: ${SCRIPT_DIR}${NC}"
echo ""

# Verificar que los scripts existan
if [ ! -f "quick_start_deltaforce.sh" ]; then
    echo -e "${RED}❌ ERROR: No se encontró quick_start_deltaforce.sh${NC}"
    echo "   Asegúrate de estar en el directorio correcto del proyecto"
    exit 1
fi

echo -e "${GREEN}✅ Scripts de Delta Force encontrados${NC}"

if [ ! -f "gui/target/release/kernelbridge-gui" ]; then
    echo -e "${YELLOW}⚠️  La GUI no está compilada. Compilando...${NC}"
    cd gui
    cargo build --release 2>&1 | while IFS= read -r line; do
        echo -e "${BLUE}[COMPILE]${NC} $line"
    done
    cd ..
    echo -e "${GREEN}✅ GUI compilada${NC}"
else
    echo -e "${GREEN}✅ GUI ya compilada${NC}"
fi

echo ""
echo -e "${CYAN}════════════════════════════════════════════════════════════════${NC}"
echo -e "${CYAN}                    LOGS EN VIVO${NC}"
echo -e "${CYAN}════════════════════════════════════════════════════════════════${NC}"
echo ""
echo "Iniciando GUI en 2 segundos..."
sleep 2
echo ""

# Habilitar logging detallado
export RUST_BACKTRACE=1
export RUST_LOG=debug

# Ejecutar la GUI mostrando TODA la salida
./gui/target/release/kernelbridge-gui 2>&1 | while IFS= read -r line; do
    # Colorear diferentes tipos de mensajes
    if [[ "$line" =~ ERROR|Error|error|❌ ]]; then
        echo -e "${RED}$line${NC}"
    elif [[ "$line" =~ WARN|Warning|warning|⚠️ ]]; then
        echo -e "${YELLOW}$line${NC}"
    elif [[ "$line" =~ SUCCESS|✅|✓ ]]; then
        echo -e "${GREEN}$line${NC}"
    elif [[ "$line" =~ DELTA.*FORCE|🎯|🎮 ]]; then
        echo -e "${CYAN}$line${NC}"
    else
        echo "$line"
    fi
done

echo ""
echo -e "${CYAN}════════════════════════════════════════════════════════════════${NC}"
echo -e "${YELLOW}GUI cerrada${NC}"
echo -e "${CYAN}════════════════════════════════════════════════════════════════${NC}"
