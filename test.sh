#!/bin/bash
# Script de pruebas para KernelBridge

echo "=== Pruebas de KernelBridge ==="

# Verificar TPM
echo "Verificando TPM..."
if command -v tpm2_getrandom &> /dev/null; then
    if tpm2_getrandom 8 &> /dev/null; then
        echo "✓ TPM operativo"
    else
        echo "✗ TPM no operativo"
    fi
else
    echo "✗ tpm2-tools no instalado"
fi

# Verificar integridad
echo "Verificando integridad..."
if [ -f /sys/kernel/security/ima/status ]; then
    echo "✓ IMA disponible"
else
    echo "✗ IMA no disponible"
fi

# Verificar compilación
echo "Verificando compilación..."
cd gui
if cargo check &> /dev/null; then
    echo "✓ GUI compila"
else
    echo "✗ GUI no compila"
fi

cd ../daemon
if cargo check &> /dev/null; then
    echo "✓ Daemon compila"
else
    echo "✗ Daemon no compila"
fi

echo "=== Fin de pruebas ==="