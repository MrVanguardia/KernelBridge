#!/bin/bash
# Script de inicio (requiere sudo)

# Asegurar privilegios de administrador
if [ "$(id -u)" -ne 0 ]; then
    echo "Este script requiere privilegios de administrador. Ejecuta:"
    echo "  sudo bash start.sh"
    exit 1
fi
# Verificar y compilar GUI
echo "Intentando compilar GUI..."

# Ejecutar TODO como root: fijar contexto raíz explícitamente
RUNTIME_USER=""   # vacío para no re-bajar privilegios
RUNTIME_HOME="/root"
RUNTIME_UID="0"

# Usar un directorio de target por-usuario real para evitar problemas de permisos
export KB_CACHE_DIR="${XDG_CACHE_HOME:-$RUNTIME_HOME/.cache}/kernelbridge"
export CARGO_TARGET_DIR="$KB_CACHE_DIR/target"
mkdir -p "$CARGO_TARGET_DIR"
cd gui
if cargo build --release; then
    GUI_AVAILABLE=true
    echo "GUI compilada exitosamente"
else
    GUI_AVAILABLE=false
    echo "GUI no disponible. Error de compilación."
fi
cd ..
# Automatiza la compilación y ejecución

echo "Iniciando KernelBridge..."

if [ -n "$SUDO_USER" ]; then
    echo "[INFO] Ejecutando como root (invocado por: $SUDO_USER)"
else
    echo "[INFO] Ejecutando como root (sin SUDO_USER; posible 'su' en lugar de 'sudo')"
fi

# Función para verificar si un comando existe
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Verificar Rust
if ! command_exists cargo; then
    echo "Error: Rust no está instalado. Instala Rust primero:"
    echo "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    echo "source ~/.cargo/env"
    exit 1
fi

# Si ya hay un daemon corriendo, detenerlo limpiamente y eliminar sockets viejos
if [ -f /tmp/kernelbridge-daemon.pid ]; then
    OLD_PID=$(cat /tmp/kernelbridge-daemon.pid 2>/dev/null || true)
    if [ -n "$OLD_PID" ] && kill -0 "$OLD_PID" >/dev/null 2>&1; then
        echo "Encontrado daemon previo (PID $OLD_PID). Deteniendo..."
        if command_exists socat; then
            echo -n "SHUTDOWN" | socat - UNIX-CONNECT:/tmp/kernelbridge.sock 2>/dev/null || kill "$OLD_PID" 2>/dev/null || true
        else
            kill "$OLD_PID" 2>/dev/null || true
        fi
        # Esperar breve para que libere sockets
        sleep 1
    fi
fi

# Limpiar sockets UNIX por si quedaron colgados
rm -f /tmp/kernelbridge.sock /tmp/kernelbridge_acgw.sock 2>/dev/null || true

# Compilar daemon (siempre necesario)
echo "Compilando daemon..."
cd daemon
if ! cargo build --release; then
    echo "Error compilando daemon"
    exit 1
fi
cd ..

# Iniciar daemon
echo "Iniciando daemon..."
# Helper para ejecutar como RUNTIME_USER si existe
# Ejecutor directo (como root)
run_as_user() { "$@"; }

# Construir entorno de ejecución preservando variables de sesión para que la GUI herede tema/portales/audio
run_env_common=("HOME=$RUNTIME_HOME")

# Usar XDG_RUNTIME_DIR de root si existe (muchas distros crean /run/user/0)
if [ -d "/run/user/$RUNTIME_UID" ]; then
    run_env_common+=("XDG_RUNTIME_DIR=/run/user/$RUNTIME_UID")
    run_env_common+=("DBUS_SESSION_BUS_ADDRESS=unix:path=/run/user/$RUNTIME_UID/bus")
fi

# Variables a pasar si existen (tema, sesión gráfica, idioma, audio)
for key in \
    DISPLAY WAYLAND_DISPLAY XAUTHORITY \
    XDG_SESSION_TYPE XDG_CURRENT_DESKTOP DESKTOP_SESSION XDG_DATA_DIRS \
    GIO_EXTRA_MODULES GTK_THEME GDK_SCALE GDK_DPI_SCALE QT_QPA_PLATFORMTHEME \
    LANG LC_ALL LC_CTYPE PULSE_SERVER PIPEWIRE_RUNTIME_DIR SSH_AUTH_SOCK; do
    val=$(printenv "$key")
    if [ -n "$val" ]; then
        run_env_common+=("$key=$val")
    fi
done

# Si corremos como sudo y no hay XAUTHORITY, intentar usar el del usuario real
if [ -n "$RUNTIME_USER" ] && [ -z "$(printenv XAUTHORITY)" ] && [ -f "$RUNTIME_HOME/.Xauthority" ]; then
    run_env_common+=("XAUTHORITY=$RUNTIME_HOME/.Xauthority")
fi

# Fallbacks de sesión si faltan (evita apariencia "oscura" o sin tema)
if [ -z "$(printenv XDG_DATA_DIRS)" ]; then
    run_env_common+=("XDG_DATA_DIRS=/usr/local/share:/usr/share")
