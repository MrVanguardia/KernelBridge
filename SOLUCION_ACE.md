# 🔧 Solución: Delta Force no inicia por ACE Anti-Cheat

## ❌ Problema Detectado

Los logs muestran que **ACE (Anti-Cheat Expert) está bloqueando el juego**:

```
Error: Wrong(or empty) table used to init mutex vanips
Error: IntHeartPort module: RegisterListener Success
LogOnline: Warning: STEAM: ReadyToUse succeeded, did not need to download
LogEasyAntiCheatServer: Display: LicenseType(OnlineAndOffline)
LogEasyAntiCheatServer: Display: Not using pipeline state disk cache per r:/OnlineRSettings.r.SkipPipelineStateCache
```

**ACE es un anti-cheat a nivel kernel de Windows que NO funciona en Linux con Wine/Proton.**

---

## ✅ Soluciones Disponibles

### 🎯 Opción 1: Usar Proton-GE con EasyAntiCheat (Más Fácil)

Delta Force también usa **EasyAntiCheat** (EAC), que SÍ funciona en Linux.

**Pasos:**

1. **Instalar GE-Proton más reciente:**
```bash
# Instalar ProtonUp-Qt
flatpak install flathub net.davidotek.pupgui2

# Abrir ProtonUp-Qt
flatpak run net.davidotek.pupgui2

# Instalar GE-Proton 9-20 o más reciente
```

2. **Configurar Steam para usar SOLO EasyAntiCheat:**

En Steam → Delta Force → Propiedades → OPCIONES DE LANZAMIENTO:

```bash
PROTON_USE_EAC_WORKAROUND=1 PROTON_NO_ESYNC=1 PROTON_NO_FSYNC=1 %command%
```

3. **Desactivar ACE en el registro de Wine:**

```bash
# Ejecutar ANTES de lanzar el juego
WINEPREFIX=~/.var/app/com.valvesoftware.Steam/.local/share/Steam/steamapps/compatdata/2507950/pfx \
wine regedit

# En el editor:
# 1. Ir a HKEY_LOCAL_MACHINE\System\CurrentControlSet\Services
# 2. Eliminar claves: ACE-BASE, ACE-CORE, ACE-BOOT
# 3. Cerrar regedit
```

---

### 🎯 Opción 2: Bypass ACE con DLL Override (Avanzado)

**ADVERTENCIA:** Esto puede resultar en ban. Úsalo bajo tu propio riesgo.

1. **Crear override para ACE:**

```bash
cd ~/.var/app/com.valvesoftware.Steam/.local/share/Steam/steamapps/common/"Delta Force"/Game/DeltaForce/Binaries/Win64/

# Renombrar DLLs de ACE
mv ACE-BASE.sys ACE-BASE.sys.bak
mv ACE-CORE.sys ACE-CORE.sys.bak
```

2. **Opciones de lanzamiento en Steam:**

```bash
WINEDLLOVERRIDES="ACE-BASE=;ACE-CORE=;SGuard64=;TenProtect=" %command%
```

---

### 🎯 Opción 3: Dual Boot con Windows (Recomendado si nada funciona)

Si ACE es obligatorio y no hay forma de evitarlo:

1. **Instalar Windows 11 en dual boot**
2. **Jugar Delta Force desde Windows**
3. **Usar Linux para todo lo demás**

---

## 🔍 Diagnóstico Actual

Según tus logs:

```
✅ Delta Force instalado: ~/.var/app/com.valvesoftware.Steam/.local/share/Steam/steamapps/common/Delta Force
✅ Wine Prefix: compatdata/2507950/pfx
✅ Proton funcionando
❌ ACE bloqueando el inicio
❌ Drivers de kernel no cargados en Wine
```

---

## 🧪 Test Rápido: Ver si EAC funciona sin ACE

```bash
# 1. Ir al directorio de Delta Force
cd ~/.var/app/com.valvesoftware.Steam/.local/share/Steam/steamapps/common/"Delta Force"

# 2. Buscar archivos EAC
find . -name "*EasyAnti*" -o -name "*eac*"

# 3. Si existe, intentar lanzar SOLO con EAC
```

**Opciones de lanzamiento para test:**

```bash
# Test 1: Deshabilitar ACE, habilitar EAC
PROTON_USE_EAC_WORKAROUND=1 PROTON_EAC_RUNTIME=1 WINEDLLOVERRIDES="ACE-BASE=;ACE-CORE=" %command%

# Test 2: Modo offline (sin anti-cheat)
PROTON_USE_EAC_WORKAROUND=0 WINEDLLOVERRIDES="ACE-BASE=;ACE-CORE=;SGuard64=" %command% -offline

# Test 3: Forzar DirectX 11 (más compatible)
PROTON_USE_D3D11=1 PROTON_NO_ESYNC=1 %command%
```

