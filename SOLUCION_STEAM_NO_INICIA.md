# 🚨 Solución: Delta Force no inicia desde Steam

## El Problema

Cuando usas **Steam Flatpak**, el wrapper script no funciona porque:

1. Steam Flatpak corre en un **sandbox**
2. No puede acceder a archivos fuera de ciertos directorios
3. `/home/mrvanguardia/Documentos/` está **fuera del sandbox**

## ✅ Solución 1: Copiar el wrapper dentro del sandbox de Steam

### Paso 1: Copiar el wrapper al directorio de Steam

```bash
# Crear directorio para scripts personalizados
mkdir -p ~/.var/app/com.valvesoftware.Steam/data/scripts

# Copiar el wrapper
cp ~/Documentos/PROYECTOS/kernelBridge/steam_deltaforce_wrapper.sh \
   ~/.var/app/com.valvesoftware.Steam/data/scripts/

# Copiar todo el directorio Win64
cp -r ~/Documentos/PROYECTOS/kernelBridge/Win64 \
   ~/.var/app/com.valvesoftware.Steam/data/

# Hacer ejecutable
chmod +x ~/.var/app/com.valvesoftware.Steam/data/scripts/steam_deltaforce_wrapper.sh
```

### Paso 2: Actualizar el wrapper para usar la nueva ruta

```bash
cd ~/Documentos/PROYECTOS/kernelBridge
./fix_steam_flatpak.sh
```

### Paso 3: Usar la nueva ruta en Steam Launch Options

En Steam → Delta Force → Propiedades → Launch Options:

```
~/.var/app/com.valvesoftware.Steam/data/scripts/steam_deltaforce_wrapper.sh %command%
```

---

## ✅ Solución 2: Dar permisos al Flatpak (Recomendado)

### Paso 1: Instalar Flatseal

```bash
flatpak install flathub com.github.tchx84.Flatseal
```

### Paso 2: Abrir Flatseal y configurar Steam

1. Abre **Flatseal**
2. Selecciona **Steam** en la lista
3. En **Filesystem**, clic en el **+**
4. Agrega: `/home/mrvanguardia/Documentos/PROYECTOS/kernelBridge`
5. Cierra Flatseal

### Paso 3: Reiniciar Steam

```bash
flatpak kill com.valvesoftware.Steam
```

### Paso 4: Configurar Launch Options

En Steam → Delta Force → Propiedades → Launch Options:

```
/home/mrvanguardia/Documentos/PROYECTOS/kernelBridge/steam_deltaforce_wrapper.sh %command%
```

---

## ✅ Solución 3: Usar scripts directos (más simple)

Si las soluciones anteriores fallan, **NO uses Steam Launch Options**.

En su lugar:

### Opción A: GUI de KernelBridge

```bash
cd ~/Documentos/PROYECTOS/kernelBridge
./start_gui_deltaforce.sh
```

Luego click en **"🎯 Lanzar Delta Force (Quick Start)"**

### Opción B: Script directo

```bash
cd ~/Documentos/PROYECTOS/kernelBridge
./quick_start_deltaforce.sh
```

### Opción C: Alias

```bash
# Instalar alias primero
cd ~/Documentos/PROYECTOS/kernelBridge
./install_aliases.sh
source ~/.bashrc

# Luego simplemente:
deltaforce
```

---

## 🔍 Verificar qué está pasando

### Ver logs del wrapper

```bash
cat ~/.cache/kernelbridge/steam_wrapper.log
```

### Ver si Steam está ejecutando el wrapper

```bash
cd ~/Documentos/PROYECTOS/kernelBridge
./check_steam_logs.sh
```

### Probar el wrapper manualmente

```bash
/home/mrvanguardia/Documentos/PROYECTOS/kernelBridge/steam_deltaforce_wrapper.sh echo "Test"
```

Si esto funciona pero Steam no, el problema es el sandbox de Flatpak.

---

## 📋 Resumen de Opciones

| Método | Dificultad | Integración Steam | ACE Funciona |
|--------|-----------|-------------------|--------------|
| **Flatseal + permisos** | ⭐⭐ | ✅ Completa | ✅ Sí |
| **Copiar dentro sandbox** | ⭐⭐⭐ | ✅ Completa | ✅ Sí |
| **GUI KernelBridge** | ⭐ | ❌ Separado | ✅ Sí |
| **Script directo** | ⭐ | ❌ Separado | ✅ Sí |
| **Alias** | ⭐ | ❌ Separado | ✅ Sí |

**Recomendación:** Usa **Flatseal** (Solución 2) si quieres la integración completa con Steam.

Si solo quieres jugar rápido, usa la **GUI o scripts directos** (Solución 3).

---

## 🛠️ Script automático de fix

He creado un script que hace todo automáticamente:

```bash
cd ~/Documentos/PROYECTOS/kernelBridge
./fix_steam_flatpak.sh
```

Este script:
1. Detecta si usas Steam Flatpak
2. Copia los archivos necesarios
3. Te da las instrucciones exactas

---

## ❓ ¿Por qué no funciona directamente?

Steam Flatpak tiene **restricciones de seguridad**:

```
Steam Flatpak puede acceder a:
✅ ~/.var/app/com.valvesoftware.Steam/
✅ ~/Descargas/
✅ Algunos directorios del sistema

Steam Flatpak NO puede acceder a:
❌ ~/Documentos/ (tu proyecto está aquí)
❌ Rutas arbitrarias fuera del sandbox
```

Por eso el wrapper no se ejecuta: **Steam no puede verlo**.

---

## 🎯 Solución Rápida (1 minuto)

**Opción más fácil - Sin tocar Steam:**

```bash
cd ~/Documentos/PROYECTOS/kernelBridge
./install_aliases.sh
source ~/.bashrc
deltaforce
```

**¡Listo!** El juego iniciará con ACE configurado.

Para futuras veces, simplemente escribe `deltaforce` en cualquier terminal.
