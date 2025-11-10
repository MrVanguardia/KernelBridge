#!/bin/bash

# Instalar herramientas de rendimiento para AMD
echo "Instalando GameMode y MangoHud para optimizar Delta Force en AMD..."

sudo dnf install -y gamemode mangohud

echo ""
echo "✅ GameMode instalado - Optimiza CPU/GPU automáticamente"
echo "✅ MangoHud instalado - Overlay de FPS y estadísticas"
echo ""
echo "════════════════════════════════════════════════════════════════"
echo "CONFIGURACIÓN ÓPTIMA PARA STEAM"
echo "════════════════════════════════════════════════════════════════"
echo ""
echo "En Steam → Delta Force → Propiedades → Launch Options, usa:"
echo ""
echo "gamemoderun mangohud ~/.var/app/com.valvesoftware.Steam/data/scripts/steam_deltaforce_wrapper.sh %command%"
echo ""
echo "════════════════════════════════════════════════════════════════"
echo ""
echo "Esto te dará:"
echo "  ✓ Máximo rendimiento GPU AMD (RADV optimizado)"
echo "  ✓ GameMode activo (CPU/GPU en modo performance)"
echo "  ✓ MangoHud overlay (FPS, temp, uso de recursos)"
echo "  ✓ ACE configurado automáticamente"
echo "  ✓ GE-Proton 10-25 con todas las optimizaciones"
echo ""
echo "Lee AMD_OPTIMIZATIONS.md para más detalles."
echo ""
