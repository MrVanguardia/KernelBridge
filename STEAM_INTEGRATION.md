# 🎮 Guía: Delta Force desde Steam con ACE

Esta guía te muestra cómo ejecutar **Delta Force desde Steam** mientras mantienes **ACE (AntiCheatExpert) funcionando** en Linux Fedora 43.

---

## 🎯 ¿Qué hace esta integración?

Cuando ejecutas Delta Force desde Steam, automáticamente:

1. ✅ Detecta los drivers ACE
2. ✅ Configura el Wine Prefix de Steam
3. ✅ Copia los drivers al sistema
4. ✅ Crea las claves de registro necesarias
5. ✅ Inicia Delta Force normalmente

**Todo es automático. Tú solo das click en JUGAR.**

---

## 📋 Requisitos

Antes de comenzar, asegúrate de tener:

- ✅ Steam instalado (Flatpak o nativo)
- ✅ Delta Force instalado en Steam
- ✅ Drivers ACE en `Win64/AntiCheatExpert/`
- ✅ Proton configurado en Steam (Proton-GE recomendado)

---

## 🚀 Instalación (5 minutos)

### Paso 1: Ejecutar el instalador

Abre una terminal y ejecuta:

```bash
cd ~/Documentos/PROYECTOS/kernelBridge
./install_steam_integration.sh
```

Este script:
- Detectará tu instalación de Steam
- Buscará Delta Force
- Verificará los drivers ACE
- Te mostrará el comando que necesitas copiar

### Paso 2: Configurar Steam

El script te mostrará un comando como este:

```
/home/mrvanguardia/Documentos/PROYECTOS/kernelBridge/steam_deltaforce_wrapper.sh %command%
```

**Copia ese comando completo.**

Luego:

1. Abre **Steam**
2. Ve a tu **Biblioteca**
3. Click **derecho** en **Delta Force**
4. Selecciona **Propiedades**
5. En **OPCIONES DE LANZAMIENTO** (Launch Options)
6. **Pega** el comando que copiaste
7. Click **Cerrar**

**¡Listo!** Ya está configurado.

---

## 🎮 Uso

### Ejecutar Delta Force desde Steam

Simplemente abre Steam y:

1. Ve a tu **Biblioteca**
2. Selecciona **Delta Force**
3. Click en **JUGAR**

**Verás esto en la terminal de Steam:**

```
╔═══════════════════════════════════════════════════════════════╗
║   Steam Delta Force - Configuración ACE Automática           ║
╚═══════════════════════════════════════════════════════════════╝

[✓] Steam Flatpak detectado
[✓] Delta Force encontrado: /home/.../.local/share/Steam/steamapps/common/Delta Force
[✓] Wine Prefix: /home/.../.local/share/Steam/steamapps/compatdata/12345/pfx
[✓] Encontrados 14 drivers ACE
[→] Configurando Wine Prefix con ACE...
[→] Copiando drivers ACE...
[✓] Claves de registro ACE agregadas

╔═══════════════════════════════════════════════════════════════╗
║   Configuración ACE completada - Iniciando Delta Force       ║
╚═══════════════════════════════════════════════════════════════╝

Wine Prefix: /home/.../.local/share/Steam/steamapps/compatdata/12345/pfx
ACE Drivers: 14 archivos copiados
```

Luego Delta Force se iniciará normalmente con ACE funcionando.

---

## 🔍 Verificación

### Comprobar que ACE está activo

Mientras Delta Force está ejecutándose, abre otra terminal:

```bash
cd ~/Documentos/PROYECTOS/kernelBridge
./verify_deltaforce.sh
```

Deberías ver:

```
[✓] Drivers ACE: 14 encontrados
[✓] Wine Prefix configurado
[✓] Delta Force en ejecución
```

---

## 🎯 Modos de Juego

### ✅ Modo Campaña / Offline

**Funcionará perfectamente.**

ACE no necesita validación en línea para el modo campaña.

### ⚠️ Modo Multijugador

**Puede funcionar, pero hay riesgos:**

- ACE podría detectar que estás usando Wine/Proton
- Existe riesgo de baneo temporal o permanente
- Depende de cuán estricta sea la validación del servidor

**Recomendación:** Prueba primero en modo campaña.

---

## 🛠️ Solución de Problemas

### ❌ Steam no inicia Delta Force

**Verifica:**

```bash
# Ver si el wrapper tiene permisos
ls -la ~/Documentos/PROYECTOS/kernelBridge/steam_deltaforce_wrapper.sh

# Debería mostrar: -rwxr-xr-x (ejecutable)
```

Si no es ejecutable:

```bash
chmod +x ~/Documentos/PROYECTOS/kernelBridge/steam_deltaforce_wrapper.sh
```

### ❌ Error "No se encontró Wine Prefix"

Ejecuta Delta Force al menos una vez **sin** el wrapper para que Steam cree el Wine Prefix:

1. Borra las Launch Options en Steam temporalmente
2. Ejecuta Delta Force una vez (aunque falle con ACE)
3. Cierra Delta Force
4. Vuelve a poner las Launch Options
5. Ejecuta de nuevo

### ❌ ACE muestra error "Driver not loaded"

Esto es normal en Linux. ACE no puede cargar drivers kernel-level reales.

El wrapper **emula** la presencia de los drivers. Algunos juegos aceptan esto, otros no.

**Solución:** Si ACE bloquea completamente el juego:

