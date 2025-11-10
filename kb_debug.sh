#!/bin/bash

################################################################################
# KernelBridge - Launcher con Debug Automático
################################################################################
# Este script ejecuta la GUI mostrando TODOS los logs en tiempo real.
# Perfecto para ver errores, debugging y seguir el proceso de Delta Force.
################################################################################

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
MAGENTA='\033[0;35m'
NC='\033[0m'

clear
cat << "EOF"
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║   ██╗  ██╗███████╗██████╗ ███╗   ██╗███████╗██╗     ██████╗██████╗  ║
║   ██║ ██╔╝██╔════╝██╔══██╗████╗  ██║██╔════╝██║     ██╔══██╗██╔══██╗║
║   █████╔╝ █████╗  ██████╔╝██╔██╗ ██║█████╗  ██║     ██████╔╝██████╔╝║
║   ██╔═██╗ ██╔══╝  ██╔══██╗██║╚██╗██║██╔══╝  ██║     ██╔══██╗██╔══██╗║
║   ██║  ██╗███████╗██║  ██║██║ ╚████║███████╗███████╗██████╔╝██║  ██║║
║   ╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝╚═╝  ╚═══╝╚══════╝╚══════╝╚═════╝ ╚═╝  ╚═╝║
║                                                                       ║
║                    Modo Debug - Logs Completos                       ║
╚═══════════════════════════════════════════════════════════════════════╝
EOF

echo ""
echo -e "${CYAN}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${CYAN}                INFORMACIÓN DEL SISTEMA${NC}"
echo -e "${CYAN}═══════════════════════════════════════════════════════════════${NC}"
echo ""

# Mostrar info del sistema
echo -e "${BLUE}Sistema Operativo:${NC} $(cat /etc/os-release | grep PRETTY_NAME | cut -d= -f2 | tr -d '"')"
echo -e "${BLUE}Kernel:${NC} $(uname -r)"
echo -e "${BLUE}GPU:${NC} $(lspci | grep -i vga | cut -d: -f3)"
echo -e "${BLUE}Mesa:${NC} $(glxinfo | grep "OpenGL version" | cut -d: -f2 || echo 'No detectado')"
echo ""

# Cambiar al directorio del proyecto
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$SCRIPT_DIR"

echo -e "${BLUE}Directorio:${NC} ${SCRIPT_DIR}"
echo ""

echo -e "${CYAN}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${CYAN}                VERIFICACIÓN DE COMPONENTES${NC}"
echo -e "${CYAN}═══════════════════════════════════════════════════════════════${NC}"
echo ""

# Verificar scripts
COMPONENTS=(
    "quick_start_deltaforce.sh:Script de inicio rápido"
    "launch_deltaforce.sh:Launcher principal"
    "steam_deltaforce_wrapper.sh:Wrapper de Steam"
    "Win64/AntiCheatExpert:Drivers ACE"
)

ALL_OK=true
for component in "${COMPONENTS[@]}"; do
    IFS=':' read -r file desc <<< "$component"
    if [ -e "$file" ]; then
        echo -e "${GREEN}✅${NC} ${desc}"
    else
        echo -e "${RED}❌${NC} ${desc} - NO ENCONTRADO"
        ALL_OK=false
    fi
done

echo ""

# Verificar Steam
if [ -d "$HOME/.var/app/com.valvesoftware.Steam" ]; then
    echo -e "${GREEN}✅${NC} Steam Flatpak detectado"
    
    # Verificar Delta Force
    DF_DIR=$(find "$HOME/.var/app/com.valvesoftware.Steam/.local/share/Steam/steamapps/common" -maxdepth 1 -type d -iname "*delta*force*" 2>/dev/null | head -n1)
    if [ -n "$DF_DIR" ]; then
        echo -e "${GREEN}✅${NC} Delta Force instalado: $(basename "$DF_DIR")"
    else
        echo -e "${YELLOW}⚠️${NC}  Delta Force no detectado (¿no instalado?)"
    fi
    
    # Verificar GE-Proton
    GE_PROTON=$(find "$HOME/.var/app/com.valvesoftware.Steam/.local/share/Steam/compatibilitytools.d" -maxdepth 1 -type d -name "GE-Proton*" 2>/dev/null | head -n1)
    if [ -n "$GE_PROTON" ]; then
        echo -e "${GREEN}✅${NC} GE-Proton: $(basename "$GE_PROTON")"
    else
        echo -e "${YELLOW}⚠️${NC}  GE-Proton no detectado"
    fi
elif [ -d "$HOME/.local/share/Steam" ]; then
    echo -e "${GREEN}✅${NC} Steam nativo detectado"
