# ✅ Delta Force desde Steam con GE-Proton 10-25 + ACE

## 🎯 Todo está listo

He configurado todo automáticamente para que Delta Force funcione desde Steam con ACE.

---

## 📝 Configuración de Steam (2 pasos)

### Paso 1: Abrir propiedades de Delta Force

1. Abre **Steam**
2. Ve a **Biblioteca**
3. Click **derecho** en **Delta Force**
4. Selecciona **Propiedades**

### Paso 2: Configurar compatibilidad y launch options

**En la pestaña COMPATIBILIDAD:**

- ✅ Marca: **"Forzar el uso de una herramienta de compatibilidad específica de Steam Play"**
- ✅ Selecciona: **GE-Proton10-25** (el que ya tienes)

**En OPCIONES DE LANZAMIENTO (Launch Options), pega esto:**

```
~/.var/app/com.valvesoftware.Steam/data/scripts/steam_deltaforce_wrapper.sh %command%
```

**IMPORTANTE:** 
- Incluye `%command%` al final
- Usa la ruta completa `~/.var/app/...` (no `/home/mrvanguardia/Documentos/...`)

---

## 🚀 Ejecutar Delta Force

Simplemente:

1. Ve a tu **Biblioteca** en Steam
2. Click en **Delta Force**
3. Click **JUGAR**

Verás la configuración ACE en pantalla antes de que el juego inicie.

---

## 🔧 Optimizaciones Extra (Opcional)

Para **mejor rendimiento**, usa esto en Launch Options en lugar del comando básico:

```
PROTON_ENABLE_NVAPI=1 DXVK_ASYNC=1 ~/.var/app/com.valvesoftware.Steam/data/scripts/steam_deltaforce_wrapper.sh %command%
```

Esto habilita:
- **NVAPI**: Mejor rendimiento en tarjetas NVIDIA
- **DXVK_ASYNC**: Compilación asíncrona de shaders (menos stuttering)

---

## 📊 ¿Qué se instaló?

```
✓ Wrapper script copiado al sandbox de Steam
✓ 25 archivos ACE disponibles (drivers, DLLs, ejecutables)
✓ Rutas actualizadas para Steam Flatpak
✓ Sistema de logs habilitado
```

**Ubicaciones:**
- **Wrapper:** `~/.var/app/com.valvesoftware.Steam/data/scripts/steam_deltaforce_wrapper.sh`
- **Drivers ACE:** `~/.var/app/com.valvesoftware.Steam/data/Win64/AntiCheatExpert/`
- **Logs:** `~/.cache/kernelbridge/steam_wrapper.log`

---

## 🔍 Verificación

Después de intentar ejecutar Delta Force, revisa los logs:

```bash
cat ~/.cache/kernelbridge/steam_wrapper.log
```

Deberías ver:
- ✅ Steam Flatpak detectado
- ✅ Delta Force encontrado
- ✅ Wine Prefix configurado
- ✅ Drivers ACE copiados
- ✅ Juego ejecutado

---

## ⚠️ Notas Importantes

### Modo de Juego

- **✅ Campaña/Offline:** Debería funcionar perfectamente
- **⚠️ Multijugador:** Puede funcionar, pero hay riesgo de detección ACE
- **❌ Competitivo:** NO recomendado (alto riesgo de baneo)

### Primera Ejecución

La **primera vez** que ejecutes Delta Force con el wrapper:
- Puede tardar más (GE-Proton crea el Wine Prefix)
- Verás shaders compilándose
- El juego puede tardar en iniciar

**Esto es normal.** Las siguientes ejecuciones serán más rápidas.

### Si no funciona

1. Verifica que pegaste el comando EXACTO en Launch Options
2. Verifica que GE-Proton10-25 está seleccionado en Compatibilidad
3. Revisa los logs: `cat ~/.cache/kernelbridge/steam_wrapper.log`
4. Lee la guía completa: `cat ~/Documentos/PROYECTOS/kernelBridge/SOLUCION_STEAM_NO_INICIA.md`

---

## 🎮 Resumen Ultra Rápido

```bash
# Ya ejecutaste esto:
./fix_steam_flatpak.sh  ✅

# Ahora en Steam:
1. Delta Force → Propiedades
2. Compatibilidad → GE-Proton10-25
3. Launch Options → ~/.var/app/com.valvesoftware.Steam/data/scripts/steam_deltaforce_wrapper.sh %command%
4. JUGAR
```

---

## 🆘 Alternativa (si Steam falla)

Si prefieres **no usar Steam Launch Options**, simplemente usa:

```bash
cd ~/Documentos/PROYECTOS/kernelBridge
./quick_start_deltaforce.sh
```

O instala los alias:

```bash
./install_aliases.sh
source ~/.bashrc
deltaforce
```

---

¡Listo! Ahora configura Steam y prueba Delta Force con ACE. 🎮🐧