fi

# Incluir export paths de Flatpak para que aparezcan launchers y temas de Flatpak
FLATPAK_EXPORT_SYS="/var/lib/flatpak/exports/share"
FLATPAK_EXPORT_USER="$RUNTIME_HOME/.local/share/flatpak/exports/share"
build_xdg_data_dirs() {
    local base="$1"
    local add=""
    if [ -d "$FLATPAK_EXPORT_SYS" ] && ! echo "$base" | grep -q "$FLATPAK_EXPORT_SYS"; then
        add="$FLATPAK_EXPORT_SYS"
    fi
    if [ -d "$FLATPAK_EXPORT_USER" ] && ! echo "$base" | grep -q "$FLATPAK_EXPORT_USER"; then
        if [ -n "$add" ]; then add="$add:$FLATPAK_EXPORT_USER"; else add="$FLATPAK_EXPORT_USER"; fi
    fi
    if [ -n "$add" ]; then
        if [ -n "$base" ]; then echo "$add:$base"; else echo "$add"; fi
    else
        echo "$base"
    fi
}

curr_xdg=$(printenv XDG_DATA_DIRS)
new_xdg=$(build_xdg_data_dirs "$curr_xdg")
if [ -n "$new_xdg" ]; then
    # Reemplazar/establecer XDG_DATA_DIRS con paths de Flatpak añadidos al frente
    # Nota: se aplicará a daemon y GUI
    # shellcheck disable=SC2206
    run_env_common+=("XDG_DATA_DIRS=$new_xdg")
fi

# Forzar tema claro opcionalmente: export KB_FORCE_LIGHT_THEME=1 (y opcional GTK_THEME_OVERRIDE)
if [ "${KB_FORCE_LIGHT_THEME:-0}" = "1" ]; then
    run_env_common+=("GTK_THEME=${GTK_THEME_OVERRIDE:-Adwaita:light}")
fi

run_as_user env "${run_env_common[@]}" "$CARGO_TARGET_DIR/release/kernelbridge-daemon" &
DAEMON_PID=$!

# Asegurar limpieza al salir del script (Ctrl+C o señales)
cleanup() {
    echo "\nDeteniendo procesos..."
    if [ -n "$GUI_PID" ] && kill -0 "$GUI_PID" >/dev/null 2>&1; then
        kill "$GUI_PID" 2>/dev/null || true
        wait "$GUI_PID" 2>/dev/null || true
    fi
    if kill -0 "$DAEMON_PID" >/dev/null 2>&1; then
        # Intentar apagado ordenado vía socket; si falla, SIGTERM
        if command_exists socat; then
            echo -n "SHUTDOWN" | socat - UNIX-CONNECT:/tmp/kernelbridge.sock 2>/dev/null || kill "$DAEMON_PID" 2>/dev/null || true
        else
            kill "$DAEMON_PID" 2>/dev/null || true
        fi
        wait "$DAEMON_PID" 2>/dev/null || true
    fi
    # Limpiar PID y sockets
    rm -f /tmp/kernelbridge-daemon.pid /tmp/kernelbridge.sock /tmp/kernelbridge_acgw.sock 2>/dev/null || true
}
trap cleanup INT TERM EXIT

# Esperar un poco para que inicie
sleep 2

# Preparar entorno específico para la GUI (forzar tema claro por defecto)
run_env_gui=("${run_env_common[@]}")
# Siempre forzar un tema claro salvo que el usuario lo sobrescriba con GTK_THEME_OVERRIDE
run_env_gui+=("GTK_THEME=${GTK_THEME_OVERRIDE:-Adwaita:light}")

# Si hay X11 y existe xhost, permitir al root conectarse al servidor X del usuario que invocó sudo
if command -v xhost >/dev/null 2>&1 && [ -n "$SUDO_USER" ] && [ -n "$(printenv DISPLAY)" ]; then
    # ejecutar xhost en la sesión del usuario para autorizar a root
    if command -v runuser >/dev/null 2>&1; then
        runuser -u "$SUDO_USER" -- xhost +SI:localuser:root >/dev/null 2>&1 || true
    else
        sudo -u "$SUDO_USER" -- xhost +SI:localuser:root >/dev/null 2>&1 || true
    fi
fi

# Iniciar GUI si está disponible
if [ "$GUI_AVAILABLE" = true ]; then
    echo "Iniciando GUI..."
    run_as_user env "${run_env_gui[@]}" "$CARGO_TARGET_DIR/release/kernelbridge-gui" &
    GUI_PID=$!
    echo "KernelBridge completo iniciado. Daemon PID: $DAEMON_PID, GUI PID: $GUI_PID"
    echo "Esperando a que se cierre la GUI para detener el daemon..."
    # Esperar a que la GUI termine; luego limpiar (trap hará el resto)
    wait "$GUI_PID"
else
    echo "Daemon iniciado. PID: $DAEMON_PID"
    echo "Para detener: kill $DAEMON_PID"
    # Mantener el script corriendo hasta Ctrl+C
    wait "$DAEMON_PID"
fi