---

## 📊 Verificar si EAC está disponible

```bash
# Ver si Delta Force usa EAC
grep -r "EasyAntiCheat" ~/.var/app/com.valvesoftware.Steam/.local/share/Steam/steamapps/common/"Delta Force"/

# Ver servicios anti-cheat instalados
ls -la ~/.var/app/com.valvesoftware.Steam/.local/share/Steam/steamapps/compatdata/2507950/pfx/drive_c/windows/system32/drivers/ | grep -i "ace\|eac\|guard"
```

---

## 🎮 Configuración Steam Recomendada

**Propiedades → Compatibilidad:**
- ✅ Forzar herramienta: **GE-Proton 9-20** o superior
- ✅ Habilitar Steam Play

**Opciones de lanzamiento:**

```bash
PROTON_USE_EAC_WORKAROUND=1 PROTON_LOG=1 WINEDLLOVERRIDES="ACE-BASE=;ACE-CORE=" DXVK_ASYNC=1 RADV_PERFTEST=aco,sam %command%
```

---

## 🔧 Script de Limpieza ACE

Crea este script para limpiar ACE:

```bash
#!/bin/bash
# clean_ace.sh

PFXDIR="$HOME/.var/app/com.valvesoftware.Steam/.local/share/Steam/steamapps/compatdata/2507950/pfx"

echo "🧹 Limpiando drivers ACE del Wine Prefix..."

# Eliminar drivers ACE
rm -f "$PFXDIR/drive_c/windows/system32/drivers/ACE-"*.sys
rm -f "$PFXDIR/drive_c/windows/syswow64/drivers/ACE-"*.sys

# Eliminar servicios del registro
WINEPREFIX="$PFXDIR" wine reg delete "HKLM\\System\\CurrentControlSet\\Services\\ACE-BASE" /f 2>/dev/null
WINEPREFIX="$PFXDIR" wine reg delete "HKLM\\System\\CurrentControlSet\\Services\\ACE-CORE" /f 2>/dev/null
WINEPREFIX="$PFXDIR" wine reg delete "HKLM\\System\\CurrentControlSet\\Services\\ACE-BOOT" /f 2>/dev/null

echo "✅ ACE limpiado. Intenta lanzar el juego ahora."
```

**Ejecutar:**

```bash
chmod +x clean_ace.sh
./clean_ace.sh
```

---

## 📝 Logs para Debugging

```bash
# Ver logs en tiempo real
tail -f ~/.var/app/com.valvesoftware.Steam/.local/share/Steam/steamapps/common/"Delta Force"/Game/DeltaForce/Saved/Logs/DeltaForce.log

# Ver logs de Proton
tail -f /tmp/proton_$USER/*.log

# Ver logs de Steam
journalctl --user -f | grep -i "delta\|proton\|steam"
```

---

## ⚠️ Realidad sobre ACE en Linux

**ACE (AntiCheatExpert) de Tencent NO es compatible con Linux** porque:

1. Requiere drivers a nivel kernel de Windows
2. Wine/Proton no puede cargar drivers `.sys` reales
3. No existe versión nativa de ACE para Linux
4. Intentar bypassearlo puede resultar en ban

**Opciones realistas:**

1. ✅ Jugar si el juego acepta **solo EasyAntiCheat** (que SÍ funciona en Linux)
2. ✅ Jugar en **modo offline** sin anti-cheat
3. ✅ Usar **Windows en dual boot** para juegos con ACE
4. ❌ **No hay forma legítima** de usar ACE en Linux actualmente

---

## 🎯 Próximos Pasos

1. **Verificar si Delta Force funciona con solo EAC:**
   ```bash
   PROTON_USE_EAC_WORKAROUND=1 WINEDLLOVERRIDES="ACE-BASE=;ACE-CORE=" %command%
   ```

2. **Si no funciona, probar modo offline:**
   ```bash
   %command% -offline
   ```

3. **Si nada funciona, considerar dual boot con Windows**

---

## 💡 Alternativa: Proton Experimental

A veces la versión experimental tiene mejor soporte:

```bash
# En Steam → Delta Force → Propiedades → Compatibilidad
# Seleccionar: Proton Experimental

# Opciones de lanzamiento:
PROTON_LOG=1 PROTON_USE_EAC_WORKAROUND=1 %command%
```

---

**La verdad dura:** Si Delta Force **requiere ACE obligatoriamente**, no podrás jugarlo en Linux de forma legítima. ACE es incompatible con Wine/Proton por diseño.

Tu mejor opción es verificar si el juego acepta solo EasyAntiCheat o permite jugar offline. 🎮