else
    echo -e "${YELLOW}⚠️${NC}  Steam no detectado"
fi

echo ""

# Verificar GUI compilada
GUI_BIN="./gui/target/release/kernelbridge-gui"
if [ ! -f "$GUI_BIN" ]; then
    echo -e "${YELLOW}⚠️  GUI no compilada${NC}"
    echo -e "${BLUE}[→] Compilando GUI...${NC}"
    echo ""
    
    cd gui
    cargo build --release 2>&1 | while IFS= read -r line; do
        if [[ "$line" =~ Compiling|Building ]]; then
            echo -e "${BLUE}   $line${NC}"
        elif [[ "$line" =~ Finished ]]; then
            echo -e "${GREEN}   $line${NC}"
        elif [[ "$line" =~ warning ]]; then
            echo -e "${YELLOW}   $line${NC}" >&2
        fi
    done
    cd ..
    
    if [ -f "$GUI_BIN" ]; then
        echo ""
        echo -e "${GREEN}✅ GUI compilada exitosamente${NC}"
    else
        echo ""
        echo -e "${RED}❌ Error al compilar la GUI${NC}"
        exit 1
    fi
else
    echo -e "${GREEN}✅${NC} GUI ya compilada"
fi

echo ""
echo -e "${CYAN}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${CYAN}                 INSTRUCCIONES DE USO${NC}"
echo -e "${CYAN}═══════════════════════════════════════════════════════════════${NC}"
echo ""
echo -e "${YELLOW}1.${NC} La GUI se abrirá en una ${GREEN}ventana nueva${NC}"
echo -e "${YELLOW}2.${NC} Ve a la sección ${CYAN}'🎮 Juegos'${NC}"
echo -e "${YELLOW}3.${NC} Click en ${CYAN}'🎯 Lanzar Delta Force (Quick Start)'${NC}"
echo -e "${YELLOW}4.${NC} ${MAGENTA}TODOS los logs aparecerán AQUÍ en esta terminal${NC}"
echo ""
echo -e "${RED}⚠️  IMPORTANTE:${NC}"
echo -e "   ${YELLOW}→ NO cierres esta terminal mientras uses Delta Force${NC}"
echo -e "   ${YELLOW}→ Aquí verás errores, warnings y progreso${NC}"
echo -e "   ${YELLOW}→ Útil para debugging si algo no funciona${NC}"
echo ""

echo -e "${CYAN}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${CYAN}                    LOGS EN TIEMPO REAL${NC}"
echo -e "${CYAN}═══════════════════════════════════════════════════════════════${NC}"
echo ""

# Configurar logging detallado
export RUST_BACKTRACE=full
export RUST_LOG=debug

echo -e "${GREEN}Iniciando GUI en 3 segundos...${NC}"
sleep 1
echo -e "${YELLOW}3...${NC}"
sleep 1
echo -e "${YELLOW}2...${NC}"
sleep 1
echo -e "${YELLOW}1...${NC}"
sleep 1
echo ""

# Timestamp para logs
get_timestamp() {
    date '+%H:%M:%S'
}

# Ejecutar GUI con logs coloreados
"$GUI_BIN" 2>&1 | while IFS= read -r line; do
    timestamp="[$(get_timestamp)]"
    
    # Colorear según el contenido
    if [[ "$line" =~ ERROR|Error|error|❌|FAILED|Failed ]]; then
        echo -e "${timestamp} ${RED}${line}${NC}"
    elif [[ "$line" =~ WARN|Warning|warning|⚠️ ]]; then
        echo -e "${timestamp} ${YELLOW}${line}${NC}"
    elif [[ "$line" =~ SUCCESS|success|✅|✓|Completado|completado ]]; then
        echo -e "${timestamp} ${GREEN}${line}${NC}"
    elif [[ "$line" =~ DELTA.*FORCE|Delta.*Force|🎯|ACE ]]; then
        echo -e "${timestamp} ${CYAN}${line}${NC}"
    elif [[ "$line" =~ Steam|STEAM|Proton|PROTON ]]; then
        echo -e "${timestamp} ${MAGENTA}${line}${NC}"
    elif [[ "$line" =~ \[.*\] ]]; then
        echo -e "${timestamp} ${BLUE}${line}${NC}"
    else
        echo -e "${timestamp} ${line}"
    fi
done

# GUI cerrada
echo ""
echo -e "${CYAN}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${YELLOW}                    GUI CERRADA${NC}"
echo -e "${CYAN}═══════════════════════════════════════════════════════════════${NC}"
echo ""
echo "Gracias por usar KernelBridge!"
echo ""
