#!/bin/bash
# Instalador de alias para facilitar el lanzamiento de Delta Force

echo "🔧 Instalando alias de KernelBridge..."
echo ""

# Detectar shell del usuario
SHELL_RC=""
if [ -f "$HOME/.bashrc" ]; then
    SHELL_RC="$HOME/.bashrc"
elif [ -f "$HOME/.zshrc" ]; then
    SHELL_RC="$HOME/.zshrc"
else
    echo "❌ No se detectó .bashrc ni .zshrc"
    echo "   Agrega manualmente los alias a tu archivo de configuración"
    exit 1
fi

echo "📝 Agregando alias a $SHELL_RC"

# Crear backup
cp "$SHELL_RC" "${SHELL_RC}.backup-$(date +%Y%m%d)"
echo "✅ Backup creado: ${SHELL_RC}.backup-$(date +%Y%m%d)"

# Agregar alias si no existen
if ! grep -q "# KernelBridge Delta Force Aliases" "$SHELL_RC"; then
    cat >> "$SHELL_RC" << 'EOF'

# Agregar aliases al .bashrc
cat >> ~/.bashrc << 'EOF'

# ═══════════════════════════════════════════════════════════════
# KernelBridge + Delta Force Aliases
# ═══════════════════════════════════════════════════════════════

# Delta Force - Inicio rápido
alias deltaforce='cd ~/Documentos/PROYECTOS/kernelBridge && ./quick_start_deltaforce.sh'

# Delta Force - GUI con logs visibles
alias deltaforce-gui='cd ~/Documentos/PROYECTOS/kernelBridge && ./start_gui_deltaforce.sh'

# Delta Force - Verificación de sistema
alias deltaforce-verify='cd ~/Documentos/PROYECTOS/kernelBridge && ./verify_deltaforce.sh'

# KernelBridge - GUI normal
alias kb='cd ~/Documentos/PROYECTOS/kernelBridge && ./gui/target/release/kernelbridge-gui &'

# KernelBridge - GUI con debug completo (recomendado)
alias kb-debug='cd ~/Documentos/PROYECTOS/kernelBridge && ./kb_debug.sh'

# Logs de Steam wrapper
alias deltaforce-logs='cat ~/.cache/kernelbridge/steam_wrapper.log'

# Limpiar cache de shaders (si hay problemas de rendimiento)
alias deltaforce-clean='rm -rf ~/.cache/mesa_shader_cache/* ~/.cache/dxvk/* ~/.cache/vkd3d/* && echo "✅ Cache limpiado"'

EOF
EOF
    echo "✅ Alias agregados"
else
    echo "ℹ️  Alias ya existen, saltando..."
fi

echo ""
echo "🎯 Alias instalados:"
echo ""
echo "  deltaforce          - Lanzar Delta Force (quick start)"
echo "  deltaforce-gui      - Abrir GUI de KernelBridge"
echo "  deltaforce-verify   - Verificar sistema"
echo "  kb                  - Ir al directorio de KernelBridge"
echo ""
echo "Para usar los alias AHORA (sin reiniciar):"
echo "  source $SHELL_RC"
echo ""
echo "Después de reiniciar la terminal, simplemente escribe:"
echo "  deltaforce"
echo ""
echo "✅ ¡Instalación completada!"
