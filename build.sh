#!/bin/bash
# Script de build para KernelBridge

echo "Construyendo KernelBridge..."

# Compilar daemon
echo "Compilando daemon..."
cd daemon
cargo build --release
cd ..

# Compilar GUI
echo "Compilando GUI..."
cd gui
cargo build --release
cd ..

echo "Build completado. Ejecutables en daemon/target/release/ y gui/target/release/"