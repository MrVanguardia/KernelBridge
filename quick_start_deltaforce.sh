#!/bin/bash
# Script de inicio rápido para Delta Force
# Compila si es necesario y lanza el juego

set -e

cd "$(dirname "$0")"

echo "🎮 KernelBridge - Delta Force Quick Start"
echo "=========================================="
echo ""

# Verificar si el daemon está compilado
if [ ! -f "daemon/target/release/kernelbridge-daemon" ]; then
    echo "📦 Compilando daemon (primera vez)..."
    cd daemon
    cargo build --release 2>&1 | grep -E "(Compiling|Finished)" || true
    cd ..
    echo "✅ Daemon compilado"
fi

# Verificar si la GUI está compilada
if [ ! -f "gui/target/release/kernelbridge-gui" ]; then
    echo "📦 Compilando GUI (primera vez)..."
    cd gui
    cargo build --release 2>&1 | grep -E "(Compiling|Finished)" || true
    cd ..
    echo "✅ GUI compilada"
fi

# Verificar Delta Force en Steam
FOUND=false
for steam_dir in \
    "$HOME/.local/share/Steam" \
    "$HOME/.steam/steam" \
    "$HOME/.var/app/com.valvesoftware.Steam/.local/share/Steam"; do
    
    if [ -d "$steam_dir/steamapps/common/Delta Force" ]; then
        FOUND=true
        echo "✅ Delta Force encontrado en Steam"
        break
    fi
done

if [ "$FOUND" = false ]; then
    echo "⚠️  Delta Force no encontrado en Steam"
    echo "   Por favor instala Delta Force desde Steam primero"
    echo ""
    echo "   ¿Quieres abrir Steam ahora? (s/n)"
    read -r response
    if [ "$response" = "s" ] || [ "$response" = "S" ]; then
        steam &
    fi
    exit 1
fi

echo ""
echo "🚀 Lanzando Delta Force..."
echo ""

# Ejecutar script de lanzamiento
exec ./launch_deltaforce.sh "$@"