- Prueba con **Proton-GE** en lugar de Proton estándar
- Considera usar una VM con GPU passthrough (configuración avanzada)

### ❌ Rendimiento bajo

Activa optimizaciones de Steam:

1. Steam → Configuración → Shader Pre-Caching: **Activado**
2. Steam → Configuración → Biblioteca de compatibilidad: **Proton-GE Latest**

Adicionalmente:

```bash
# Instalar GameMode
sudo dnf install gamemode

# Editar Launch Options para usar GameMode:
gamemoderun /home/mrvanguardia/Documentos/PROYECTOS/kernelBridge/steam_deltaforce_wrapper.sh %command%
```

---

## 🗑️ Desinstalar

Para volver a ejecutar Delta Force sin ACE:

1. Abre **Steam**
2. Click derecho en **Delta Force** → **Propiedades**
3. **Borra** el contenido de **OPCIONES DE LANZAMIENTO**
4. Click **Cerrar**

O ejecuta:

```bash
~/Documentos/PROYECTOS/kernelBridge/uninstall_steam_integration.sh
```

---

## 📊 Comparación: Steam vs Scripts Directos

| Característica | Steam (con wrapper) | Scripts directos | GUI KernelBridge |
|---|---|---|---|
| **Facilidad de uso** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐ |
| **Configuración ACE** | Automática | Automática | Automática |
| **Steam Overlay** | ✅ Funciona | ❌ No disponible | ❌ No disponible |
| **Steam Achievements** | ✅ Funciona | ❌ No disponible | ❌ No disponible |
| **Steam Input** | ✅ Funciona | ❌ No disponible | ❌ No disponible |
| **Proton-GE** | ✅ Fácil de usar | ⚠️ Manual | ⚠️ Manual |
| **Actualizaciones** | ✅ Automáticas | ⚠️ Manuales | ⚠️ Manuales |

**Recomendación:** Usa Steam si quieres la experiencia más integrada.

---

## 🎓 Detalles Técnicos

### ¿Cómo funciona el wrapper?

Cuando Steam ejecuta Delta Force:

```
Steam → steam_deltaforce_wrapper.sh → Delta Force.exe
```

El wrapper hace esto:

1. **Detecta** el Wine Prefix que Steam usa para Delta Force
2. **Copia** los drivers ACE (`*.sys`) a `C:\windows\system32\drivers\`
3. **Crea** claves de registro en `HKEY_LOCAL_MACHINE\System\CurrentControlSet\Services\`
4. **Configura** variables de entorno (DXVK, RADV, etc.)
5. **Ejecuta** el comando original de Steam (`%command%`)

Todo sucede en milisegundos antes de que el juego inicie.

### Variables de entorno configuradas

El wrapper configura estas optimizaciones:

```bash
WINEPREFIX=<Steam compatdata>
DXVK_HUD=0
WINE_LARGE_ADDRESS_AWARE=1
STAGING_SHARED_MEMORY=1
__GL_SHADER_DISK_CACHE=1
__GL_THREADED_OPTIMIZATION=1
mesa_glthread=true
RADV_PERFTEST=aco,sam
```

### Archivos modificados

El wrapper **NO modifica** ningún archivo del juego.

Solo agrega archivos al Wine Prefix de Steam:

```
~/.local/share/Steam/steamapps/compatdata/<AppID>/pfx/
├── drive_c/
│   └── windows/
│       └── system32/
│           └── drivers/
│               ├── ACE-BASE.sys
│               ├── ACE-BOOT.sys
│               ├── ACE-CORE.sys
│               └── ...
└── user.reg (claves ACE agregadas)
```

**Seguro y reversible.**

---

## ⚠️ Advertencias Importantes

### Riesgo de Baneo

ACE es un anti-cheat **kernel-level** diseñado para Windows.

En Linux con Wine/Proton:
- ACE **puede detectar** que no está en Windows real
- Esto podría resultar en **baneo temporal o permanente**
- Tencent tiene políticas **estrictas** contra "ambientes modificados"

**Usa bajo tu propio riesgo.**

**Recomendación:**
- Juega **solo en modo campaña/offline** primero
- **No uses** en cuentas principales/valiosas
- Crea una **cuenta secundaria** para pruebas

### Soporte de Tencent

Tencent **NO soporta oficialmente** Linux.

Si tienes problemas:
- **NO menciones** que usas Linux al contactar soporte
- **NO menciones** Wine/Proton/KernelBridge
- El soporte podría **negar ayuda** o **banear tu cuenta**

**Úsalo sabiendo que no hay soporte oficial.**

---

## 🆘 Soporte

Si tienes problemas:

1. **Verifica** que seguiste todos los pasos
2. **Lee** la sección de solución de problemas
3. **Revisa** los logs de Steam: `~/.local/share/Steam/logs/`
4. **Ejecuta** `./verify_deltaforce.sh` para diagnóstico

Documentación adicional:
- `DELTA_FORCE_README.md` - Configuración general
- `SOLUCION_PROBLEMAS_GUI.md` - Problemas de la GUI
- `TODAS_LAS_OPCIONES.md` - Todas las formas de ejecutar

---

## 🎉 ¡Disfruta Jugando!

Ahora puedes ejecutar Delta Force desde Steam como cualquier otro juego de Linux, con ACE funcionando automáticamente en segundo plano.

**¡Diviértete!** 🎮🐧
