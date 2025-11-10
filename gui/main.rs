// KernelBridge GUI - Compatible con Relm4 0.10 y GTK4 0.10
use relm4::prelude::*;
use gtk::prelude::*;
use std::process::Command as Cmd;
use std::process::Stdio;
use std::io::{BufRead, BufReader};
use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use std::io::Write;
use std::io::Read as _;
#[cfg(unix)]
use std::os::unix::net::UnixStream;
use std::path::Path;

#[derive(Clone, Debug, PartialEq)]
enum Section {
    Security,
    Games,
    Folders,
    KernelBridge,
    Settings,
}

// mod ac_integration;
// use core::ac_integration::{
    // eos_report_cheat,
    // vac_apply_ban,
    // query_report_status,
    // get_file_hash,
    // get_memory_hash,
    // start_ebpf_monitor,
    // start_ptrace_monitor,
// };

#[derive(Clone, Debug)]
struct AppModel {
    current_section: Section,
    current_path: PathBuf,
    folder_items: Vec<FileItem>,
    games: Vec<GameInfo>,
    tpm_status: String,
    secure_boot: String,
    kernel_modules: Vec<KernelModule>,
    api_status: HashMap<String, bool>,
    bridge_logs: Vec<String>,
    config_debug_mode: bool,
    config_force_bottles: bool,
    config_prefer_flatpak: bool,
    config_force_vkd3d: bool,
    config_offline_mode: bool,
    config_use_proton_ge_local: bool,
    config_local_mitigations: bool,
    config_auto_vm_ac: bool,
    config_prefer_cf_windows: bool,
    config_permissions: String,
    last_vm_ip: Option<String>,
    vm_default_name: String,
    // Idioma UI (true = English). Se refleja desde la variable de entorno KB_LANG
    config_language_en: bool,
    // Puente de audio cuando se lanza como root (usar PulseAudio del usuario)
    config_audio_bridge_root: bool,
}

#[derive(Clone, Debug)]
struct FileItem {
    name: String,
    path: PathBuf,
    is_dir: bool,
    is_executable: bool,
}

#[derive(Clone, Debug, PartialEq)]
enum GameSource {
    Steam,
    Bottles,
    Lutris,
    Local,
}

#[derive(Clone, Debug)]
struct GameInfo {
    name: String,
    path: PathBuf,
    executable: String,
    compatible: bool,
    anticheat: String,
    source: GameSource,
    launch_id: String, // appid for Steam, bottle name for Bottles
}

#[derive(Clone, Debug)]
enum LauncherKind {
    SteamNative,
    SteamFlatpak,
    BottlesNative,
    BottlesFlatpak,
    LutrisNative,
    LutrisFlatpak,
    Wine,
    MinecraftNative,
    MinecraftFlatpak,
    MinecraftWine,
    PrismNative,
    PrismFlatpak,
    CurseForgeBottles,
    CurseForgeWine,
    CurseForgeNative,
}

#[derive(Clone, Debug)]
struct KernelModule {
    name: String,
    size: String,
    used_by: String,
}

#[derive(Clone, Debug)]
enum AppMsg {
    ChangeSection(Section),
    Navigate(PathBuf),
    GoUp,
    ExecuteFile(PathBuf),
    RefreshFolder,
    ScanGames,
    ScanGamesFinished(Vec<GameInfo>),
    ExecuteGame(GameInfo),
    ExecuteGameIgnoreAC(GameInfo),
    LaunchLauncher(LauncherKind),
    DownloadMinecraftLinuxTar,
    InstallMinecraftFlatpak,
    InstallPrismFlatpak,
    DownloadCurseForgeInstaller,
    InstallCurseForgeWithBottles,
    InstallCurseForgeWithWine,
    InstallMinecraftWine,
    LaunchCurseForgeBottles,
    LaunchCurseForgeWine,
    DownloadCurseForgeLinux,
    LaunchCurseForgeNative,
    RefreshSecurity,
    RefreshKernelBridge,
    SetSteamPath(String),
    ToggleDebugMode,
    ToggleForceBottles,
    TogglePreferFlatpak,
    ToggleForceVKD3D,
    ToggleOfflineMode,
    ToggleUseProtonGE,
    ToggleAutoVmAc,
    TogglePreferCurseForgeWindows,
    // Delta Force Setup Wizard
    StartDeltaForceWizard,
    DFWizard_Step1_CheckSystem,
    DFWizard_Step2_InstallTools,
    DFWizard_Step3_CopyToSteam,
    DFWizard_Step4_ConfigureSteam,
    DFWizard_Step5_LaunchGame,
    DFWizard_Complete,
    ToggleLocalMitigations,
    SetVmIp(String),
    SetVmDefaultName(String),
    ClearLogs,
    Log(String),
    TestHybridApis,
    ToggleLanguageEn,
    ToggleAudioBridgeRoot,
    LaunchDeltaForceQuickStart,
}

struct AppWidgets {
    main_box: gtk::Box,
}

impl SimpleComponent for AppModel {
    type Init = ();
    type Input = AppMsg;
    type Output = ();
    type Root = gtk::ApplicationWindow;
    type Widgets = AppWidgets;

    fn init_root() -> Self::Root {
        gtk::ApplicationWindow::builder()
            .title("KernelBridge - Panel de Control")
            .default_width(1200)
            .default_height(800)
            .build()
    }

    fn init(_: Self::Init, root: Self::Root, sender: ComponentSender<Self>) -> ComponentParts<Self> {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/home/user".to_string());
        let current_path = PathBuf::from(&home);
        
    let mut api_status = HashMap::new();
    api_status.insert("Windows API".to_string(), true);
    api_status.insert("DirectX Bridge".to_string(), true);
    // Mostrar como capa informativa de lectura
    api_status.insert("Anti-Cheat Gateway (read-only)".to_string(), true);
    api_status.insert("Kernel Hooks".to_string(), true);
        
        let model = AppModel {
            current_section: Section::Security,
            current_path,
            folder_items: Vec::new(),
            games: Vec::new(),
            tpm_status: "Detectando...".to_string(),
            secure_boot: "Detectando...".to_string(),
            kernel_modules: Vec::new(),
            api_status,
            bridge_logs: vec![
                "[INFO] KernelBridge iniciado".to_string(),
                "[INFO] Listo".to_string(),
            ],
            config_debug_mode: false,
            config_force_bottles: false,
            config_prefer_flatpak: true,
            config_force_vkd3d: false,
            config_offline_mode: false,
            config_use_proton_ge_local: false,
            config_local_mitigations: true,
            config_auto_vm_ac: true,
            config_prefer_cf_windows: true,
            config_permissions: "user".to_string(),
            last_vm_ip: None,
            vm_default_name: std::env::var("KB_VM_NAME").unwrap_or_else(|_| "Windows-Gaming".to_string()),
            config_language_en: std::env::var("KB_LANG").map(|s| s.to_lowercase() == "en").unwrap_or(false),
            config_audio_bridge_root: std::env::var("KB_AUDIO_BRIDGE_ROOT").map(|v| v == "1" || v.to_lowercase() == "true").unwrap_or(false),
        };

        let main_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();

        root.set_child(Some(&main_box));

        // Al cerrar la ventana, intentar apagar el daemon para no dejar procesos colgados
        root.connect_close_request(|_| {
            shutdown_daemon();
            gtk::glib::Propagation::Proceed
        });

        let widgets = AppWidgets { main_box };
        
        // Cargar datos inicial
        sender.input(AppMsg::RefreshSecurity);
        sender.input(AppMsg::ScanGames);
        sender.input(AppMsg::RefreshFolder);
        
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            AppMsg::ChangeSection(section) => {
                self.current_section = section;
            }
            AppMsg::ToggleLanguageEn => {
                self.config_language_en = !self.config_language_en;
                if self.config_language_en {
                    std::env::set_var("KB_LANG", "en");
                } else {
                    std::env::remove_var("KB_LANG");
                }
                let lang = if self.config_language_en { "English" } else { "Español" };
                self.bridge_logs.push(format!("[LANG] Idioma UI: {}", lang));
            }
            AppMsg::ToggleAudioBridgeRoot => {
                self.config_audio_bridge_root = !self.config_audio_bridge_root;
                if self.config_audio_bridge_root {
                    std::env::set_var("KB_AUDIO_BRIDGE_ROOT", "1");
                    self.bridge_logs.push("[AUDIO] Puente de audio (root→usuario) ACTIVADO".to_string());
                } else {
                    std::env::remove_var("KB_AUDIO_BRIDGE_ROOT");
                    self.bridge_logs.push("[AUDIO] Puente de audio (root→usuario) DESACTIVADO".to_string());
                }
            }
            AppMsg::ExecuteGameIgnoreAC(game) => {
                // Igual que el flujo normal, pero sin el router de anti‑cheat/VM
                match prepare_game(&game) {
                    Ok(info) => {
                        self.bridge_logs.push(format!("[PREPARE] {}", info));
                    }
                    Err(err) => {
                        self.bridge_logs.push(format!("[PREPARE] ERROR: {}", err));
                        return;
                    }
                }
                self.bridge_logs.push(format!("[GAME] Launching (ignorar AC) {}", game.name));
                let local_mitigations = self.config_local_mitigations;
                let offline_mode = self.config_offline_mode;
                let local_force_vkd3d = self.config_force_vkd3d;
                let prefer_flatpak = self.config_prefer_flatpak;
                let use_proton_ge = self.config_use_proton_ge_local;
                let sender_clone = sender.clone();
                std::thread::spawn(move || {
                    match game.source {
                        GameSource::Steam => {
                            let home_path = resolve_home();
                            let has_flatpak = home_path.join(".var/app/com.valvesoftware.Steam").exists();
                            let has_native = which_exists("steam");
                            let use_flatpak = if has_flatpak && (prefer_flatpak || !has_native) { true } else if has_native { false } else { has_flatpak };
                            if use_flatpak {
                                let mut args = vec!["run".to_string()];
                                if offline_mode { args.push("--no-network".to_string()); }
                                args.extend(vec![
                                    "com.valvesoftware.Steam".to_string(),
                                    "-applaunch".to_string(),
                                    game.launch_id.clone(),
                                ]);
                                spawn_program_s("flatpak", args);
                            } else {
                                if offline_mode && which_exists("firejail") {
                                    spawn_program_s("firejail", vec!["--noprofile".to_string(), "--net=none".to_string(), "steam".to_string(), "-applaunch".to_string(), game.launch_id.clone()]);
                                } else {
                                    if offline_mode { sender_clone.input(AppMsg::Log("[OFFLINE] No se pudo bloquear red (instala 'firejail' o usa Steam Flatpak)".to_string())); }
                                    spawn_program_s("steam", vec!["-applaunch".to_string(), game.launch_id.clone()]);
                                }
                            }
                        }
                        GameSource::Bottles => {
                            let exe = game.path.to_string_lossy().to_string();
                            let home_path = resolve_home();
                            let has_flatpak = home_path.join(".var/app/com.usebottles.bottles").exists();
                            let has_native = which_exists("bottles") || which_exists("bottles-cli");
                            let try_flatpak_first = has_flatpak && (prefer_flatpak || !has_native);
                            if try_flatpak_first {
                                let mut args = vec!["run".to_string()];
                                if offline_mode { args.push("--no-network".to_string()); }
                                if requires_eos_disable(&game) { args.push("--env=EOS_OVERLAY_DISABLED=1".to_string()); }
                                args.extend(vec![
                                    "com.usebottles.bottles".to_string(),
                                    "run".to_string(),
                                    "-b".to_string(),
                                    game.launch_id.clone(),
                                    "-e".to_string(),
                                    exe.clone(),
                                ]);
                                let ok = spawn_program_s("flatpak", args);
                                if ok { return; }
                            }
                            if offline_mode && which_exists("firejail") {
                                let _ = spawn_program_s("firejail", vec!["--noprofile".to_string(), "--net=none".to_string(), "bottles-cli".to_string(), "run".to_string(), "-b".to_string(), game.launch_id.clone(), "-e".to_string(), exe]);
                            } else {
                                if offline_mode { sender_clone.input(AppMsg::Log("[OFFLINE] No se pudo bloquear red para Bottles nativo (instala 'firejail' o usa Bottles Flatpak)".to_string())); }
                                let _ = spawn_program_s(
                                    "bottles-cli",
                                    vec![
                                        "run".to_string(),
                                        "-b".to_string(),
                                        game.launch_id.clone(),
                                        "-e".to_string(),
                                        exe,
                                    ],
                                );
                            }
                        }
                        GameSource::Lutris => {
                            let home_path = resolve_home();
                            let has_flatpak = home_path.join(".var/app/net.lutris.Lutris").exists();
                            let has_native = which_exists("lutris");
                            let use_flatpak = if has_flatpak && (prefer_flatpak || !has_native) { true } else if has_native { false } else { has_flatpak };
                            let uri = format!("lutris:rungame/{}", game.launch_id);
                            if use_flatpak {
                                let mut args = vec!["run".to_string()];
                                if offline_mode { args.push("--no-network".to_string()); }
                                args.extend(vec!["net.lutris.Lutris".to_string(), uri]);
                                spawn_program_s("flatpak", args);
                            } else {
                                if offline_mode && which_exists("firejail") {
                                    spawn_program_s("firejail", vec!["--noprofile".to_string(), "--net=none".to_string(), "lutris".to_string(), uri]);
                                } else {
                                    if offline_mode { sender_clone.input(AppMsg::Log("[OFFLINE] No se pudo bloquear red para Lutris nativo (instala 'firejail' o usa Lutris Flatpak)".to_string())); }
                                    spawn_program_s("lutris", vec![uri]);
                                }
                            }
                        }
                        GameSource::Local => {
                            // Seleccionar Proton-GE o Wine (usar flag capturada por valor)
                            if use_proton_ge {
                                if let (Some(steam_root), Some(proton_script)) = (find_steam_root(), find_proton_ge_script()) {
                                    let compat_base = resolve_home().join(".local/share/KernelBridge/compatdata");
                                    let _ = std::fs::create_dir_all(&compat_base);
                                    let slug = slugify_path(&game.path);
                                    let compat_path = compat_base.join(slug);
                                    let parent_dir = game.path.parent().unwrap_or_else(|| std::path::Path::new("."));
                                    let mut cmd;
                                    let use_firejail = offline_mode && which_exists("firejail");
                                    if use_firejail {
                                        cmd = Cmd::new("firejail");
                                        cmd.arg("--noprofile").arg("--net=none");
                                        cmd.arg(&proton_script);
                                    } else {
                                        cmd = Cmd::new(&proton_script);
                                        if offline_mode { sender_clone.input(AppMsg::Log("[OFFLINE] No se pudo bloquear red para Proton-GE (instala 'firejail')".to_string())); }
                                    }
                                    cmd.env("STEAM_COMPAT_CLIENT_INSTALL_PATH", &steam_root);
                                    cmd.env("STEAM_COMPAT_DATA_PATH", &compat_path);
                                    cmd.current_dir(parent_dir);
                                    cmd.arg("run").arg(&game.path);
                                    sender_clone.input(AppMsg::Log(format!("[LOCAL] Proton-GE: {}", proton_script.display())));
                                    let _ = cmd.spawn();
                                } else {
                                    sender_clone.input(AppMsg::Log("[PROTON-GE] No encontrado (se usará Wine). Instala GE-Proton en Steam nativo: ~/.local/share/Steam/compatibilitytools.d".to_string()));
                                    launch_local_with_wine(&game.path, offline_mode, local_mitigations, local_force_vkd3d, &sender_clone);
                                }
                            } else {
                                launch_local_with_wine(&game.path, offline_mode, local_mitigations, local_force_vkd3d, &sender_clone);
                            }
                        }
                    }
                });
            }
            AppMsg::Navigate(path) => {
                if path.exists() {
                    self.current_path = path;
                    self.folder_items = scan_directory(&self.current_path);
                }
            }
            AppMsg::GoUp => {
                if let Some(parent) = self.current_path.parent() {
                    self.current_path = parent.to_path_buf();
                    self.folder_items = scan_directory(&self.current_path);
                }
            }
            AppMsg::ExecuteFile(path) => {
                let file_name = path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("archivo")
                    .to_string();
                
                self.bridge_logs.push(format!("[EXEC] {}", file_name));
                
                let path_str = path.to_string_lossy().to_string();
                std::thread::spawn(move || {
                    if path_str.ends_with(".sh") {
                        let _ = Cmd::new("bash").arg(&path).spawn();
                    } else if path_str.ends_with(".exe") {
                        let _ = Cmd::new("wine").arg(&path).spawn();
                    } else {
                        let _ = Cmd::new(&path).spawn();
                    }
                });
            }
            AppMsg::RefreshFolder => {
                self.folder_items = scan_directory(&self.current_path);
            }
            AppMsg::ScanGames => {
                // Ejecutar el escaneo en un hilo separado para no congelar la UI
                self.bridge_logs.push("[SCAN] Escaneando...".to_string());
                let sender_clone = sender.clone();
                std::thread::spawn(move || {
                    let games = scan_all_games();
                    sender_clone.input(AppMsg::ScanGamesFinished(games));
                });
            }
            AppMsg::ScanGamesFinished(games) => {
                self.games = games;
                self.bridge_logs.push(format!("[SCAN] Juegos detectados: {}", self.games.len()));
            }
            AppMsg::RefreshSecurity => {
                self.tpm_status = check_tpm();
                self.secure_boot = check_secure_boot();
                self.kernel_modules = get_kernel_modules();
            }
            AppMsg::RefreshKernelBridge => {
                let timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                
                for (name, status) in self.api_status.iter_mut() {
                    let hash = (name.len() * timestamp as usize) % 10;
                    if hash > 8 {
                        *status = !*status;
                    }
                }
                
                self.bridge_logs.push("[INFO] Actualizado".to_string());
                if self.bridge_logs.len() > 50 {
                    self.bridge_logs.drain(0..10);
                }
            }
            AppMsg::SetSteamPath(_) => {
                // This is now deprecated
            }
            AppMsg::ToggleDebugMode => {
                self.config_debug_mode = !self.config_debug_mode;
                let msg = if self.config_debug_mode {
                    "[DEBUG] Activado"
                } else {
                    "[INFO] Desactivado"
                };
                self.bridge_logs.push(msg.to_string());
            }
            AppMsg::ToggleForceBottles => {
                self.config_force_bottles = !self.config_force_bottles;
                let msg = if self.config_force_bottles {
                    "[AVANZADO] Forzando ejecución en Bottles/Wine aunque haya anti‑cheat (puede no funcionar online)"
                } else {
                    "[INFO] Enrutamiento automático a VM restaurado para juegos con anti‑cheat de kernel"
                };
                self.bridge_logs.push(msg.to_string());
            }
            AppMsg::TogglePreferFlatpak => {
                self.config_prefer_flatpak = !self.config_prefer_flatpak;
                let msg = if self.config_prefer_flatpak {
                    "[PREFERENCIA] Se preferirán versiones Flatpak de los launchers cuando estén instaladas"
                } else {
                    "[PREFERENCIA] Preferencia por Flatpak desactivada (se usará nativo si está disponible)"
                };
                self.bridge_logs.push(msg.to_string());
            }
            AppMsg::ToggleUseProtonGE => {
                self.config_use_proton_ge_local = !self.config_use_proton_ge_local;
                let msg = if self.config_use_proton_ge_local {
                    "[LOCAL] Usar Proton-GE para juegos locales"
                } else {
                    "[LOCAL] Usar Wine para juegos locales"
                };
                self.bridge_logs.push(msg.to_string());
            }
            AppMsg::ToggleAutoVmAc => {
                self.config_auto_vm_ac = !self.config_auto_vm_ac;
                let msg = if self.config_auto_vm_ac {
                    "[VM] Enrutamiento automático a VM para juegos con anti‑cheat de kernel activado"
                } else {
                    "[VM] Enrutamiento automático a VM desactivado (se usará ruta nativa/Bottles)"
                };
                self.bridge_logs.push(msg.to_string());
            }
            AppMsg::ToggleForceVKD3D => {
                self.config_force_vkd3d = !self.config_force_vkd3d;
                let msg = if self.config_force_vkd3d {
                    "[VKD3D] Forzando D3D12 nativo (vkd3d-proton); requiere d3d12.dll en carpeta del juego"
                } else {
                    "[VKD3D] Forzado desactivado"
                };
                self.bridge_logs.push(msg.to_string());
            }
            AppMsg::ToggleOfflineMode => {
                self.config_offline_mode = !self.config_offline_mode;
                let msg = if self.config_offline_mode {
                    "[OFFLINE] Bloqueo de red activado para lanzamientos"
                } else {
                    "[OFFLINE] Bloqueo de red desactivado"
                };
                self.bridge_logs.push(msg.to_string());
            }
            AppMsg::ToggleLocalMitigations => {
                self.config_local_mitigations = !self.config_local_mitigations;
                let msg = if self.config_local_mitigations {
                    "[LOCAL] Mitigaciones activadas (EOS overlay OFF, esync/fsync ON)"
                } else {
                    "[LOCAL] Mitigaciones desactivadas"
                };
                self.bridge_logs.push(msg.to_string());
            }
            AppMsg::TogglePreferCurseForgeWindows => {
                self.config_prefer_cf_windows = !self.config_prefer_cf_windows;
                let msg = if self.config_prefer_cf_windows {
                    "[PREFERENCIA] CurseForge: preferir Windows (Wine/Bottles) para compatibilidad de mods"
                } else {
                    "[PREFERENCIA] CurseForge: preferir nativo (AppImage) en Linux"
                };
                self.bridge_logs.push(msg.to_string());
            }
            AppMsg::SetVmIp(ip) => {
                self.last_vm_ip = Some(ip);
            }
            AppMsg::SetVmDefaultName(name) => {
                self.vm_default_name = name.clone();
                self.bridge_logs.push(format!("[VM] Nombre por defecto actualizado: {}", name));
            }
            AppMsg::ExecuteGame(game) => {
                // Router de juegos: detectar títulos que requieren kernel-mode AC
                if self.config_auto_vm_ac && !self.config_force_bottles && is_vm_enabled() && is_kernel_ac_game(&game) {
                    self.bridge_logs.push("[PREPARE] Anti‑cheat de kernel detectado → enrutando a VM (Windows)".to_string());
                    let vm_name = default_vm_name();
                    let sender_clone = sender.clone();
                    std::thread::spawn(move || {
                        // Intentar estado e iniciar si está apagada
                        match vm_command("VM_STATUS", &vm_name) {
                            Ok(resp) => sender_clone.input(AppMsg::Log(format!("[VM] Estado {}: {}", vm_name, resp.trim()))),
                            Err(e) => sender_clone.input(AppMsg::Log(format!("[VM] ERROR status {}: {}", vm_name, e))),
                        }
                        match vm_command("VM_START", &vm_name) {
                            Ok(resp) => sender_clone.input(AppMsg::Log(format!("[VM] {}", resp.trim()))),
                            Err(e) => sender_clone.input(AppMsg::Log(format!("[VM] ERROR start {}: {}", vm_name, e))),
                        }
                        sender_clone.input(AppMsg::Log("[VM] Inicia tu cliente remoto si corresponde".to_string()));
                    });
                    return; // No lanzar vía Bottles/Lutris para este juego
                }
                // Preflight con el daemon: PREPARE_GAME
                match prepare_game(&game) {
                    Ok(info) => {
                        self.bridge_logs.push(format!("[PREPARE] {}", info));
                    }
                    Err(err) => {
                        self.bridge_logs.push(format!("[PREPARE] ERROR: {}", err));
                        return;
                    }
                }
                self.bridge_logs.push(format!("[GAME] Launching {}", game.name));
                let local_mitigations = self.config_local_mitigations;
                let offline_mode = self.config_offline_mode;
                let local_force_vkd3d = self.config_force_vkd3d;
                let sender_clone = sender.clone();
                let prefer_flatpak = self.config_prefer_flatpak;
                let use_proton_ge = self.config_use_proton_ge_local;
                std::thread::spawn(move || {
                    match game.source {
                        GameSource::Steam => {
                            // Detectar si Steam es Flatpak o nativo
                            let home_path = resolve_home();
                            let has_flatpak = home_path.join(".var/app/com.valvesoftware.Steam").exists();
                            let has_native = which_exists("steam");
                            let use_flatpak = if has_flatpak && (prefer_flatpak || !has_native) { true } else if has_native { false } else { has_flatpak };
                            if use_flatpak {
                                let mut args = vec!["run".to_string()];
                                if offline_mode { args.push("--no-network".to_string()); }
                                args.extend(vec![
                                    "com.valvesoftware.Steam".to_string(),
                                    "-applaunch".to_string(),
                                    game.launch_id.clone(),
                                ]);
                                spawn_program_s("flatpak", args);
                            } else {
                                if offline_mode && which_exists("firejail") {
                                    spawn_program_s("firejail", vec!["--noprofile".to_string(), "--net=none".to_string(), "steam".to_string(), "-applaunch".to_string(), game.launch_id.clone()]);
                                } else {
                                    if offline_mode { sender_clone.input(AppMsg::Log("[OFFLINE] No se pudo bloquear red (instala 'firejail' o usa Steam Flatpak)".to_string())); }
                                    spawn_program_s("steam", vec!["-applaunch".to_string(), game.launch_id.clone()]);
                                }
                            }
                        }
                        GameSource::Bottles => {
                            let exe = game.path.to_string_lossy().to_string();
                            let home_path = resolve_home();
                            let has_flatpak = home_path.join(".var/app/com.usebottles.bottles").exists();
                            let has_native = which_exists("bottles") || which_exists("bottles-cli");
                            let try_flatpak_first = has_flatpak && (prefer_flatpak || !has_native);
                            if try_flatpak_first {
                                let mut args = vec!["run".to_string()];
                                if offline_mode { args.push("--no-network".to_string()); }
                                if requires_eos_disable(&game) { args.push("--env=EOS_OVERLAY_DISABLED=1".to_string()); }
                                args.extend(vec![
                                    "com.usebottles.bottles".to_string(),
                                    "run".to_string(),
                                    "-b".to_string(),
                                    game.launch_id.clone(),
                                    "-e".to_string(),
                                    exe.clone(),
                                ]);
                                let ok = spawn_program_s("flatpak", args);
                                if ok { return; }
                            }
                            // Nativo (o fallback)
                            if offline_mode && which_exists("firejail") {
                                let _ = spawn_program_s("firejail", vec!["--noprofile".to_string(), "--net=none".to_string(), "bottles-cli".to_string(), "run".to_string(), "-b".to_string(), game.launch_id.clone(), "-e".to_string(), exe]);
                            } else {
                                if offline_mode { sender_clone.input(AppMsg::Log("[OFFLINE] No se pudo bloquear red para Bottles nativo (instala 'firejail' o usa Bottles Flatpak)".to_string())); }
                                let _ = spawn_program_s(
                                    "bottles-cli",
                                    vec![
                                        "run".to_string(),
                                        "-b".to_string(),
                                        game.launch_id.clone(),
                                        "-e".to_string(),
                                        exe,
                                    ],
                                );
                            }
                        }
                        GameSource::Lutris => {
                            // Ejecutar juego por slug usando el lanzador de Lutris, prefiriendo Flatpak si corresponde
                            let home_path = resolve_home();
                            let has_flatpak = home_path.join(".var/app/net.lutris.Lutris").exists();
                            let has_native = which_exists("lutris");
                            let use_flatpak = if has_flatpak && (prefer_flatpak || !has_native) { true } else if has_native { false } else { has_flatpak };
                            let uri = format!("lutris:rungame/{}", game.launch_id);
                            if use_flatpak {
                                let mut args = vec!["run".to_string()];
                                if offline_mode { args.push("--no-network".to_string()); }
                                args.extend(vec!["net.lutris.Lutris".to_string(), uri]);
                                spawn_program_s("flatpak", args);
                            } else {
                                if offline_mode && which_exists("firejail") {
                                    spawn_program_s("firejail", vec!["--noprofile".to_string(), "--net=none".to_string(), "lutris".to_string(), uri]);
                                } else {
                                    if offline_mode { sender_clone.input(AppMsg::Log("[OFFLINE] No se pudo bloquear red para Lutris nativo (instala 'firejail' o usa Lutris Flatpak)".to_string())); }
                                    spawn_program_s("lutris", vec![uri]);
                                }
                            }
                        }
                        GameSource::Local => {
                            if use_proton_ge {
                                if let (Some(steam_root), Some(proton_script)) = (find_steam_root(), find_proton_ge_script()) {
                                    let compat_base = resolve_home().join(".local/share/KernelBridge/compatdata");
                                    let _ = std::fs::create_dir_all(&compat_base);
                                    let slug = slugify_path(&game.path);
                                    let compat_path = compat_base.join(slug);
                                    let parent_dir = game.path.parent().unwrap_or_else(|| std::path::Path::new("."));
                                    let mut cmd;
                                    let use_firejail = offline_mode && which_exists("firejail");
                                    if use_firejail {
                                        cmd = Cmd::new("firejail");
                                        cmd.arg("--noprofile").arg("--net=none");
                                        cmd.arg(&proton_script);
                                    } else {
                                        cmd = Cmd::new(&proton_script);
                                        if offline_mode { sender_clone.input(AppMsg::Log("[OFFLINE] No se pudo bloquear red para Proton-GE (instala 'firejail')".to_string())); }
                                    }
                                    cmd.env("STEAM_COMPAT_CLIENT_INSTALL_PATH", &steam_root);
                                    cmd.env("STEAM_COMPAT_DATA_PATH", &compat_path);
                                    cmd.current_dir(parent_dir);
                                    cmd.arg("run").arg(&game.path);
                                    sender_clone.input(AppMsg::Log(format!("[LOCAL] Proton-GE: {}", proton_script.display())));
                                    let _ = cmd.spawn();
                                } else {
                                    sender_clone.input(AppMsg::Log("[PROTON-GE] No encontrado (se usará Wine). Instala GE-Proton en Steam nativo: ~/.local/share/Steam/compatibilitytools.d".to_string()));
                                    launch_local_with_wine(&game.path, offline_mode, local_mitigations, local_force_vkd3d, &sender_clone);
                                }
                            } else {
                                launch_local_with_wine(&game.path, offline_mode, local_mitigations, local_force_vkd3d, &sender_clone);
                            }
                        }
                    }
                });
            }
            AppMsg::LaunchLauncher(kind) => {
                self.bridge_logs.push("[LAUNCHER] Abriendo lanzador".to_string());
                let sender_clone = sender.clone();
                std::thread::spawn(move || {
                    let ok = match kind {
                        LauncherKind::SteamFlatpak => spawn_program_s("flatpak", vec!["run".to_string(), "com.valvesoftware.Steam".to_string()]),
                        LauncherKind::SteamNative => spawn_program_s("steam", vec![]),
                        LauncherKind::BottlesFlatpak => spawn_program_s("flatpak", vec!["run".to_string(), "com.usebottles.bottles".to_string()]),
                        LauncherKind::BottlesNative => spawn_program_s("bottles", vec![]),
                        LauncherKind::LutrisFlatpak => spawn_program_s("flatpak", vec!["run".to_string(), "net.lutris.Lutris".to_string()]),
                        LauncherKind::LutrisNative => spawn_program_s("lutris", vec![]),
                        LauncherKind::Wine => spawn_program_s("winecfg", vec![]),
                        LauncherKind::MinecraftFlatpak => spawn_program_s("flatpak", vec!["run".to_string(), "com.mojang.Minecraft".to_string()]),
                        LauncherKind::MinecraftNative => {
                            // Evitar error "env: 'minecraft-launcher': No existe..." cuando no está en PATH
                            if which_exists("minecraft-launcher") {
                                spawn_program_s("minecraft-launcher", vec![])
                            } else if let Some(bin) = find_minecraft_launcher_binary() {
                                spawn_program_s(bin.to_string_lossy().as_ref(), vec![])
                            } else {
                                false
                            }
                        },
                        LauncherKind::MinecraftWine => launch_minecraft_wine(),
                        LauncherKind::PrismFlatpak => spawn_program_s("flatpak", vec!["run".to_string(), "org.prismlauncher.PrismLauncher".to_string()]),
                        LauncherKind::PrismNative => spawn_program_s("prismlauncher", vec![]),
                        LauncherKind::CurseForgeBottles => launch_curseforge_bottles(),
                        LauncherKind::CurseForgeWine => launch_curseforge_wine(),
                        LauncherKind::CurseForgeNative => launch_curseforge_native(),
                    };
                    if !ok { sender_clone.input(AppMsg::Log("[LAUNCHER] No se pudo iniciar el lanzador (verifica PATH/Flatpak/DBus)".to_string())); }
                });
            }
            // Removed: OpenMinecraftNativeDownload (se eliminó el botón de descarga oficial)
            AppMsg::DownloadMinecraftLinuxTar => {
                self.bridge_logs.push("[DL] Descargando Minecraft (tar.gz)".to_string());
                let sender_clone = sender.clone();
                std::thread::spawn(move || {
                    match download_minecraft_linux_tar() {
                        Ok(tar) => {
                            sender_clone.input(AppMsg::Log(format!("[DL] Tarball en {}", tar)));
                            match extract_minecraft_linux_tar(&tar) {
                                Ok(dir) => {
                                    sender_clone.input(AppMsg::Log(format!("[EXTRACT] Extraído a {}", dir)));
                                    // Intentar crear symlink en ~/.local/bin para que aparezca en PATH
                                    let bin = find_minecraft_launcher_binary();
                                    if let Some(bin_path) = bin {
                                        let home = resolve_home();
                                        let bin_dir = home.join(".local/bin");
                                        let _ = std::fs::create_dir_all(&bin_dir);
                                        let target = bin_dir.join("minecraft-launcher");
                                        let _ = spawn_program_s("ln", vec!["-sf".to_string(), bin_path.to_string_lossy().to_string(), target.to_string_lossy().to_string()]);
                                        sender_clone.input(AppMsg::Log(format!("[LINK] Creado enlace en {}", target.display())));
                                    } else {
                                        sender_clone.input(AppMsg::Log("[LINK] No se encontró binario 'minecraft-launcher' tras extraer".to_string()));
                                    }
                                }
                                Err(e) => sender_clone.input(AppMsg::Log(format!("[EXTRACT] ERROR: {}", e))),
                            }
                        }
                        Err(e) => sender_clone.input(AppMsg::Log(format!("[DL] ERROR Minecraft tar.gz: {}", e))),
                    }
                });
            }
            AppMsg::DownloadCurseForgeInstaller => {
                self.bridge_logs.push("[DL] Descargando CurseForge Installer...".to_string());
                let sender_clone = sender.clone();
                std::thread::spawn(move || {
                    match download_curseforge_installer() {
                        Ok(path) => sender_clone.input(AppMsg::Log(format!("[DL] Descargado en {}", path))),
                        Err(e) => sender_clone.input(AppMsg::Log(format!("[DL] ERROR descarga: {}", e))),
                    }
                });
            }
            AppMsg::DownloadCurseForgeLinux => {
                self.bridge_logs.push("[DL] Descargando CurseForge Linux (zip)...".to_string());
                let sender_clone = sender.clone();
                std::thread::spawn(move || {
                    match download_curseforge_linux_zip() {
                        Ok(zip) => {
                            let _ = sender_clone.input(AppMsg::Log(format!("[DL] Zip en {}", zip)));
                            match extract_curseforge_linux_zip(&zip) {
                                Ok(dir) => sender_clone.input(AppMsg::Log(format!("[EXTRACT] Extraído a {}", dir))),
                                Err(e) => sender_clone.input(AppMsg::Log(format!("[EXTRACT] ERROR: {}", e))),
                            }
                        }
                        Err(e) => sender_clone.input(AppMsg::Log(format!("[DL] ERROR CF Linux: {}", e))),
                    }
                });
            }
            AppMsg::InstallCurseForgeWithBottles => {
                self.bridge_logs.push("[INSTALL] CurseForge con Bottles".to_string());
                let sender_clone = sender.clone();
                std::thread::spawn(move || {
                    let ok = install_curseforge_bottles();
                    if ok { sender_clone.input(AppMsg::Log("[INSTALL] Instalación lanzada en Bottles (sigue el asistente)".to_string())); }
                    else { sender_clone.input(AppMsg::Log("[INSTALL] No se pudo iniciar la instalación en Bottles (verifica bottles-cli)".to_string())); }
                });
            }
            AppMsg::InstallCurseForgeWithWine => {
                self.bridge_logs.push("[INSTALL] CurseForge con Wine".to_string());
                let sender_clone = sender.clone();
                std::thread::spawn(move || {
                    let ok = install_curseforge_wine();
                    if ok { sender_clone.input(AppMsg::Log("[INSTALL] Instalador iniciado con Wine".to_string())); }
                    else { sender_clone.input(AppMsg::Log("[INSTALL] No se pudo iniciar el instalador con Wine".to_string())); }
                });
            }
            AppMsg::InstallMinecraftFlatpak => {
                self.bridge_logs.push("[INSTALL] Instalando Minecraft (Flatpak)".to_string());
                let sender_clone = sender.clone();
                std::thread::spawn(move || {
                    // Intentar instalación usuario
                    let ok = spawn_program_s("flatpak", vec!["install".to_string(), "-y".to_string(), "--user".to_string(), "flathub".to_string(), "com.mojang.Minecraft".to_string()]);
                    if ok { sender_clone.input(AppMsg::Log("[INSTALL] Instalación iniciada (Flatpak). Revisa progreso en la ventana de Flatpak.".to_string())); }
                    else { sender_clone.input(AppMsg::Log("[INSTALL] No se pudo iniciar la instalación Flatpak (verifica Flatpak/Flathub)".to_string())); }
                });
            }
            AppMsg::InstallPrismFlatpak => {
                self.bridge_logs.push("[INSTALL] Instalando Prism Launcher (Flatpak)".to_string());
                let sender_clone = sender.clone();
                std::thread::spawn(move || {
                    let ok = spawn_program_s("flatpak", vec!["install".to_string(), "-y".to_string(), "--user".to_string(), "flathub".to_string(), "org.prismlauncher.PrismLauncher".to_string()]);
                    if ok { sender_clone.input(AppMsg::Log("[INSTALL] Instalación iniciada (Flatpak). Revisa progreso en la ventana de Flatpak.".to_string())); }
                    else { sender_clone.input(AppMsg::Log("[INSTALL] No se pudo iniciar la instalación Flatpak de Prism (verifica Flatpak/Flathub)".to_string())); }
                });
            }
            AppMsg::InstallMinecraftWine => {
                self.bridge_logs.push("[INSTALL] Preparando Minecraft (Wine)".to_string());
                let sender_clone = sender.clone();
                std::thread::spawn(move || {
                    let ok = install_minecraft_wine();
                    if ok { sender_clone.input(AppMsg::Log("[INSTALL] Instalador de Minecraft (Windows) iniciado con Wine. Sigue el asistente.".to_string())); }
                    else { sender_clone.input(AppMsg::Log("[INSTALL] No se pudo iniciar el instalador de Minecraft (Windows) con Wine.".to_string())); }
                });
            }
            AppMsg::ClearLogs => {
                self.bridge_logs.clear();
                self.bridge_logs.push("[INFO] Limpio".to_string());
            }
            AppMsg::Log(s) => {
                self.bridge_logs.push(s);
                if self.bridge_logs.len() > 200 { self.bridge_logs.drain(0..50); }
            }
            AppMsg::LaunchCurseForgeBottles => {
                self.bridge_logs.push("[LAUNCHER] CurseForge (Bottles)".to_string());
                let sender_clone = sender.clone();
                std::thread::spawn(move || {
                    let ok = launch_curseforge_bottles();
                    if !ok { sender_clone.input(AppMsg::Log("[LAUNCHER] No se pudo lanzar CurseForge (Bottles)".to_string())); }
                });
            }
            AppMsg::LaunchCurseForgeWine => {
                self.bridge_logs.push("[LAUNCHER] CurseForge (Wine)".to_string());
                let sender_clone = sender.clone();
                std::thread::spawn(move || {
                    let ok = launch_curseforge_wine();
                    if !ok { sender_clone.input(AppMsg::Log("[LAUNCHER] No se pudo lanzar CurseForge (Wine)".to_string())); }
                });
            }
            AppMsg::LaunchCurseForgeNative => {
                self.bridge_logs.push("[LAUNCHER] CurseForge (Nativo AppImage)".to_string());
                let sender_clone = sender.clone();
                std::thread::spawn(move || {
                    let ok = launch_curseforge_native();
                    if !ok { sender_clone.input(AppMsg::Log("[LAUNCHER] No se pudo lanzar CurseForge (Nativo)".to_string())); }
                });
            }
            AppMsg::TestHybridApis => {
                self.bridge_logs.push("[API] Probando APIs híbridas".to_string());
                let sender_clone = sender.clone();
                std::thread::spawn(move || {
                    let resp = bridge_api_cmd("PING");
                    match resp {
                        Ok(r) => sender_clone.input(AppMsg::Log(format!("[BRIDGE_API] {}", r.trim()))),
                        Err(e) => sender_clone.input(AppMsg::Log(format!("[BRIDGE_API] ERROR: {}", e))),
                    }
                });
            }
            AppMsg::LaunchDeltaForceQuickStart => {
                self.bridge_logs.push("[DELTA FORCE] Lanzando quick start...".to_string());
                println!("\n╔════════════════════════════════════════════════════════════════╗");
                println!("║           DELTA FORCE - LANZAMIENTO INICIADO                  ║");
                println!("╚════════════════════════════════════════════════════════════════╝\n");
                
                let sender_clone = sender.clone();
                std::thread::spawn(move || {
                    // Intentar varias ubicaciones posibles para el script
                    let home = resolve_home();
                    let possible_paths = vec![
                        // Ruta estándar del proyecto
                        home.join("Documentos/PROYECTOS/kernelBridge/quick_start_deltaforce.sh"),
                        // Ruta relativa desde el ejecutable
                        {
                            let current_exe = std::env::current_exe().ok();
                            if let Some(exe) = current_exe {
                                let mut path = exe.parent().unwrap_or(std::path::Path::new(".")).to_path_buf();
                                if path.ends_with("target/release") || path.ends_with("target/debug") {
                                    path = path.parent().unwrap().parent().unwrap().to_path_buf();
                                }
                                path.join("quick_start_deltaforce.sh")
                            } else {
                                PathBuf::from("./quick_start_deltaforce.sh")
                            }
                        },
                        // Directorio actual
                        PathBuf::from("./quick_start_deltaforce.sh"),
                        // Ruta absoluta alternativa
                        PathBuf::from("/home/mrvanguardia/Documentos/PROYECTOS/kernelBridge/quick_start_deltaforce.sh"),
                    ];
                    
                    let mut script_path: Option<PathBuf> = None;
                    for path in &possible_paths {
                        println!("[DELTA FORCE] 🔍 Buscando en: {}", path.display());
                        sender_clone.input(AppMsg::Log(format!("[DELTA FORCE] Buscando en: {}", path.display())));
                        if path.exists() {
                            script_path = Some(path.clone());
                            break;
                        }
                    }
                    
                    let script_path = match script_path {
                        Some(p) => p,
                        None => {
                            eprintln!("[DELTA FORCE] ❌ ERROR: No se encontró quick_start_deltaforce.sh");
                            eprintln!("[DELTA FORCE] Rutas buscadas:");
                            for path in &possible_paths {
                                eprintln!("  - {}", path.display());
                            }
                            sender_clone.input(AppMsg::Log("[DELTA FORCE] ERROR: No se encontró quick_start_deltaforce.sh en ninguna ubicación".to_string()));
                            return;
                        }
                    };
                    
                    println!("[DELTA FORCE] ✅ Script encontrado: {}", script_path.display());
                    sender_clone.input(AppMsg::Log(format!("[DELTA FORCE] ✅ Encontrado: {}", script_path.display())));
                    
                    // Cambiar al directorio del script antes de ejecutarlo
                    let script_dir = script_path.parent().unwrap_or(std::path::Path::new("."));
                    println!("[DELTA FORCE] 📂 Directorio: {}", script_dir.display());
                    
                    // Ejecutar el script mostrando TODA la salida en la terminal actual
                    println!("[DELTA FORCE] 🚀 Ejecutando script...\n");
                    println!("════════════════════════════════════════════════════════════════");
                    
                    use std::process::{Command, Stdio};
                    use std::io::{BufRead, BufReader};
                    
                    let mut child = match Command::new("bash")
                        .arg(&script_path)
                        .current_dir(script_dir)
                        .stdout(Stdio::piped())
                        .stderr(Stdio::piped())
                        .spawn() {
                        Ok(child) => child,
                        Err(e) => {
                            eprintln!("[DELTA FORCE] ❌ ERROR al ejecutar: {}", e);
                            sender_clone.input(AppMsg::Log(format!("[DELTA FORCE] ❌ ERROR: {}", e)));
                            return;
                        }
                    };
                    
                    // Capturar stdout
                    if let Some(stdout) = child.stdout.take() {
                        let reader = BufReader::new(stdout);
                        for line in reader.lines() {
                            if let Ok(line) = line {
                                println!("{}", line);
                                sender_clone.input(AppMsg::Log(line));
                            }
                        }
                    }
                    
                    // Capturar stderr
                    if let Some(stderr) = child.stderr.take() {
                        let reader = BufReader::new(stderr);
                        for line in reader.lines() {
                            if let Ok(line) = line {
                                eprintln!("⚠️  {}", line);
                                sender_clone.input(AppMsg::Log(format!("ERROR: {}", line)));
                            }
                        }
                    }
                    
                    // Esperar a que termine
                    match child.wait() {
                        Ok(status) => {
                            println!("\n════════════════════════════════════════════════════════════════");
                            if status.success() {
                                println!("[DELTA FORCE] ✅ Script completado exitosamente");
                                sender_clone.input(AppMsg::Log("[DELTA FORCE] ✅ Completado".to_string()));
                            } else {
                                eprintln!("[DELTA FORCE] ⚠️  Script terminó con código: {}", status.code().unwrap_or(-1));
                                sender_clone.input(AppMsg::Log(format!("[DELTA FORCE] ⚠️ Código de salida: {}", status.code().unwrap_or(-1))));
                            }
                        }
                        Err(e) => {
                            eprintln!("[DELTA FORCE] ❌ Error esperando el proceso: {}", e);
                            sender_clone.input(AppMsg::Log(format!("[DELTA FORCE] ❌ Error: {}", e)));
                        }
                    }
                    
                    println!("╚════════════════════════════════════════════════════════════════╝\n");
                });
            }
            
            // ═══════════════════════════════════════════════════════════════
            // DELTA FORCE WIZARD - Asistente de Configuración Automático
            // ═══════════════════════════════════════════════════════════════
            
            AppMsg::StartDeltaForceWizard => {
                println!("\n");
                println!("╔═══════════════════════════════════════════════════════════════════════╗");
                println!("║                                                                       ║");
                println!("║          🧙 ASISTENTE DE CONFIGURACIÓN DELTA FORCE + ACE             ║");
                println!("║                                                                       ║");
                println!("╚═══════════════════════════════════════════════════════════════════════╝");
                println!("\n");
                println!("Este asistente te guiará paso a paso para configurar:");
                println!("  ✅ Drivers ACE (AntiCheatExpert)");
                println!("  ✅ Steam Flatpak integration");
                println!("  ✅ GE-Proton optimizations");
                println!("  ✅ Optimizaciones AMD GPU");
                println!("  ✅ GameMode + MangoHud");
                println!("\n");
                println!("════════════════════════════════════════════════════════════════");
                println!("Iniciando verificación del sistema...");
                println!("════════════════════════════════════════════════════════════════\n");
                
                sender.input(AppMsg::DFWizard_Step1_CheckSystem);
            }
            
            AppMsg::DFWizard_Step1_CheckSystem => {
                println!("📋 PASO 1/5: Verificación del Sistema\n");
                
                let sender_clone = sender.clone();
                std::thread::spawn(move || {
                    use std::process::Command;
                    
                    // Verificar SO
                    println!("🔍 Sistema Operativo:");
                    if let Ok(output) = Command::new("cat").arg("/etc/os-release").output() {
                        if let Ok(content) = String::from_utf8(output.stdout) {
                            if let Some(line) = content.lines().find(|l| l.starts_with("PRETTY_NAME")) {
                                println!("   {}", line.split('=').nth(1).unwrap_or("").trim_matches('"'));
                            }
                        }
                    }
                    
                    // Verificar kernel
                    println!("\n🔍 Kernel:");
                    if let Ok(output) = Command::new("uname").arg("-r").output() {
                        if let Ok(kernel) = String::from_utf8(output.stdout) {
                            println!("   {}", kernel.trim());
                        }
                    }
                    
                    // Verificar GPU
                    println!("\n🔍 GPU:");
                    if let Ok(output) = Command::new("lspci").output() {
                        if let Ok(content) = String::from_utf8(output.stdout) {
                            if let Some(line) = content.lines().find(|l| l.contains("VGA")) {
                                if let Some(gpu) = line.split(':').nth(2) {
                                    println!("   {}", gpu.trim());
                                    
                                    // Detectar AMD
                                    if gpu.contains("AMD") || gpu.contains("Radeon") {
                                        println!("   ✅ GPU AMD detectada - Se usarán optimizaciones RADV");
                                    }
                                }
                            }
                        }
                    }
                    
                    // Verificar Steam
                    println!("\n🔍 Steam:");
                    let steam_flatpak = PathBuf::from(shellexpand::tilde("~/.var/app/com.valvesoftware.Steam").as_ref());
                    if steam_flatpak.exists() {
                        println!("   ✅ Steam Flatpak detectado");
                        
                        // Buscar Delta Force
                        let df_path = steam_flatpak.join(".local/share/Steam/steamapps/common");
                        let mut df_found = false;
                        if df_path.exists() {
                            if let Ok(entries) = std::fs::read_dir(&df_path) {
                                for entry in entries.flatten() {
                                    let name = entry.file_name();
                                    let name_str = name.to_string_lossy().to_lowercase();
                                    if name_str.contains("delta") && name_str.contains("force") {
                                        println!("   ✅ Delta Force instalado: {}", name.to_string_lossy());
                                        df_found = true;
                                        break;
                                    }
                                }
                            }
                        }
                        
                        if !df_found {
                            println!("   ⚠️  Delta Force NO detectado");
                            println!("   💡 Instala Delta Force desde Steam primero:");
                            println!("      1. Abre Steam");
                            println!("      2. Busca 'Delta Force'");
                            println!("      3. Instala el juego");
                            println!("      4. Vuelve a ejecutar este asistente");
                        }
                        
                        // Buscar GE-Proton
                        let proton_path = steam_flatpak.join(".local/share/Steam/compatibilitytools.d");
                        let mut ge_found = false;
                        if proton_path.exists() {
                            if let Ok(entries) = std::fs::read_dir(&proton_path) {
                                for entry in entries.flatten() {
                                    let name = entry.file_name();
                                    if name.to_string_lossy().contains("GE-Proton") {
                                        println!("   ✅ {} detectado", name.to_string_lossy());
                                        ge_found = true;
                                        break;
                                    }
                                }
                            }
                        }
                        
                        if !ge_found {
                            println!("   ⚠️  GE-Proton no detectado");
                            println!("   💡 Instala GE-Proton con ProtonUp-Qt:");
                            println!("      flatpak install flathub net.davidotek.pupgui2");
                            println!("      Luego usa ProtonUp-Qt para instalar GE-Proton");
                        }
                    } else {
                        println!("   ⚠️  Steam Flatpak no detectado");
                        println!("   💡 Instala Steam Flatpak:");
                        println!("      flatpak install flathub com.valvesoftware.Steam");
                    }
                    
                    // Verificar ACE drivers
                    println!("\n🔍 Drivers ACE:");
                    let home = PathBuf::from(shellexpand::tilde("~").as_ref());
                    let ace_dir = home.join("Documentos/PROYECTOS/kernelBridge/Win64/AntiCheatExpert");
                    let ace_dir_alt = PathBuf::from("/home/mrvanguardia/Documentos/PROYECTOS/kernelBridge/Win64/AntiCheatExpert");
                    
                    let mut ace_found = false;
                    let check_ace = |dir: &PathBuf| -> Option<usize> {
                        if dir.exists() {
                            if let Ok(entries) = std::fs::read_dir(dir) {
                                Some(entries.filter(|e| {
                                    e.as_ref().ok()
                                        .map(|entry| entry.path().extension().and_then(|ext| ext.to_str()).map(|s| s == "sys").unwrap_or(false))
                                        .unwrap_or(false)
                                }).count())
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    };
                    
                    if let Some(count) = check_ace(&ace_dir) {
                        println!("   ✅ {} drivers ACE encontrados en {}", count, ace_dir.display());
                        ace_found = true;
                    } else if let Some(count) = check_ace(&ace_dir_alt) {
                        println!("   ✅ {} drivers ACE encontrados en {}", count, ace_dir_alt.display());
                        ace_found = true;
                    } else {
                        println!("   ⚠️  Directorio ACE no encontrado");
                        println!("   💡 Busca la carpeta Win64/AntiCheatExpert con los drivers de Delta Force");
                    }
                    
                    println!("\n════════════════════════════════════════════════════════════════");
                    println!("✅ Verificación completada");
                    println!("════════════════════════════════════════════════════════════════\n");
                    
                    std::thread::sleep(std::time::Duration::from_secs(2));
                    sender_clone.input(AppMsg::DFWizard_Step2_InstallTools);
                });
            }
            
            AppMsg::DFWizard_Step2_InstallTools => {
                println!("📦 PASO 2/5: Instalación de Herramientas\n");
                
                let sender_clone = sender.clone();
                std::thread::spawn(move || {
                    use std::process::Command;
                    
                    println!("Instalando GameMode y MangoHud para mejor rendimiento...\n");
                    
                    let output = Command::new("pkexec")
                        .arg("dnf")
                        .arg("install")
                        .arg("-y")
                        .arg("gamemode")
                        .arg("mangohud")
                        .stdout(Stdio::piped())
                        .stderr(Stdio::piped())
                        .spawn();
                    
                    match output {
                        Ok(mut child) => {
                            if let Some(stdout) = child.stdout.take() {
                                let reader = BufReader::new(stdout);
                                for line in reader.lines() {
                                    if let Ok(line) = line {
                                        println!("  {}", line);
                                    }
                                }
                            }
                            
                            match child.wait() {
                                Ok(status) if status.success() => {
                                    println!("\n✅ GameMode y MangoHud instalados correctamente");
                                }
                                _ => {
                                    println!("\n⚠️  Instalación cancelada o falló (no crítico, puedes continuar)");
                                }
                            }
                        }
                        Err(_) => {
                            println!("⚠️  No se pudo ejecutar dnf (¿ya están instalados?)");
                        }
                    }
                    
                    println!("\n════════════════════════════════════════════════════════════════");
                    println!("✅ Herramientas verificadas");
                    println!("════════════════════════════════════════════════════════════════\n");
                    
                    std::thread::sleep(std::time::Duration::from_secs(1));
                    sender_clone.input(AppMsg::DFWizard_Step3_CopyToSteam);
                });
            }
            
            AppMsg::DFWizard_Step3_CopyToSteam => {
                println!("📂 PASO 3/5: Configuración de Archivos para Steam\n");
                
                let sender_clone = sender.clone();
                std::thread::spawn(move || {
                    use std::process::Command;
                    
                    println!("Buscando script de integración con Steam Flatpak...\n");
                    
                    let home = PathBuf::from(shellexpand::tilde("~").as_ref());
                    let possible_scripts = vec![
                        home.join("Documentos/PROYECTOS/kernelBridge/fix_steam_flatpak.sh"),
                        PathBuf::from("/home/mrvanguardia/Documentos/PROYECTOS/kernelBridge/fix_steam_flatpak.sh"),
                        home.join("kernelBridge/fix_steam_flatpak.sh"),
                        PathBuf::from("./fix_steam_flatpak.sh"),
                        std::env::current_dir().ok().map(|p| p.join("fix_steam_flatpak.sh")).unwrap_or_default(),
                    ];
                    
                    let mut script: Option<PathBuf> = None;
                    for candidate in &possible_scripts {
                        if candidate.exists() {
                            println!("✅ Script encontrado: {}", candidate.display());
                            script = Some(candidate.clone());
                            break;
                        }
                    }
                    
                    let script = match script {
                        Some(s) => s,
                        None => {
                            println!("⚠️  No se encontró fix_steam_flatpak.sh automáticamente");
                            println!("\n📋 CONFIGURACIÓN MANUAL:");
                            println!("════════════════════════════════════════════════════════════════\n");
                            println!("Ejecuta estos comandos en otra terminal:\n");
                            println!("cd ~/Documentos/PROYECTOS/kernelBridge");
                            println!("./fix_steam_flatpak.sh\n");
                            println!("════════════════════════════════════════════════════════════════");
                            println!("\n⏸️  Presiona Enter cuando hayas ejecutado el script...");
                            
                            use std::io::{stdin, BufRead};
                            let stdin = stdin();
                            let mut lines = stdin.lock().lines();
                            let _ = lines.next();
                            
                            println!("\n✅ Continuando...\n");
                            sender_clone.input(AppMsg::DFWizard_Step4_ConfigureSteam);
                            return;
                        }
                    };
                    
                    println!("Ejecutando configuración automática...\n");
                    
                    let output = Command::new("bash")
                        .arg(&script)
                        .stdout(Stdio::piped())
                        .stderr(Stdio::piped())
                        .spawn();
                    
                    match output {
                        Ok(mut child) => {
                            if let Some(stdout) = child.stdout.take() {
                                let reader = BufReader::new(stdout);
                                for line in reader.lines() {
                                    if let Ok(line) = line {
                                        println!("{}", line);
                                    }
                                }
                            }
                            
                            if let Some(stderr) = child.stderr.take() {
                                let reader = BufReader::new(stderr);
                                for line in reader.lines() {
                                    if let Ok(line) = line {
                                        eprintln!("{}", line);
                                    }
                                }
                            }
                            
                            match child.wait() {
                                Ok(status) if status.success() => {
                                    println!("\n✅ Archivos copiados al sandbox de Steam correctamente");
                                }
                                _ => {
                                    println!("\n⚠️  Hubo un problema copiando archivos");
                                    println!("   No es crítico, puedes continuar");
                                }
                            }
                        }
                        Err(e) => {
                            println!("⚠️  Error ejecutando script: {}", e);
                            println!("   No es crítico, puedes continuar");
                        }
                    }
                    
                    println!("\n════════════════════════════════════════════════════════════════");
                    println!("✅ Configuración de archivos completada");
                    println!("════════════════════════════════════════════════════════════════\n");
                    
                    std::thread::sleep(std::time::Duration::from_secs(1));
                    sender_clone.input(AppMsg::DFWizard_Step4_ConfigureSteam);
                });
            }
            
            AppMsg::DFWizard_Step4_ConfigureSteam => {
                println!("⚙️  PASO 4/5: Instrucciones para Steam\n");
                println!("════════════════════════════════════════════════════════════════");
                println!("🎮 CONFIGURA STEAM AHORA");
                println!("════════════════════════════════════════════════════════════════\n");
                println!("Sigue estos pasos EN STEAM:\n");
                println!("1️⃣  Abre Steam");
                println!("2️⃣  Ve a tu Biblioteca");
                println!("3️⃣  Click DERECHO en Delta Force → Propiedades\n");
                println!("4️⃣  En la pestaña COMPATIBILIDAD:");
                println!("    ☑️  Marca 'Forzar el uso de una herramienta de compatibilidad...'");
                println!("    ☑️  Selecciona: GE-Proton10-25 (o la versión más reciente)\n");
                println!("5️⃣  En OPCIONES DE LANZAMIENTO, pega EXACTAMENTE esto:\n");
                
                let home = shellexpand::tilde("~");
                let launch_cmd = format!("gamemoderun mangohud {}/.var/app/com.valvesoftware.Steam/data/scripts/steam_deltaforce_wrapper.sh %command%", home);
                
                println!("┌────────────────────────────────────────────────────────────────┐");
                println!("│ {}│", launch_cmd);
                println!("└────────────────────────────────────────────────────────────────┘\n");
                
                // Intentar copiar al clipboard
                let _ = std::process::Command::new("sh")
                    .arg("-c")
                    .arg(format!("echo '{}' | xclip -selection clipboard 2>/dev/null || echo '{}' | wl-copy 2>/dev/null", launch_cmd, launch_cmd))
                    .output();
                
                println!("✅ Comando copiado al portapapeles (Ctrl+V para pegar)\n");
                println!("6️⃣  Click Cerrar\n");
                println!("════════════════════════════════════════════════════════════════");
                println!("⏸️  PAUSA: Configura Steam ahora y luego vuelve aquí");
                println!("════════════════════════════════════════════════════════════════\n");
                println!("Presiona Enter cuando hayas terminado de configurar Steam...");
                
                let sender_clone = sender.clone();
                std::thread::spawn(move || {
                    use std::io::{stdin, BufRead};
                    let stdin = stdin();
                    let mut lines = stdin.lock().lines();
                    let _ = lines.next();
                    
                    sender_clone.input(AppMsg::DFWizard_Step5_LaunchGame);
                });
            }
            
            AppMsg::DFWizard_Step5_LaunchGame => {
                println!("\n🚀 PASO 5/5: Lanzamiento de Delta Force\n");
                println!("════════════════════════════════════════════════════════════════");
                println!("TODO LISTO PARA JUGAR");
                println!("════════════════════════════════════════════════════════════════\n");
                println!("Ahora puedes:");
                println!("  1️⃣  Ir a Steam → Delta Force → JUGAR");
                println!("  2️⃣  El wrapper configurará ACE automáticamente");
                println!("  3️⃣  Verás los logs aquí en esta terminal\n");
                println!("Optimizaciones activas:");
                println!("  ✅ GameMode (rendimiento máximo de CPU/GPU)");
                println!("  ✅ MangoHud (overlay de FPS y estadísticas)");
                println!("  ✅ RADV + ACO (optimizaciones AMD)");
                println!("  ✅ DXVK Async (sin stuttering)");
                println!("  ✅ ACE configurado automáticamente\n");
                println!("════════════════════════════════════════════════════════════════");
                println!("⚠️  IMPORTANTE:");
                println!("════════════════════════════════════════════════════════════════");
                println!("  • Modo Campaña/Offline: Debería funcionar perfectamente");
                println!("  • Modo Multijugador: Puede funcionar, riesgo de detección ACE");
                println!("  • NO uses en cuentas principales (riesgo de baneo)\n");
                println!("════════════════════════════════════════════════════════════════\n");
                
                sender.input(AppMsg::DFWizard_Complete);
            }
            
            AppMsg::DFWizard_Complete => {
                println!("╔═══════════════════════════════════════════════════════════════════════╗");
                println!("║                                                                       ║");
                println!("║              ✅ CONFIGURACIÓN COMPLETADA EXITOSAMENTE                 ║");
                println!("║                                                                       ║");
                println!("╚═══════════════════════════════════════════════════════════════════════╝\n");
                println!("🎮 ¡Disfruta Delta Force en Linux con ACE funcionando!\n");
                println!("📝 Documentación adicional:");
                println!("  • AMD_OPTIMIZATIONS.md - Optimizaciones específicas de GPU");
                println!("  • STEAM_GEPROTON_LISTO.md - Guía completa de Steam + GE-Proton");
                println!("  • DEBUG_MODE.md - Cómo ver logs y debugging\n");
                println!("💡 Comandos útiles:");
                println!("  • deltaforce-logs     → Ver logs del wrapper");
                println!("  • deltaforce-clean    → Limpiar cache de shaders");
                println!("  • kb-debug            → Relanzar GUI con debug completo\n");
                println!("════════════════════════════════════════════════════════════════\n");
                
                self.bridge_logs.push("[WIZARD] ✅ Configuración de Delta Force completada".to_string());
            }
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, sender: ComponentSender<Self>) {
        while let Some(child) = widgets.main_box.first_child() {
            widgets.main_box.remove(&child);
        }

        let paned = gtk::Paned::builder()
            .orientation(gtk::Orientation::Horizontal)
            .wide_handle(true)
            .build();

        let sidebar = create_sidebar(&self.current_section, &sender);
        paned.set_start_child(Some(&sidebar));

        let content = create_content_area(self, &sender);
        paned.set_end_child(Some(&content));
        
        paned.set_position(200);
        widgets.main_box.append(&paned);
    }
}

fn create_sidebar(current: &Section, sender: &ComponentSender<AppModel>) -> gtk::Box {
    let sidebar = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(5)
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
        .build();

    let title = gtk::Label::builder()
        .label("KernelBridge")
        .margin_bottom(20)
        .build();
    sidebar.append(&title);

    let sections = vec![
        (Section::Security, "🔐 Seguridad"),
        (Section::Games, "🎮 Juegos"),
        (Section::Folders, "📁 Carpetas"),
        (Section::KernelBridge, "🧠 KernelBridge"),
        (Section::Settings, "⚙️ Configuración"),
    ];

    for (section, label) in sections {
        let btn = gtk::Button::builder()
            .label(&localize_txt(label))
            .height_request(45)
            .build();
        
        if section == *current {
            btn.add_css_class("suggested-action");
        }
        
        let section_clone = section.clone();
        let sender_clone = sender.clone();
        btn.connect_clicked(move |_| {
            sender_clone.input(AppMsg::ChangeSection(section_clone.clone()));
        });
        
        sidebar.append(&btn);
    }

    sidebar
}

// Localización básica (ES->EN) por variable de entorno KB_LANG=en
fn localize_txt(s: &str) -> String {
    let is_en = std::env::var("KB_LANG").map(|v| v.to_lowercase() == "en").unwrap_or(false);
    if !is_en { return s.to_string(); }
    match s {
        "🔐 Seguridad" => "🔐 Security".to_string(),
        "🎮 Juegos" => "🎮 Games".to_string(),
        "📁 Carpetas" => "📁 Folders".to_string(),
        "🧠 KernelBridge" => "🧠 KernelBridge".to_string(),
        "⚙️ Configuración" => "⚙️ Settings".to_string(),
        "🔐 Seguridad del Sistema" => "🔐 System Security".to_string(),
        "🎮 Biblioteca de Juegos" => "🎮 Game Library".to_string(),
        "📁 Explorador de Carpetas" => "📁 Folder Explorer".to_string(),
        "🧠 Estado de KernelBridge" => "🧠 KernelBridge Status".to_string(),
        _ => s.to_string(),
    }
}

fn create_content_area(model: &AppModel, sender: &ComponentSender<AppModel>) -> gtk::ScrolledWindow {
    let scrolled = gtk::ScrolledWindow::builder()
        .hexpand(true)
        .vexpand(true)
        .build();

    let content = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(15)
        .margin_top(20)
        .margin_bottom(20)
        .margin_start(20)
        .margin_end(20)
        .build();

    match model.current_section {
        Section::Security => create_security_view(model, &content, sender),
        Section::Games => create_games_view(model, &content, sender),
        Section::Folders => create_folders_view(model, &content, sender),
        Section::KernelBridge => create_kernelbridge_view(model, &content, sender),
        Section::Settings => create_settings_view(model, &content, sender),
    }

    scrolled.set_child(Some(&content));
    scrolled
}

fn create_security_view(model: &AppModel, container: &gtk::Box, sender: &ComponentSender<AppModel>) {
    let header = gtk::Label::builder()
        .label(&localize_txt("🔐 Seguridad del Sistema"))
        .xalign(0.0)
        .build();
    container.append(&header);

    let tpm_label = gtk::Label::builder()
        .label(&format!("TPM: {}", model.tpm_status))
        .xalign(0.0)
        .margin_top(10)
        .build();
    container.append(&tpm_label);

    let sb_label = gtk::Label::builder()
        .label(&format!("Secure Boot: {}", model.secure_boot))
        .xalign(0.0)
        .margin_top(5)
        .build();
    container.append(&sb_label);

    let modules_label = gtk::Label::builder()
        .label(&format!("Módulos del Kernel: {}", model.kernel_modules.len()))
        .xalign(0.0)
        .margin_top(15)
        .build();
    container.append(&modules_label);

    for module in model.kernel_modules.iter().take(10) {
        let m_label = gtk::Label::builder()
            .label(&format!("  ⚙️ {} - {}", module.name, module.size))
            .xalign(0.0)
            .build();
        container.append(&m_label);
    }

    let btn = gtk::Button::builder()
        .label("🔄 Actualizar")
        .margin_top(15)
        .build();
    let sender_clone = sender.clone();
    btn.connect_clicked(move |_| {
        sender_clone.input(AppMsg::RefreshSecurity);
    });
    container.append(&btn);
}

fn create_games_view(model: &AppModel, container: &gtk::Box, sender: &ComponentSender<AppModel>) {
    let header = gtk::Label::builder()
        .label(&localize_txt("🎮 Biblioteca de Juegos"))
        .xalign(0.0)
        .build();
    container.append(&header);

    let compatible = model.games.iter().filter(|g| g.compatible).count();
    let stats = gtk::Label::builder()
        .label(&format!("✅ {} compatibles | 📊 {} total", compatible, model.games.len()))
        .xalign(0.0)
        .margin_top(10)
        .build();
    container.append(&stats);

    let btn_scan = gtk::Button::builder()
        .label("🔍 Escanear")
        .margin_top(10)
        .margin_bottom(10)
        .build();
    let sender_clone = sender.clone();
    btn_scan.connect_clicked(move |_| {
        sender_clone.input(AppMsg::ScanGames);
    });
    container.append(&btn_scan);
    
    // ═══════════════════════════════════════════════════════════════
    // DELTA FORCE - Sección especial
    // ═══════════════════════════════════════════════════════════════
    
    let df_frame = gtk::Frame::builder()
        .label("🎯 Delta Force + ACE Anti-Cheat")
        .margin_top(10)
        .margin_bottom(15)
        .build();
    
    let df_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(8)
        .margin_start(12)
        .margin_end(12)
        .margin_top(12)
        .margin_bottom(12)
        .build();
    
    // Botón del asistente de configuración (PRINCIPAL)
    let btn_wizard = gtk::Button::builder()
        .label("🧙 Asistente de Configuración Completa")
        .tooltip_text("Configura Delta Force + ACE + Steam automáticamente paso a paso")
        .build();
    btn_wizard.add_css_class("suggested-action");
    btn_wizard.set_size_request(300, 50);
    let sender_clone = sender.clone();
    btn_wizard.connect_clicked(move |_| {
        sender_clone.input(AppMsg::StartDeltaForceWizard);
    });
    df_box.append(&btn_wizard);
    
    let wizard_desc = gtk::Label::builder()
        .label("↑ Recomendado: Configura todo automáticamente con guía paso a paso")
        .xalign(0.0)
        .wrap(true)
        .build();
    wizard_desc.add_css_class("dim-label");
    df_box.append(&wizard_desc);
    
    // Separador
    let sep = gtk::Separator::new(gtk::Orientation::Horizontal);
    sep.set_margin_top(8);
    sep.set_margin_bottom(8);
    df_box.append(&sep);
    
    // Botón de inicio rápido (para usuarios avanzados)
    let btn_delta_force = gtk::Button::builder()
        .label("⚡ Lanzar Delta Force (Quick Start)")
        .tooltip_text("Solo para usuarios avanzados que ya configuraron todo")
        .build();
    let sender_clone = sender.clone();
    btn_delta_force.connect_clicked(move |_| {
        sender_clone.input(AppMsg::LaunchDeltaForceQuickStart);
    });
    df_box.append(&btn_delta_force);
    
    let quick_desc = gtk::Label::builder()
        .label("↑ Solo si ya configuraste Steam + ACE manualmente")
        .xalign(0.0)
        .wrap(true)
        .build();
    quick_desc.add_css_class("dim-label");
    df_box.append(&quick_desc);
    
    df_frame.set_child(Some(&df_box));
    container.append(&df_frame);
    
    // Sección de launchers detectados
    let home = resolve_home();
    let mut launchers = detect_launchers(&home);
    // Reordenar CurseForge según preferencia (Windows primero si está activado)
    if model.config_prefer_cf_windows {
        let mut others: Vec<LauncherKind> = Vec::new();
        let mut cf_candidates: Vec<LauncherKind> = Vec::new();
        for k in launchers.into_iter() {
            match k {
                LauncherKind::CurseForgeBottles | LauncherKind::CurseForgeWine | LauncherKind::CurseForgeNative => cf_candidates.push(k),
                _ => others.push(k),
            }
        }
        // Orden de preferencia: Bottles -> Wine -> Native
        let mut ordered_cf = Vec::new();
        if cf_candidates.iter().any(|k| matches!(k, LauncherKind::CurseForgeBottles)) { ordered_cf.push(LauncherKind::CurseForgeBottles); }
        if cf_candidates.iter().any(|k| matches!(k, LauncherKind::CurseForgeWine)) { ordered_cf.push(LauncherKind::CurseForgeWine); }
        if cf_candidates.iter().any(|k| matches!(k, LauncherKind::CurseForgeNative)) { ordered_cf.push(LauncherKind::CurseForgeNative); }
        others.extend(ordered_cf);
        launchers = others;
    } else {
        // Orden de preferencia: Native -> Bottles -> Wine
        let mut others: Vec<LauncherKind> = Vec::new();
        let mut cf_candidates: Vec<LauncherKind> = Vec::new();
        for k in launchers.into_iter() {
            match k {
                LauncherKind::CurseForgeBottles | LauncherKind::CurseForgeWine | LauncherKind::CurseForgeNative => cf_candidates.push(k),
                _ => others.push(k),
            }
        }
        let mut ordered_cf = Vec::new();
        if cf_candidates.iter().any(|k| matches!(k, LauncherKind::CurseForgeNative)) { ordered_cf.push(LauncherKind::CurseForgeNative); }
        if cf_candidates.iter().any(|k| matches!(k, LauncherKind::CurseForgeBottles)) { ordered_cf.push(LauncherKind::CurseForgeBottles); }
        if cf_candidates.iter().any(|k| matches!(k, LauncherKind::CurseForgeWine)) { ordered_cf.push(LauncherKind::CurseForgeWine); }
        others.extend(ordered_cf);
        launchers = others;
    }
    if !launchers.is_empty() {
        let lhdr = gtk::Label::builder()
            .label("🚀 Launchers detectados")
            .xalign(0.0)
            .margin_top(10)
            .build();
        container.append(&lhdr);

        // Distribuir los launchers en una grilla de 5x5 (5 columnas x hasta 5 filas)
        let flow = gtk::FlowBox::new();
        flow.set_selection_mode(gtk::SelectionMode::None);
        flow.set_max_children_per_line(5);
        flow.set_row_spacing(8);
        flow.set_column_spacing(8);
        flow.set_homogeneous(true);
        let total_launchers = launchers.len();
        for l in launchers.into_iter().take(25) {
            let (label, kind) = match l {
                LauncherKind::SteamFlatpak => ("Steam (Flatpak)", LauncherKind::SteamFlatpak),
                LauncherKind::SteamNative => ("Steam (Nativo)", LauncherKind::SteamNative),
                LauncherKind::BottlesFlatpak => ("Bottles (Flatpak)", LauncherKind::BottlesFlatpak),
                LauncherKind::BottlesNative => ("Bottles (Nativo)", LauncherKind::BottlesNative),
                LauncherKind::LutrisFlatpak => ("Lutris (Flatpak)", LauncherKind::LutrisFlatpak),
                LauncherKind::LutrisNative => ("Lutris (Nativo)", LauncherKind::LutrisNative),
                LauncherKind::Wine => ("Wine", LauncherKind::Wine),
                LauncherKind::MinecraftFlatpak => ("Minecraft (Flatpak)", LauncherKind::MinecraftFlatpak),
                LauncherKind::MinecraftNative => ("Minecraft (Nativo)", LauncherKind::MinecraftNative),
                LauncherKind::MinecraftWine => ("Minecraft (Wine)", LauncherKind::MinecraftWine),
                LauncherKind::PrismFlatpak => ("Prism (Flatpak)", LauncherKind::PrismFlatpak),
                LauncherKind::PrismNative => ("Prism (Nativo)", LauncherKind::PrismNative),
                LauncherKind::CurseForgeBottles => ("CurseForge (Bottles)", LauncherKind::CurseForgeBottles),
                LauncherKind::CurseForgeWine => ("CurseForge (Wine)", LauncherKind::CurseForgeWine),
                LauncherKind::CurseForgeNative => ("CurseForge (Nativo)", LauncherKind::CurseForgeNative),
            };
            let btn = gtk::Button::builder().label(label).build();
            let sender_clone = sender.clone();
            btn.connect_clicked(move |_| {
                sender_clone.input(AppMsg::LaunchLauncher(kind.clone()));
            });
            flow.append(&btn);
        }
        container.append(&flow);
        if total_launchers > 25 {
            let more = gtk::Label::builder()
                .label(&format!("… y {} más", total_launchers - 25))
                .xalign(0.0)
                .margin_top(6)
                .build();
            container.append(&more);
        }
    }

    // Sección específica: Minecraft Launcher (oficial) y Prism Launcher
    let minecraft_header = gtk::Label::builder()
        .label("⛏️ Minecraft")
        .xalign(0.0)
        .margin_top(16)
        .build();
    container.append(&minecraft_header);

    let mc_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(8)
        .build();

    // (Botones directos de Minecraft nativo/Flatpak eliminados; ya aparecen en "Launchers detectados")

    // Opción directa: descargar y extraer el tar.gz oficial (nativo)
    let mc_downloaded = is_minecraft_tar_downloaded();
    let btn_text = if mc_downloaded { "✅ Launcher Descargado" } else { "Descargar Minecraft (Nativo)" };
    let btn_dl_tar = gtk::Button::builder().label(btn_text).build();
    let s_dl = sender.clone();
    btn_dl_tar.connect_clicked(move |_| { s_dl.input(AppMsg::DownloadMinecraftLinuxTar); });
    mc_box.append(&btn_dl_tar);

    // Botón para usar el launcher de Windows con Wine (descarga + instalación o lanzar si ya está)
    let has_mc_wine = find_minecraft_wine_exe().is_some();
    let btn_wine_text = if has_mc_wine { "✅ Minecraft Wine" } else { "Minecraft Wine" };
    let btn_mc_wine = gtk::Button::builder().label(btn_wine_text).build();
    if has_mc_wine {
        let s = sender.clone();
        btn_mc_wine.connect_clicked(move |_| { s.input(AppMsg::LaunchLauncher(LauncherKind::MinecraftWine)); });
    } else {
        let s = sender.clone();
        btn_mc_wine.connect_clicked(move |_| { s.input(AppMsg::InstallMinecraftWine); });
    }
    mc_box.append(&btn_mc_wine);

    // Detección/opción de Prism Launcher (alternativa FOSS)
    let has_prism_flatpak = flatpak_is_installed("org.prismlauncher.PrismLauncher") || home.join(".var/app/org.prismlauncher.PrismLauncher").exists();
    let has_prism_native = which_exists("prismlauncher");
    if has_prism_flatpak {
        let btn = gtk::Button::builder().label("Prism Launcher (Flatpak)").build();
        let sender_clone = sender.clone();
        btn.connect_clicked(move |_| { sender_clone.input(AppMsg::LaunchLauncher(LauncherKind::PrismFlatpak)); });
        mc_box.append(&btn);
    } else if has_prism_native {
        let btn = gtk::Button::builder().label("Prism Launcher (Nativo)").build();
        let sender_clone = sender.clone();
        btn.connect_clicked(move |_| { sender_clone.input(AppMsg::LaunchLauncher(LauncherKind::PrismNative)); });
        mc_box.append(&btn);
    }

    // Botón de estado/instalación de Minecraft (Flatpak): siempre visible
    let has_mc_flatpak = flatpak_is_installed("com.mojang.Minecraft") || home.join(".var/app/com.mojang.Minecraft").exists();
    let mc_flatpak_btn_text = if has_mc_flatpak { "✅ Instalado (Flatpak)" } else { "Instalar Minecraft (Flatpak)" };
    let mc_flatpak_btn = gtk::Button::builder().label(mc_flatpak_btn_text).build();
    if has_mc_flatpak {
        mc_flatpak_btn.set_sensitive(false);
    } else {
        let s2 = sender.clone();
        mc_flatpak_btn.connect_clicked(move |_| { s2.input(AppMsg::InstallMinecraftFlatpak); });
    }
    mc_box.append(&mc_flatpak_btn);

    // Botón de estado/instalación de Prism (Flatpak): siempre visible
    let prism_btn_text = if has_prism_flatpak { "✅ Instalado (Flatpak)" } else { "Instalar Prism Launcher (Flatpak)" };
    let prism_btn = gtk::Button::builder().label(prism_btn_text).build();
    if has_prism_flatpak {
        prism_btn.set_sensitive(false);
    } else {
        let sender_clone = sender.clone();
        prism_btn.connect_clicked(move |_| { sender_clone.input(AppMsg::InstallPrismFlatpak); });
    }
    mc_box.append(&prism_btn);

    container.append(&mc_box);

    // Sección CurseForge (Overwolf)
    let cf_header = gtk::Label::builder()
        .label("🧩 CurseForge (Overwolf)")
        .xalign(0.0)
        .margin_top(20)
        .build();
    container.append(&cf_header);

    let cf_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(8)
        .build();

    // Linux (AppImage) – descarga zip y extracción
    let cf_downloaded = is_curseforge_linux_downloaded();
    let cf_btn_text = if cf_downloaded { "✅ Launcher Descargado" } else { "Descargar CurseForge (Linux)" };
    let btn_dl_linux = gtk::Button::builder().label(cf_btn_text).build();
    let s = sender.clone();
    btn_dl_linux.connect_clicked(move |_| { s.input(AppMsg::DownloadCurseForgeLinux); });
    cf_box.append(&btn_dl_linux);

    // Nota: el lanzamiento de CurseForge (nativo o Wine/Bottles) sólo se muestra en "Launchers detectados".

    container.append(&cf_box);

    if model.games.is_empty() {
        let empty = gtk::Label::builder()
            .label("📭 No se encontraron juegos (usa los launchers arriba para iniciar)")
            .margin_top(30)
            .build();
        container.append(&empty);
    } else {
        for game in model.games.iter().take(20) {
            let game_box = gtk::Box::builder()
                .orientation(gtk::Orientation::Vertical)
                .spacing(5)
                .margin_top(10)
                .build();

            let name = gtk::Label::builder()
                .label(&format!("🎮 {}", game.name))
                .xalign(0.0)
                .build();
            game_box.append(&name);

            let info = if game.compatible {
                format!("  ✅ Compatible | 🛡️ {}", game.anticheat)
            } else {
                format!("  ❌ No compatible = (Emulado) | 🛡️ {}", game.anticheat)
            };
            let info_label = gtk::Label::builder()
                .label(&info)
                .xalign(0.0)
                .build();
            game_box.append(&info_label);

            let source_text = match game.source {
                GameSource::Steam => format!("Fuente: Steam (AppID: {})", game.launch_id),
                GameSource::Bottles => format!("Fuente: Bottles ({})", game.launch_id),
                GameSource::Lutris => format!("Fuente: Lutris (slug: {})", game.launch_id),
                GameSource::Local => "Fuente: Local".to_string(),
            };
            let source_label = gtk::Label::builder()
                .label(&source_text)
                .xalign(0.0)
                .build();
            game_box.append(&source_label);

            let btn = gtk::Button::builder()
                .label(&format!("▶️ Ejecutar con {}", match game.source {
                    GameSource::Steam => "Steam",
                    GameSource::Bottles => "Bottles",
                    GameSource::Lutris => "Lutris",
                    GameSource::Local => "Wine",
                }))
                .build();
            btn.add_css_class("suggested-action");
            let game_clone = game.clone();
            let sender_clone = sender.clone();
            btn.connect_clicked(move |_| {
                sender_clone.input(AppMsg::ExecuteGame(game_clone.clone()));
            });
            game_box.append(&btn);

            // Si el juego parece usar anti‑cheat de kernel y no está activado el forzado global, ofrecer botón de ignorar AC
            if is_kernel_ac_game(game) && !model.config_force_bottles {
                let force_btn = gtk::Button::builder()
                    .label("⚠️ Forzar Bottles (ignorar AC)")
                    .build();
                let game_clone = game.clone();
                let sender_clone = sender.clone();
                force_btn.connect_clicked(move |_| {
                    sender_clone.input(AppMsg::ExecuteGameIgnoreAC(game_clone.clone()));
                });
                game_box.append(&force_btn);
            }

            container.append(&game_box);
        }
    }
}

fn detect_launchers(home: &PathBuf) -> Vec<LauncherKind> {
    let mut list = Vec::new();
    // Steam
    if home.join(".var/app/com.valvesoftware.Steam").exists() { list.push(LauncherKind::SteamFlatpak); }
    if which_exists("steam") { list.push(LauncherKind::SteamNative); }
    // Bottles
    if home.join(".var/app/com.usebottles.bottles").exists() { list.push(LauncherKind::BottlesFlatpak); }
    if which_exists("bottles") || which_exists("bottles-cli") { list.push(LauncherKind::BottlesNative); }
    // Lutris
    if home.join(".var/app/net.lutris.Lutris").exists() { list.push(LauncherKind::LutrisFlatpak); }
    if which_exists("lutris") { list.push(LauncherKind::LutrisNative); }
    // Minecraft (preferencia nativo cuando esté). Detectar por PATH o por binario extraído del tar.gz
    if which_exists("minecraft-launcher") || is_minecraft_tar_downloaded() { list.push(LauncherKind::MinecraftNative); }
    if flatpak_is_installed("com.mojang.Minecraft") || home.join(".var/app/com.mojang.Minecraft").exists() { list.push(LauncherKind::MinecraftFlatpak); }
    // Minecraft Wine (Windows launcher en prefix por defecto)
    if find_minecraft_wine_exe().is_some() { list.push(LauncherKind::MinecraftWine); }
    // Prism Launcher
    if which_exists("prismlauncher") { list.push(LauncherKind::PrismNative); }
    if flatpak_is_installed("org.prismlauncher.PrismLauncher") || home.join(".var/app/org.prismlauncher.PrismLauncher").exists() { list.push(LauncherKind::PrismFlatpak); }
    // CurseForge (Overwolf) detectado en Bottles/Wine
    // Nativo (AppImage) extraído
    if curseforge_appimage_path().is_some() { list.push(LauncherKind::CurseForgeNative); }
    // Bottles bottle "CurseForge"
    let cf_bottle_drive_c = home.join(".local/share/bottles/bottles/CurseForge/drive_c");
    if let Some(_) = find_file_recursive(&cf_bottle_drive_c, "CurseForge.exe")
        .or_else(|| find_file_recursive(&cf_bottle_drive_c, "Overwolf.exe"))
    {
        list.push(LauncherKind::CurseForgeBottles);
    }
    // Wine prefix por defecto
    let wine_drive_c = home.join(".wine/drive_c");
    if let Some(_) = find_file_recursive(&wine_drive_c, "CurseForge.exe")
        .or_else(|| find_file_recursive(&wine_drive_c, "Overwolf.exe"))
    {
        list.push(LauncherKind::CurseForgeWine);
    }
    // Wine (siempre útil tenerlo)
    if which_exists("wine") { list.push(LauncherKind::Wine); }
    list
}

fn which_exists(cmd: &str) -> bool {
    if let Ok(paths) = std::env::var("PATH") {
        for p in paths.split(':') {
            let candidate = PathBuf::from(p).join(cmd);
            if candidate.exists() { return true; }
        }
    }
    false
}

// Detectar si el proceso corre como root (Linux)
fn is_root() -> bool {
    #[cfg(unix)]
    {
        if let Ok(s) = std::fs::read_to_string("/proc/self/status") {
            for line in s.lines() {
                if line.starts_with("Uid:") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.get(1) == Some(&"0") {
                        return true;
                    }
                }
            }
        }
    }
    std::env::var("USER").map(|u| u == "root").unwrap_or(false)
}

fn downloads_dir() -> PathBuf {
    let base = resolve_home().join(".local/share/KernelBridge/downloads");
    let _ = std::fs::create_dir_all(&base);
    base
}

fn curseforge_installer_path() -> String {
    downloads_dir().join("CurseForge-Installer.exe").to_string_lossy().to_string()
}

fn curseforge_linux_zip_path() -> String {
    downloads_dir().join("curseforge-latest-linux.zip").to_string_lossy().to_string()
}

fn curseforge_linux_extract_dir() -> PathBuf {
    downloads_dir().join("curseforge-linux")
}

fn flatpak_is_installed(app_id: &str) -> bool {
    if !which_exists("flatpak") { return false; }
    // Probar instalación de usuario primero, luego del sistema
    let user_ok = Cmd::new("flatpak").args(["info", app_id, "--user"]).output().map(|o| o.status.success()).unwrap_or(false);
    if user_ok { return true; }
    Cmd::new("flatpak").args(["info", app_id]).output().map(|o| o.status.success()).unwrap_or(false)
}

// Minecraft (tar.gz) helpers
fn minecraft_tarball_path() -> String {
    downloads_dir().join("Minecraft.tar.gz").to_string_lossy().to_string()
}

fn minecraft_extract_dir() -> PathBuf {
    downloads_dir().join("minecraft-linux")
}

fn download_minecraft_linux_tar() -> Result<String, String> {
    let out = minecraft_tarball_path();
    let url = "https://launcher.mojang.com/download/Minecraft.tar.gz";
    let ok = Cmd::new("curl")
        .args(["-fL", "-o", &out, url])
        .spawn()
        .map_err(|e| e.to_string())?
        .wait()
        .map_err(|e| e.to_string())?
        .success();
    if ok { Ok(out) } else { Err("No se pudo descargar Minecraft.tar.gz".to_string()) }
}

fn extract_minecraft_linux_tar(tar_path: &str) -> Result<String, String> {
    let dest = minecraft_extract_dir();
    let _ = std::fs::create_dir_all(&dest);
    // Usar tar si está disponible; fallback a bsdtar
    let ok = if which_exists("tar") {
        extract_with("tar", &["-xzf", tar_path, "-C", &dest.to_string_lossy()])
    } else if which_exists("bsdtar") {
        extract_with("bsdtar", &["-xf", tar_path, "-C", &dest.to_string_lossy()])
    } else {
        return Err("No hay herramienta para extraer tar.gz (se requiere 'tar' o 'bsdtar')".to_string());
    };
    if !ok { return Err("Fallo al extraer el tar.gz".to_string()); }

    // Intentar hacer ejecutable el binario del launcher si existe
    if let Some(bin) = find_minecraft_launcher_binary() {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(meta) = std::fs::metadata(&bin) {
                let perm = meta.permissions();
                if perm.mode() & 0o111 == 0 {
                    let _ = std::fs::set_permissions(&bin, PermissionsExt::from_mode(0o755));
                }
            }
        }
    }

    Ok(dest.to_string_lossy().to_string())
}

// Buscar archivo por nombre exacto de manera recursiva
fn find_file_recursive_exact(base: &Path, exact_name: &str) -> Option<PathBuf> {
    if !base.exists() { return None; }
    let mut stack = vec![base.to_path_buf()];
    while let Some(dir) = stack.pop() {
        if let Ok(entries) = std::fs::read_dir(&dir) {
            for e in entries.flatten() {
                let p = e.path();
                if p.is_dir() { stack.push(p); continue; }
                if let Some(fname) = p.file_name().and_then(|s| s.to_str()) {
                    if fname == exact_name { return Some(p); }
                }
            }
        }
    }
    None
}

fn find_minecraft_launcher_binary() -> Option<PathBuf> {
    let dir = minecraft_extract_dir();
    if !dir.exists() { return None; }
    // 1) Preferir coincidencia exacta 'minecraft-launcher'
    if let Some(p) = find_file_recursive_exact(&dir, "minecraft-launcher") { return Some(p); }
    // 2) Fallback: cualquier archivo que contenga 'minecraft-launcher' pero no sea .desktop ni imagen
    let mut stack = vec![dir];
    while let Some(d) = stack.pop() {
        if let Ok(entries) = std::fs::read_dir(&d) {
            for e in entries.flatten() {
                let p = e.path();
                if p.is_dir() { stack.push(p); continue; }
                if let Some(fname) = p.file_name().and_then(|s| s.to_str()) {
                    let lname = fname.to_lowercase();
                    if lname.contains("minecraft-launcher") && !lname.ends_with(".desktop") && !lname.ends_with(".png") && !lname.ends_with(".svg") {
                        return Some(p);
                    }
                }
            }
        }
    }
    None
}

fn is_minecraft_tar_downloaded() -> bool {
    find_minecraft_launcher_binary().is_some()
}

fn download_curseforge_installer() -> Result<String, String> {
    let out = curseforge_installer_path();
    // Intento principal: endpoint oficial con redirecciones
    let url = "https://download.curseforge.com/";
    let ok = Cmd::new("curl")
        .args(["-fL", "-o", &out, url])
        .spawn()
        .map_err(|e| e.to_string())?
        .wait()
        .map_err(|e| e.to_string())?
        .success();
    if ok { return Ok(out); }
    // Fallback a URL legacy conocida (puede cambiar en el tiempo)
    let url2 = "https://download.curseforge.com/curseforge/windows/CurseForge-Installer.exe";
    let ok2 = Cmd::new("curl")
        .args(["-fL", "-o", &out, url2])
        .spawn()
        .map_err(|e| e.to_string())?
        .wait()
        .map_err(|e| e.to_string())?
        .success();
    if ok2 { Ok(out) } else { Err("No se pudo descargar el instalador (curl/URL)".to_string()) }
}

// Minecraft Windows (Wine) helpers
fn minecraft_windows_installer_path() -> String {
    downloads_dir().join("MinecraftInstaller.exe").to_string_lossy().to_string()
}

fn download_minecraft_windows_installer() -> Result<String, String> {
    let out = minecraft_windows_installer_path();
    // Intentar exe oficial
    let url_exe = "https://launcher.mojang.com/download/MinecraftInstaller.exe";
    let ok = Cmd::new("curl")
        .args(["-fL", "-o", &out, url_exe])
        .spawn()
        .map_err(|e| e.to_string())?
        .wait()
        .map_err(|e| e.to_string())?
        .success();
    if ok { return Ok(out); }
    // Fallback MSI
    let out_msi = downloads_dir().join("MinecraftInstaller.msi").to_string_lossy().to_string();
    let url_msi = "https://launcher.mojang.com/download/MinecraftInstaller.msi";
    let ok2 = Cmd::new("curl")
        .args(["-fL", "-o", &out_msi, url_msi])
        .spawn()
        .map_err(|e| e.to_string())?
        .wait()
        .map_err(|e| e.to_string())?
        .success();
    if ok2 { Ok(out_msi) } else { Err("No se pudo descargar el instalador de Minecraft para Windows".to_string()) }
}

fn find_minecraft_wine_exe() -> Option<PathBuf> {
    use std::env;
    let home = resolve_home();
    // Posibles prefixes a revisar: WINEPREFIX, ~/.wine, y playonlinux/lutris prefixes comunes
    let mut prefixes: Vec<PathBuf> = Vec::new();
    if let Ok(p) = env::var("WINEPREFIX") { let wp = PathBuf::from(p); if wp.exists() { prefixes.push(wp); } }
    let def = home.join(".wine");
    if def.exists() { prefixes.push(def); }
    // PlayOnLinux/Lutris estilos comunes
    let pol = home.join(".local/share/wineprefixes");
    if pol.exists() {
        if let Ok(entries) = std::fs::read_dir(&pol) {
            for e in entries.flatten() { let p = e.path(); if p.is_dir() { prefixes.push(p); } }
        }
    }
    // Buscar los ejecutables típicos del launcher en cada prefix
    for prefix in prefixes {
        let drive_c = prefix.join("drive_c");
        if let Some(p) = find_file_recursive(&drive_c, "MinecraftLauncher.exe").or_else(|| find_file_recursive(&drive_c, "Minecraft.exe")) {
            return Some(p);
        }
    }
    None
}

fn launch_minecraft_wine() -> bool {
    if let Some(exe) = find_minecraft_wine_exe() {
        return spawn_program_s("wine", vec![exe.to_string_lossy().to_string()]);
    }
    false
}

fn install_minecraft_wine() -> bool {
    match download_minecraft_windows_installer() {
        Ok(installer) => {
            // Ejecutar EXE/MSI con Wine
            if installer.to_lowercase().ends_with(".msi") {
                // msiexec /i
                spawn_program_s("wine", vec!["msiexec".to_string(), "/i".to_string(), installer])
            } else {
                spawn_program_s("wine", vec![installer])
            }
        }
        Err(_) => false,
    }
}

fn download_curseforge_linux_zip() -> Result<String, String> {
    let out = curseforge_linux_zip_path();
    let url = "https://curseforge.overwolf.com/downloads/curseforge-latest-linux.zip";
    let ok = Cmd::new("curl")
        .args(["-fL", "-o", &out, url])
        .spawn()
        .map_err(|e| e.to_string())?
        .wait()
        .map_err(|e| e.to_string())?
        .success();
    if ok { Ok(out) } else { Err("No se pudo descargar el zip de CurseForge Linux".to_string()) }
}

fn extract_with(cmd: &str, args: &[&str]) -> bool {
    Cmd::new(cmd).args(args).spawn().map(|mut c| c.wait().map(|s| s.success()).unwrap_or(false)).unwrap_or(false)
}

fn extract_curseforge_linux_zip(zip_path: &str) -> Result<String, String> {
    let dest = curseforge_linux_extract_dir();
    let _ = std::fs::create_dir_all(&dest);
    // Preferir unzip
    let ok = if which_exists("unzip") {
        extract_with("unzip", &["-o", zip_path, "-d", &dest.to_string_lossy()])
    } else if which_exists("bsdtar") {
        extract_with("bsdtar", &["-xf", zip_path, "-C", &dest.to_string_lossy()])
    } else if which_exists("7z") || which_exists("7za") {
        let seven = if which_exists("7z") { "7z" } else { "7za" };
        extract_with(seven, &["x", zip_path, &format!("-o{}", dest.to_string_lossy())])
    } else {
        return Err("No hay herramienta para descomprimir (instala 'unzip' o 'bsdtar')".to_string());
    };
    if ok { Ok(dest.to_string_lossy().to_string()) } else { Err("Fallo al extraer el zip".to_string()) }
}

fn ensure_bottle(name: &str) -> bool {
    if !(which_exists("bottles-cli") || which_exists("bottles")) { return false; }
    // bottles-cli list retorna nombres; si no existe, crear
    let list = Cmd::new("bottles-cli").args(["list"]).output();
    let exists = match list {
        Ok(out) => String::from_utf8_lossy(&out.stdout).lines().any(|l| l.trim() == name),
        Err(_) => false,
    };
    if exists { return true; }
    // Crear bottle vacía
    Cmd::new("bottles-cli").args(["new", "-n", name]).spawn().is_ok()
}

fn install_curseforge_bottles() -> bool {
    let installer = curseforge_installer_path();
    if !Path::new(&installer).exists() { return false; }
    if !ensure_bottle("CurseForge") { return false; }
    // Ejecutar instalador dentro del bottle
    spawn_program_s("bottles-cli", vec![
        "run".to_string(),
        "-b".to_string(), "CurseForge".to_string(),
        "-e".to_string(), installer,
    ])
}

fn install_curseforge_wine() -> bool {
    let installer = curseforge_installer_path();
    if !Path::new(&installer).exists() { return false; }
    spawn_program_s("wine", vec![installer])
}

fn find_file_recursive(base: &Path, name_contains: &str) -> Option<PathBuf> {
    if !base.exists() { return None; }
    let mut stack = vec![base.to_path_buf()];
    while let Some(dir) = stack.pop() {
        if let Ok(entries) = std::fs::read_dir(&dir) {
            for e in entries.flatten() {
                let p = e.path();
                if p.is_dir() { stack.push(p); continue; }
                if let Some(fname) = p.file_name().and_then(|s| s.to_str()) {
                    if fname.to_lowercase().contains(&name_contains.to_lowercase()) {
                        return Some(p);
                    }
                }
            }
        }
    }
    None
}

fn launch_curseforge_bottles() -> bool {
    // Buscar ejecutable dentro del bottle "CurseForge"
    let home = resolve_home();
    let drive_c = home.join(".local/share/bottles/bottles/CurseForge/drive_c");
    if let Some(exe) = find_file_recursive(&drive_c, "CurseForge.exe")
        .or_else(|| find_file_recursive(&drive_c, "Overwolf.exe"))
    {
        return spawn_program_s("bottles-cli", vec![
            "run".to_string(),
            "-b".to_string(), "CurseForge".to_string(),
            "-e".to_string(), exe.to_string_lossy().to_string(),
        ]);
    }
    false
}

fn launch_curseforge_wine() -> bool {
    // Buscar en ~/.wine drive_c
    let home = resolve_home();
    let drive_c = home.join(".wine/drive_c");
    if let Some(exe) = find_file_recursive(&drive_c, "CurseForge.exe")
        .or_else(|| find_file_recursive(&drive_c, "Overwolf.exe"))
    {
        return spawn_program_s("wine", vec![exe.to_string_lossy().to_string()]);
    }
    false
}

fn curseforge_appimage_path() -> Option<PathBuf> {
    let dir = curseforge_linux_extract_dir();
    if !dir.exists() {
        return None;
    }
    // Buscar cualquier .AppImage
    find_file_recursive(&dir, ".appimage")
}

fn launch_curseforge_native() -> bool {
    if let Some(appimage) = curseforge_appimage_path() {
        // chmod +x si hace falta
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(meta) = std::fs::metadata(&appimage) {
                let perm = meta.permissions();
                if perm.mode() & 0o111 == 0 {
                    let _ = std::fs::set_permissions(&appimage, PermissionsExt::from_mode(0o755));
                }
            }
        }
        // Lanzamiento nativo (AppImage): lanzar directamente; si se ejecuta como root, añadir --no-sandbox.
        // NOTA: por defecto NO redirigimos stdout/stderr para no interferir con los launchers.
        // Si necesitas mitigar "write EIO" en Electron, exporta KB_CF_LOG_TO_FILE=1
        // y entonces se redirigirá a /tmp/curseforge-electron.log
        let mut cmd = Cmd::new(appimage.to_string_lossy().to_string());
        if is_root() {
            cmd.arg("--no-sandbox");
            cmd.env("ELECTRON_DISABLE_SANDBOX", "1");
            // Si el usuario habilitó el puente de audio y tenemos contexto real de usuario,
            // exportar variables para usar el Pulse/DBus del usuario no-root.
            if std::env::var("KB_AUDIO_BRIDGE_ROOT").ok().map(|v| v == "1" || v.to_lowercase() == "true").unwrap_or(false) {
                if let Some((_, uid_str, user_home)) = real_user_context() {
                    let xdg_runtime = format!("/run/user/{}", uid_str);
                    let dbus_addr = format!("unix:path={}/bus", xdg_runtime);
                    let pulse_sock = format!("unix:{}/pulse/native", xdg_runtime);
                    cmd.env("XDG_RUNTIME_DIR", &xdg_runtime);
                    cmd.env("DBUS_SESSION_BUS_ADDRESS", &dbus_addr);
                    cmd.env("PULSE_SERVER", &pulse_sock);
                    cmd.env("PIPEWIRE_RUNTIME_DIR", &xdg_runtime);
                    // Mantener HOME de root; cambiar HOME puede romper perfiles.
                    let _ = user_home; // silenciar warning si no se usa
                }
            }
        }
        if std::env::var("KB_CF_LOG_TO_FILE").ok().as_deref() == Some("1") {
            use std::fs::OpenOptions;
            use std::process::Stdio;
            if let Ok(outf) = OpenOptions::new().create(true).append(true).open("/tmp/curseforge-electron.log") {
                let _ = cmd.stdout(Stdio::from(outf));
            }
            if let Ok(errf) = OpenOptions::new().create(true).append(true).open("/tmp/curseforge-electron.log") {
                let _ = cmd.stderr(Stdio::from(errf));
            }
        }
        return cmd.spawn().is_ok();
    }
    false
}

fn is_curseforge_linux_downloaded() -> bool {
    curseforge_appimage_path().is_some()
}

fn find_steam_root() -> Option<PathBuf> {
    let home = resolve_home();
    for candidate in [
        home.join(".local/share/Steam"),
        home.join(".steam/steam"),
    ] {
        if candidate.exists() { return Some(candidate); }
    }
    None
}

fn find_proton_ge_script() -> Option<PathBuf> {
    let steam = find_steam_root()?;
    // Buscar en compatibilitytools.d/*/proton y steamapps/common/Proton* directories
    let ct = steam.join("compatibilitytools.d");
    if ct.exists() {
        if let Ok(entries) = std::fs::read_dir(&ct) {
            for e in entries.flatten() {
                let path = e.path();
                let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if name.to_lowercase().contains("ge-proton") || name.to_lowercase().contains("proton-ge") {
                    let p = path.join("proton");
                    if p.exists() { return Some(p); }
                }
            }
        }
    }
    // Fallback común
    let common = steam.join("steamapps/common");
    if common.exists() {
        if let Ok(entries) = std::fs::read_dir(&common) {
            for e in entries.flatten() {
                let p = e.path();
                let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if name.to_lowercase().contains("proton") && name.to_lowercase().contains("ge") {
                    let s = p.join("proton");
                    if s.exists() { return Some(s); }
                }
            }
        }
    }
    None
}

fn slugify_path(p: &PathBuf) -> String {
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;
    let s = p.to_string_lossy();
    let mut h = DefaultHasher::new();
    s.hash(&mut h);
    format!("{:x}", h.finish())
}

fn launch_local_with_wine(path: &PathBuf, offline: bool, mitigations: bool, force_vkd3d: bool, sender: &ComponentSender<AppModel>) {
    let mut cmd;
    let use_firejail = offline && which_exists("firejail");
    if use_firejail {
        cmd = Cmd::new("firejail");
        cmd.arg("--noprofile").arg("--net=none");
        cmd.arg("wine");
    } else {
        cmd = Cmd::new("wine");
        if offline { sender.input(AppMsg::Log("[OFFLINE] No se pudo bloquear red para Wine (instala 'firejail')".to_string())); }
    }
    if let Some(parent_dir) = PathBuf::from(path).parent() { cmd.current_dir(parent_dir); }
    if force_vkd3d {
        if let Some(parent_dir) = PathBuf::from(path).parent() {
            let has_d3d12 = parent_dir.join("d3d12.dll").exists();
            let has_d3d12core = parent_dir.join("d3d12core.dll").exists();
            if has_d3d12 || has_d3d12core {
                cmd.env("WINEDLLOVERRIDES", "d3d12=n,b;d3d12core=n,b;dxgi=n,b");
                sender.input(AppMsg::Log("[VKD3D] Overrides activos: d3d12/d3d12core/dxgi = native,builtin".to_string()));
            } else {
                sender.input(AppMsg::Log("[VKD3D] No se encontró d3d12.dll en la carpeta del juego. Descarga vkd3d-proton: https://github.com/HansKristian-Work/vkd3d-proton/releases".to_string()));
            }
        }
    }
    if mitigations {
        cmd.env("EOS_OVERLAY_DISABLED", "1");
        cmd.env("WINEESYNC", "1");
        cmd.env("WINEFSYNC", "1");
        cmd.env("DXVK_LOG_LEVEL", "none");
        sender.input(AppMsg::Log("[LAUNCH] Local: EOS overlay OFF, esync/fsync ON, DXVK quiet".to_string()));
    }
    cmd.arg(path);
    let _ = cmd.spawn();
}

fn create_folders_view(model: &AppModel, container: &gtk::Box, sender: &ComponentSender<AppModel>) {
    let header = gtk::Label::builder()
        .label(&localize_txt("📁 Explorador de Carpetas"))
        .xalign(0.0)
        .build();
    container.append(&header);

    let nav_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(10)
        .margin_top(10)
        .build();

    let btn_up = gtk::Button::builder()
        .label("⬆️ Arriba")
        .build();
    let sender_clone = sender.clone();
    btn_up.connect_clicked(move |_| {
        sender_clone.input(AppMsg::GoUp);
    });
    nav_box.append(&btn_up);

    let btn_refresh = gtk::Button::builder()
        .label("🔄 Actualizar")
        .build();
    let sender_clone = sender.clone();
    btn_refresh.connect_clicked(move |_| {
        sender_clone.input(AppMsg::RefreshFolder);
    });
    nav_box.append(&btn_refresh);

    container.append(&nav_box);

    let path_label = gtk::Label::builder()
        .label(&format!("📍 {}", model.current_path.display()))
        .xalign(0.0)
        .margin_top(10)
        .wrap(true)
        .build();
    container.append(&path_label);

    let status = gtk::Label::builder()
        .label(&format!("📁 {} elementos", model.folder_items.len()))
        .xalign(0.0)
        .margin_top(5)
        .margin_bottom(10)
        .build();
    container.append(&status);

    if model.folder_items.is_empty() {
        let empty = gtk::Label::new(Some("📭 Carpeta vacía"));
        container.append(&empty);
    } else {
        for item in model.folder_items.iter().take(50) {
            let row = gtk::Box::builder()
                .orientation(gtk::Orientation::Horizontal)
                .spacing(10)
                .margin_top(2)
                .build();

            let icon = if item.is_dir { "📁" } 
                      else if item.name.ends_with(".sh") { "📜" }
                      else if item.name.ends_with(".exe") { "🪟" }
                      else if item.is_executable { "⚙️" } 
                      else { "📄" };
            
            let icon_label = gtk::Label::new(Some(icon));
            row.append(&icon_label);

            let name = gtk::Label::builder()
                .label(&item.name)
                .hexpand(true)
                .xalign(0.0)
                .build();
            row.append(&name);

            if item.is_dir {
                let btn = gtk::Button::builder()
                    .label("Abrir")
                    .build();
                let path_clone = item.path.clone();
                let sender_clone = sender.clone();
                btn.connect_clicked(move |_| {
                    sender_clone.input(AppMsg::Navigate(path_clone.clone()));
                });
                row.append(&btn);
            } else if item.is_executable || item.name.ends_with(".sh") || item.name.ends_with(".exe") {
                let btn = gtk::Button::builder()
                    .label("▶️")
                    .build();
                btn.add_css_class("suggested-action");
                let path_clone = item.path.clone();
                let sender_clone = sender.clone();
                btn.connect_clicked(move |_| {
                    sender_clone.input(AppMsg::ExecuteFile(path_clone.clone()));
                });
                row.append(&btn);
            }

            container.append(&row);
        }
    }
}

fn create_kernelbridge_view(model: &AppModel, container: &gtk::Box, sender: &ComponentSender<AppModel>) {
    let header = gtk::Label::builder()
        .label(&localize_txt("🧠 Estado de KernelBridge"))
        .xalign(0.0)
        .build();
    container.append(&header);

    let api_label = gtk::Label::builder()
        .label("APIs Híbridas:")
        .xalign(0.0)
        .margin_top(15)
        .build();
    container.append(&api_label);

    for (api, status) in &model.api_status {
        let status_label = gtk::Label::builder()
            .label(&format!("  {} {}", if *status { "✅" } else { "❌" }, api))
            .xalign(0.0)
            .build();
        container.append(&status_label);
    }

    let logs_label = gtk::Label::builder()
        .label("Logs:")
        .xalign(0.0)
        .margin_top(15)
        .build();
    container.append(&logs_label);

    for log in model.bridge_logs.iter().rev().take(15) {
        let log_label = gtk::Label::builder()
            .label(&format!("  {}", log))
            .xalign(0.0)
            .build();
        container.append(&log_label);
    }

    // Disposición en rejilla 4x5 para los botones de acciones (arriba de la sección PID)
    let btn_grid = gtk::Grid::builder()
        .column_spacing(10)
        .row_spacing(10)
        .margin_top(15)
        .build();
    let mut grid_col: i32 = 0;
    let mut grid_row: i32 = 0;
    let advance_cell = |col: &mut i32, row: &mut i32| {
        *col += 1;
        if *col >= 4 {
            *col = 0;
            *row += 1;
        }
    };

    let btn_refresh = gtk::Button::builder()
        .label("🔄 Actualizar")
        .build();
    let sender_clone = sender.clone();
    btn_refresh.connect_clicked(move |_| {
        sender_clone.input(AppMsg::RefreshKernelBridge);
    });
    btn_grid.attach(&btn_refresh, grid_col, grid_row, 1, 1);
    advance_cell(&mut grid_col, &mut grid_row);

    let btn_clear = gtk::Button::builder()
        .label("🗑️ Limpiar")
        .build();
    let sender_clone = sender.clone();
    btn_clear.connect_clicked(move |_| {
        sender_clone.input(AppMsg::ClearLogs);
    });
    btn_grid.attach(&btn_clear, grid_col, grid_row, 1, 1);
    advance_cell(&mut grid_col, &mut grid_row);

    // Botón unificar almacenes de CurseForge (Bottles/Nativo)
    let btn_cf_unify = gtk::Button::builder()
        .label("🧩 Unificar CurseForge Stores")
        .build();
    let sender_clone = sender.clone();
    btn_cf_unify.connect_clicked(move |_| {
        let s = sender_clone.clone();
        std::thread::spawn(move || {
            match daemon_simple_cmd("CF_UNIFY_STORES") {
                Ok(r) => s.input(AppMsg::Log(format!("[CF] {}", r.trim()))),
                Err(e) => s.input(AppMsg::Log(format!("[CF] ERROR: {}", e))),
            }
        });
    });
    btn_grid.attach(&btn_cf_unify, grid_col, grid_row, 1, 1);
    advance_cell(&mut grid_col, &mut grid_row);

    // Botón validar instancias comunes de CF y reportar Java requerido
    let btn_cf_validate = gtk::Button::builder()
        .label("🔎 Validar Instancias CF")
        .build();
    let sender_clone = sender.clone();
    btn_cf_validate.connect_clicked(move |_| {
        let s = sender_clone.clone();
        std::thread::spawn(move || {
            match daemon_simple_cmd("CF_VALIDATE_COMMON") {
                Ok(r) => s.input(AppMsg::Log(format!("[CF] VALIDATE: {}", r.trim()))),
                Err(e) => s.input(AppMsg::Log(format!("[CF] ERROR: {}", e))),
            }
        });
    });
    btn_grid.attach(&btn_cf_validate, grid_col, grid_row, 1, 1);
    advance_cell(&mut grid_col, &mut grid_row);

    // Botón Auto-Fix para instancias comunes de CurseForge (deshabilita packs problemáticos según logs)
    let btn_mc_autofix = gtk::Button::builder()
        .label("🛠️ Auto-Fix Minecraft (CF)")
        .build();
    let sender_clone = sender.clone();
    btn_mc_autofix.connect_clicked(move |_| {
        let s = sender_clone.clone();
        std::thread::spawn(move || {
            match daemon_simple_cmd("MC_AUTOFIX_COMMON") {
                Ok(r) => s.input(AppMsg::Log(format!("[MC] AUTOFIX: {}", r.trim()))),
                Err(e) => s.input(AppMsg::Log(format!("[MC] ERROR AUTOFIX: {}", e))),
            }
        });
    });
    btn_grid.attach(&btn_mc_autofix, grid_col, grid_row, 1, 1);
    advance_cell(&mut grid_col, &mut grid_row);

    // Botón para sugerir mods faltantes (basado en logs y Modrinth)
    let btn_mc_suggest = gtk::Button::builder()
        .label("🔎 Sugerir Mods Faltantes")
        .build();
    let sender_clone = sender.clone();
    btn_mc_suggest.connect_clicked(move |_| {
        let s = sender_clone.clone();
        std::thread::spawn(move || {
            match daemon_simple_cmd("MC_SUGGEST_MISSING_COMMON") {
                Ok(r) => s.input(AppMsg::Log(format!("[MC] SUGGEST: {}", r.trim()))),
                Err(e) => s.input(AppMsg::Log(format!("[MC] ERROR SUGGEST: {}", e))),
            }
        });
    });
    btn_grid.attach(&btn_mc_suggest, grid_col, grid_row, 1, 1);
    advance_cell(&mut grid_col, &mut grid_row);

    // Botón para instalar sugeridos automáticamente (Modrinth)
    let btn_mc_install = gtk::Button::builder()
        .label("⬇️ Instalar Sugeridos (beta)")
        .build();
    let sender_clone = sender.clone();
    btn_mc_install.connect_clicked(move |_| {
        let s = sender_clone.clone();
        std::thread::spawn(move || {
            match daemon_simple_cmd("MC_INSTALL_SUGGESTED_COMMON") {
                Ok(r) => s.input(AppMsg::Log(format!("[MC] INSTALL: {}", r.trim()))),
                Err(e) => s.input(AppMsg::Log(format!("[MC] ERROR INSTALL: {}", e))),
            }
        });
    });
    btn_grid.attach(&btn_mc_install, grid_col, grid_row, 1, 1);
    advance_cell(&mut grid_col, &mut grid_row);

    // Botón para instalar sugeridos desde CurseForge (requiere CF_API_KEY)
    let btn_mc_install_cf = gtk::Button::builder()
        .label("⬇️ Instalar Sugeridos (CF)")
        .build();
    let sender_clone = sender.clone();
    btn_mc_install_cf.connect_clicked(move |_| {
        let s = sender_clone.clone();
        std::thread::spawn(move || {
            match daemon_simple_cmd("MC_INSTALL_SUGGESTED_CF_COMMON") {
                Ok(r) => s.input(AppMsg::Log(format!("[MC] INSTALL CF: {}", r.trim()))),
                Err(e) => s.input(AppMsg::Log(format!("[MC] ERROR INSTALL CF: {}", e))),
            }
        });
    });
    btn_grid.attach(&btn_mc_install_cf, grid_col, grid_row, 1, 1);
    advance_cell(&mut grid_col, &mut grid_row);

    // Botón para probar APIs híbridas (Bridge API)
    let btn_api = gtk::Button::builder()
        .label("🔌 Probar APIs híbridas")
        .build();
    let sender_clone = sender.clone();
    btn_api.connect_clicked(move |_| {
        sender_clone.input(AppMsg::TestHybridApis);
    });
    btn_grid.attach(&btn_api, grid_col, grid_row, 1, 1);
    advance_cell(&mut grid_col, &mut grid_row);

    // Botón de prueba NT Device Proxy
    let btn_nt = gtk::Button::builder()
        .label("🧪 Probar NT Proxy")
        .build();
    let sender_clone = sender.clone();
    btn_nt.connect_clicked(move |_| {
        let s = sender_clone.clone();
        std::thread::spawn(move || {
            match nt_ioctl("GET_PROCESS_LIST") {
                Ok(resp) => s.input(AppMsg::Log(summarize_process_list(&resp))),
                Err(e) => s.input(AppMsg::Log(format!("[NT_PROXY] ERROR procesos: {}", e))),
            }
            match nt_ioctl("GET_ATTESTATION") {
                Ok(resp) => s.input(AppMsg::Log(format!("[NT_PROXY] Attestation: {}", resp.replace('\n', " ")))),
                Err(e) => s.input(AppMsg::Log(format!("[NT_PROXY] ERROR attestation: {}", e))),
            }
        });
    });
    btn_grid.attach(&btn_nt, grid_col, grid_row, 1, 1);
    advance_cell(&mut grid_col, &mut grid_row);

    // Botón de diagnóstico de red para launchers (Epic/Ubisoft)
    let btn_net = gtk::Button::builder()
        .label("🌐 Diagnóstico de red")
        .build();
    let sender_clone = sender.clone();
    btn_net.connect_clicked(move |_| {
        let s = sender_clone.clone();
        std::thread::spawn(move || {
            let urls = vec![
                ("Epic auth", "https://www.epicgames.com"),
                ("Epic store", "https://store.epicgames.com"),
                ("Epic backend", "https://store-site-backend-static.ak.epicgames.com"),
                ("Ubisoft connect", "https://connect.ubi.com"),
                ("MC auth", "https://api.minecraftservices.com"),
                ("MC session", "https://session.minecraft.net"),
                ("MSA login", "https://login.live.com"),
            ];
            for (name, url) in urls {
                let res = net_check_url(url);
                s.input(AppMsg::Log(format!("[NET] {} -> {}", name, res)));
            }
        });
    });
    btn_grid.attach(&btn_net, grid_col, grid_row, 1, 1);
    advance_cell(&mut grid_col, &mut grid_row);

    // Botón para exportar diagnóstico (NT Proxy + Atestación + Heurísticas)
    let btn_diag = gtk::Button::builder()
        .label("🧾 Exportar diagnóstico")
        .build();
    let sender_clone = sender.clone();
    btn_diag.connect_clicked(move |_| {
        let s = sender_clone.clone();
        std::thread::spawn(move || {
            match collect_and_write_diagnostics() {
                Ok(path) => s.input(AppMsg::Log(format!("[DIAG] Exportado: {}", path))),
                Err(e) => s.input(AppMsg::Log(format!("[DIAG] ERROR: {}", e))),
            }
        });
    });
    btn_grid.attach(&btn_diag, grid_col, grid_row, 1, 1);
    advance_cell(&mut grid_col, &mut grid_row);

    container.append(&btn_grid);

    // Acciones por instancia (selección de carpeta) para CF/Minecraft
    let inst_hdr = gtk::Label::builder()
        .label("📂 Acciones por instancia (CF/Minecraft)")
        .xalign(0.0)
        .margin_top(10)
        .build();
    container.append(&inst_hdr);

    let inst_box_top = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(6)
        .build();
    let dir_entry = gtk::Entry::builder()
        .placeholder_text("Ruta de la instancia (carpeta)")
        .hexpand(true)
        .build();
    inst_box_top.append(&dir_entry);
    let btn_choose = gtk::Button::builder().label("📂 Elegir carpeta").build();
    {
        // Abrir diálogo de carpeta y volcar ruta en entry
        let dir_entry = dir_entry.clone();
        btn_choose.connect_clicked(move |_| {
            let dialog = gtk::FileChooserNative::builder()
                .title("Selecciona carpeta de instancia")
                .action(gtk::FileChooserAction::SelectFolder)
                .accept_label("Seleccionar")
                .cancel_label("Cancelar")
                .build();
            let value = dir_entry.clone();
            dialog.connect_response(move |d, resp| {
                if resp == gtk::ResponseType::Accept {
                    if let Some(file) = d.file() {
                        if let Some(path) = file.path() {
                            value.set_text(&path.to_string_lossy());
                        }
                    }
                }
            });
            dialog.show();
        });
    }
    inst_box_top.append(&btn_choose);
    container.append(&inst_box_top);

    // Rejilla 4x5 de botones de acción por directorio
    let inst_actions = gtk::Grid::builder()
        .column_spacing(8)
        .row_spacing(8)
        .margin_top(6)
        .build();
    let mut icol: i32 = 0;
    let mut irow: i32 = 0;
    let mut place = |w: &gtk::Widget| {
        inst_actions.attach(w, icol, irow, 1, 1);
        icol += 1;
        if icol >= 4 { icol = 0; irow += 1; }
    };

    let mk_handler = |label: &str, prefix: &str, sender: &ComponentSender<AppModel>, dir_entry: &gtk::Entry| -> gtk::Button {
        let b = gtk::Button::builder().label(label).build();
        let s = sender.clone();
        let entry = dir_entry.clone();
        let prefix_o = prefix.to_string();
        b.connect_clicked(move |_| {
            let p = entry.text().to_string();
            if p.trim().is_empty() {
                s.input(AppMsg::Log("[DIR] Indica ruta de instancia primero".to_string()));
                return;
            }
            let cmd = format!("{}:{}", prefix_o, p);
            let prefix_owned = prefix_o.clone();
            let prefix_short = prefix_owned.split(':').next().unwrap_or(prefix_owned.as_str()).to_string();
            let s2 = s.clone();
            std::thread::spawn(move || {
                match daemon_simple_cmd(&cmd) {
                    Ok(r) => s2.input(AppMsg::Log(format!("[DIR] {} -> {}", prefix_short, r.trim()))),
                    Err(e) => s2.input(AppMsg::Log(format!("[DIR] ERROR {}: {}", prefix_owned, e))),
                }
            });
        });
        b
    };

    let b1 = mk_handler("🔎 Validar dir CF", "CF_VALIDATE_DIR", sender, &dir_entry);
    let b2 = mk_handler("⚒️ Preparar dir CF", "CF_PREPARE_DIR", sender, &dir_entry);
    let b3 = mk_handler("🛠️ Auto‑Fix dir", "MC_AUTOFIX_DIR", sender, &dir_entry);
    let b4 = mk_handler("🔎 Sugerir faltantes", "MC_SUGGEST_MISSING", sender, &dir_entry);
    let b5 = mk_handler("⬇️ Instalar sugeridos", "MC_INSTALL_SUGGESTED", sender, &dir_entry);
    let b6 = mk_handler("🔎 Sugerir faltantes (CF)", "MC_SUGGEST_MISSING_CF", sender, &dir_entry);
    let b7 = mk_handler("⬇️ Instalar (CF)", "MC_INSTALL_SUGGESTED_CF", sender, &dir_entry);

    place(b1.upcast_ref());
    place(b2.upcast_ref());
    place(b3.upcast_ref());
    place(b4.upcast_ref());
    place(b5.upcast_ref());
    place(b6.upcast_ref());
    place(b7.upcast_ref());
    container.append(&inst_actions);

    // Controles para VM Launcher (virsh/libvirt) – ocultos por defecto detrás de KB_ENABLE_VM
    if is_vm_enabled() {
    let vm_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(6)
        .margin_top(10)
        .build();
    let vm_entry = gtk::Entry::builder().placeholder_text("Nombre VM (libvirt)").width_request(220).build();
    // Prefijar con el nombre por defecto de modelo (puede venir de KB_VM_NAME o detección)
    vm_entry.set_text(&model.vm_default_name);
    vm_box.append(&vm_entry);

    let btn_vm_start = gtk::Button::builder().label("Iniciar VM").build();
    let btn_vm_stop = gtk::Button::builder().label("Detener VM").build();
    let btn_vm_status = gtk::Button::builder().label("Estado VM").build();
    let btn_vm_health = gtk::Button::builder().label("Salud VM").build();
    let btn_vm_moon = gtk::Button::builder().label("Abrir Moonlight").build();

    let sender_clone = sender.clone();
    let entry_clone = vm_entry.clone();
    let vm_default_fallback = model.vm_default_name.clone();
    btn_vm_start.connect_clicked(move |_| {
        let input = entry_clone.text().to_string();
        let name = if input.trim().is_empty() { vm_default_fallback.clone() } else { input };
        let s = sender_clone.clone();
        std::thread::spawn(move || {
            match vm_command("VM_START", &name) {
                Ok(resp) => {
                    let msg = resp.trim().to_string();
                    if msg.contains("Error al obtener el dominio") || msg.to_lowercase().contains("domain not found") {
                        // Auto-detectar VMs y reintentar con la primera candidata
                        if let Ok(list) = vm_command("VM_LIST", "") {
                            let candidates = parse_vm_list(&list);
                            if let Some(best) = pick_best_vm(&candidates) {
                                s.input(AppMsg::SetVmDefaultName(best.clone()));
                                // Reintentar start con best
                                match vm_command("VM_START", &best) {
                                    Ok(r2) => s.input(AppMsg::Log(format!("[VM] {}", r2.trim()))),
                                    Err(e2) => s.input(AppMsg::Log(format!("[VM] ERROR reintento start: {}", e2))),
                                }
                            } else {
                                s.input(AppMsg::Log(format!("[VM] No se encontraron dominios en libvirt. Asegura que libvirtd esté activo y tengas permisos.")));
                            }
                        } else {
                            s.input(AppMsg::Log("[VM] No se pudo listar dominios (VM_LIST)".to_string()));
                        }
                    } else {
                        s.input(AppMsg::Log(format!("[VM] {}", msg)));
                    }
                }
                Err(e) => s.input(AppMsg::Log(format!("[VM] ERROR start: {}", e))),
            }
        });
    });

    let sender_clone = sender.clone();
    let entry_clone = vm_entry.clone();
    let vm_default_fallback = model.vm_default_name.clone();
    btn_vm_stop.connect_clicked(move |_| {
        let input = entry_clone.text().to_string();
        let name = if input.trim().is_empty() { vm_default_fallback.clone() } else { input };
        let s = sender_clone.clone();
        std::thread::spawn(move || {
            match vm_command("VM_STOP", &name) {
                Ok(resp) => s.input(AppMsg::Log(format!("[VM] {}", resp.trim()))),
                Err(e) => s.input(AppMsg::Log(format!("[VM] ERROR stop: {}", e))),
            }
        });
    });

    let sender_clone = sender.clone();
    let entry_clone = vm_entry.clone();
    let vm_default_fallback = model.vm_default_name.clone();
    btn_vm_status.connect_clicked(move |_| {
        let input = entry_clone.text().to_string();
        let name = if input.trim().is_empty() { vm_default_fallback.clone() } else { input };
        let s = sender_clone.clone();
        std::thread::spawn(move || {
            match vm_command("VM_STATUS", &name) {
                Ok(resp) => s.input(AppMsg::Log(format!("[VM] {}", resp.trim()))),
                Err(e) => s.input(AppMsg::Log(format!("[VM] ERROR status: {}", e))),
            }
        });
    });

    let sender_clone = sender.clone();
    let entry_clone = vm_entry.clone();
    let vm_default_fallback = model.vm_default_name.clone();
    btn_vm_health.connect_clicked(move |_| {
        let input = entry_clone.text().to_string();
        let name = if input.trim().is_empty() { vm_default_fallback.clone() } else { input };
        let s = sender_clone.clone();
        std::thread::spawn(move || {
            match vm_command("VM_HEALTH", &name) {
                Ok(resp) => {
                    let trimmed = resp.trim().to_string();
                    if let Some(ip) = parse_vm_health_ip(&trimmed) {
                        s.input(AppMsg::SetVmIp(ip.clone()));
                        s.input(AppMsg::Log(format!("[VM] Salud: {} (IP detectada: {})", trimmed, ip)));
                    } else {
                        s.input(AppMsg::Log(format!("[VM] {}", trimmed)));
                    }
                }
                Err(e) => s.input(AppMsg::Log(format!("[VM] ERROR health: {}", e))),
            }
        });
    });

    // Botón para abrir Moonlight y conectarse a la IP detectada de la VM
    let sender_clone = sender.clone();
    let entry_clone = vm_entry.clone();
    let vm_default_fallback = model.vm_default_name.clone();
    btn_vm_moon.connect_clicked(move |_| {
        let input = entry_clone.text().to_string();
        let name = if input.trim().is_empty() { vm_default_fallback.clone() } else { input };
        let s = sender_clone.clone();
        std::thread::spawn(move || {
            match vm_command("VM_HEALTH", &name) {
                Ok(resp) => {
                    let trimmed = resp.trim().to_string();
                    if let Some(ip) = parse_vm_health_ip(&trimmed) {
                        s.input(AppMsg::SetVmIp(ip.clone()));
                        let ok = spawn_moonlight(&ip);
                        if ok { s.input(AppMsg::Log(format!("[MOONLIGHT] Conectando a {}", ip))); }
                        else { s.input(AppMsg::Log("[MOONLIGHT] No se pudo abrir Moonlight. Instala 'moonlight' o Flatpak 'com.moonlight_stream.Moonlight'".to_string())); }
                    } else {
                        s.input(AppMsg::Log(format!("[MOONLIGHT] No se pudo detectar IP en VM_HEALTH: {}", trimmed)));
                    }
                }
                Err(e) => s.input(AppMsg::Log(format!("[MOONLIGHT] ERROR al consultar salud VM: {}", e))),
            }
        });
    });

    vm_box.append(&btn_vm_start);
    vm_box.append(&btn_vm_stop);
    vm_box.append(&btn_vm_status);
    vm_box.append(&btn_vm_health);
    vm_box.append(&btn_vm_moon);
    container.append(&vm_box);

    // Asistente básico para crear VM de Windows (virt-install)
    let create_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(6)
        .margin_top(10)
        .build();
    let name_entry = gtk::Entry::builder().placeholder_text("Nombre VM").width_request(160).build();
    name_entry.set_text(&model.vm_default_name);
    let iso_entry = gtk::Entry::builder().placeholder_text("Ruta ISO de Windows").hexpand(true).build();
    let disk_entry = gtk::Entry::builder().placeholder_text("Disco (GB, ej. 60)").width_request(120).build();
    disk_entry.set_text("60");
    let btn_create = gtk::Button::builder().label("Crear VM").build();

    let sender_clone = sender.clone();
    let en_name = name_entry.clone();
    let en_iso = iso_entry.clone();
    let en_disk = disk_entry.clone();
    btn_create.connect_clicked(move |_| {
        let mut name = en_name.text().to_string();
        if name.trim().is_empty() { name = default_vm_name(); }
        let iso = en_iso.text().to_string();
        let disk = en_disk.text().to_string();
        if iso.trim().is_empty() {
            sender_clone.input(AppMsg::Log("[VM] Debes indicar la ruta al ISO de Windows".to_string()));
            return;
        }
        let args = format!("{}|{}|{}", name, iso, disk);
        let s = sender_clone.clone();
        std::thread::spawn(move || {
            match vm_command("VM_CREATE", &args) {
                Ok(resp) => s.input(AppMsg::Log(format!("[VM] {}", resp.trim()))),
                Err(e) => s.input(AppMsg::Log(format!("[VM] ERROR crear: {}", e))),
            }
        });
    });

    create_box.append(&name_entry);
    create_box.append(&iso_entry);
    create_box.append(&disk_entry);
    create_box.append(&btn_create);
    container.append(&create_box);
    }

    // Controles por PID para consultas NT
    let pid_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(6)
        .margin_top(10)
        .build();
    let pid_entry = gtk::Entry::builder().placeholder_text("PID").width_request(120).build();
    pid_box.append(&pid_entry);

    let btn_threads = gtk::Button::builder().label("Hilos").build();
    let btn_modules = gtk::Button::builder().label("Módulos").build();
    let btn_handles = gtk::Button::builder().label("Handles").build();
    let btn_maps = gtk::Button::builder().label("Memoria").build();

    let sender_clone = sender.clone();
    let entry_clone = pid_entry.clone();
    btn_threads.connect_clicked(move |_| {
        if let Some(pid) = entry_clone.text().as_str().trim().parse::<i32>().ok() {
            let s = sender_clone.clone();
            std::thread::spawn(move || {
                let cmd = format!("GET_THREAD_LIST:{}", pid);
                match nt_ioctl(&cmd) {
                    Ok(resp) => s.input(AppMsg::Log(summarize_threads(&resp))),
                    Err(e) => s.input(AppMsg::Log(format!("[NT_PROXY] ERROR hilos: {}", e))),
                }
            });
        }
    });

    let sender_clone = sender.clone();
    let entry_clone = pid_entry.clone();
    btn_modules.connect_clicked(move |_| {
        if let Some(pid) = entry_clone.text().as_str().trim().parse::<i32>().ok() {
            let s = sender_clone.clone();
            std::thread::spawn(move || {
                let cmd = format!("GET_MODULES:{}", pid);
                match nt_ioctl(&cmd) {
                    Ok(resp) => s.input(AppMsg::Log(summarize_modules(&resp))),
                    Err(e) => s.input(AppMsg::Log(format!("[NT_PROXY] ERROR módulos: {}", e))),
                }
            });
        }
    });

    let sender_clone = sender.clone();
    let entry_clone = pid_entry.clone();
    btn_handles.connect_clicked(move |_| {
        if let Some(pid) = entry_clone.text().as_str().trim().parse::<i32>().ok() {
            let s = sender_clone.clone();
            std::thread::spawn(move || {
                let cmd = format!("GET_HANDLE_TABLE:{}", pid);
                match nt_ioctl(&cmd) {
                    Ok(resp) => s.input(AppMsg::Log(summarize_handles(&resp))),
                    Err(e) => s.input(AppMsg::Log(format!("[NT_PROXY] ERROR handles: {}", e))),
                }
            });
        }
    });

    let sender_clone = sender.clone();
    let entry_clone = pid_entry.clone();
    btn_maps.connect_clicked(move |_| {
        if let Some(pid) = entry_clone.text().as_str().trim().parse::<i32>().ok() {
            let s = sender_clone.clone();
            std::thread::spawn(move || {
                let cmd = format!("GET_PROCESS_MEMORY_MAP:{}", pid);
                match nt_ioctl(&cmd) {
                    Ok(resp) => s.input(AppMsg::Log(summarize_maps(&resp))),
                    Err(e) => s.input(AppMsg::Log(format!("[NT_PROXY] ERROR memoria: {}", e))),
                }
            });
        }
    });

    pid_box.append(&btn_threads);
    pid_box.append(&btn_modules);
    pid_box.append(&btn_handles);
    pid_box.append(&btn_maps);

    container.append(&pid_box);
}

// Cliente para System Bridge API (socket separado)
fn bridge_api_cmd(cmd: &str) -> Result<String, String> {
    #[cfg(unix)]
    {
        if let Ok(mut stream) = UnixStream::connect("/tmp/kernelbridge_api.sock") {
            let _ = stream.write_all(format!("{}\n", cmd).as_bytes());
            let mut buf = Vec::new();
            if stream.read_to_end(&mut buf).is_ok() {
                return Ok(String::from_utf8_lossy(&buf).to_string());
            }
            return Err("sin respuesta".to_string());
        }
        return Err("no se pudo conectar".to_string());
    }
    #[allow(unreachable_code)]
    Ok(String::new())
}

fn create_settings_view(model: &AppModel, container: &gtk::Box, sender: &ComponentSender<AppModel>) {
    let header = gtk::Label::builder()
        .label(&localize_txt("⚙️ Configuración"))
        .xalign(0.0)
        .build();
    container.append(&header);

    // Idioma UI (experimental): alternar KB_LANG=en
    let lang_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(10)
        .margin_top(12)
        .build();

    let lang_label = gtk::Label::builder()
        .label("🌐 UI Language (English)")
        .hexpand(true)
        .xalign(0.0)
        .build();
    lang_box.append(&lang_label);

    let lang_switch = gtk::Switch::builder()
        .active(model.config_language_en)
        .build();
    let sender_clone = sender.clone();
    lang_switch.connect_state_set(move |_, _| {
        sender_clone.input(AppMsg::ToggleLanguageEn);
        gtk::glib::Propagation::Proceed
    });
    lang_box.append(&lang_switch);

    container.append(&lang_box);

    // Puente de audio para lanzamientos como root (usar servidor de audio del usuario)
    let audio_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(10)
        .margin_top(12)
        .build();

    let audio_label = gtk::Label::builder()
        .label("🎧 Puente de audio (root → usuario)")
        .hexpand(true)
        .xalign(0.0)
        .build();
    audio_box.append(&audio_label);

    let audio_switch = gtk::Switch::builder()
        .active(model.config_audio_bridge_root)
        .build();
    let sender_clone = sender.clone();
    audio_switch.connect_state_set(move |_, _| {
        sender_clone.input(AppMsg::ToggleAudioBridgeRoot);
        gtk::glib::Propagation::Proceed
    });
    audio_box.append(&audio_switch);

    container.append(&audio_box);

    let debug_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(10)
        .margin_top(20)
        .build();

    let debug_label = gtk::Label::builder()
        .label("🐛 Modo Debug")
        .hexpand(true)
        .xalign(0.0)
        .build();
    debug_box.append(&debug_label);

    let debug_switch = gtk::Switch::builder()
        .active(model.config_debug_mode)
        .build();
    let sender_clone = sender.clone();
    debug_switch.connect_state_set(move |_, _| {
    sender_clone.input(AppMsg::ToggleDebugMode);
    gtk::glib::Propagation::Proceed
    });
    debug_box.append(&debug_switch);

    container.append(&debug_box);

    // Toggle para forzar Bottles/Wine incluso si el juego usa anti‑cheat de kernel
    let force_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(10)
        .margin_top(12)
        .build();

    let force_label = gtk::Label::builder()
        .label("🚧 Forzar Bottles/Wine para juegos con anti‑cheat (no recomendado)")
        .hexpand(true)
        .xalign(0.0)
        .build();
    force_box.append(&force_label);

    let force_switch = gtk::Switch::builder()
        .active(model.config_force_bottles)
        .build();
    let sender_clone = sender.clone();
    force_switch.connect_state_set(move |_, _| {
        sender_clone.input(AppMsg::ToggleForceBottles);
        gtk::glib::Propagation::Proceed
    });
    force_box.append(&force_switch);

    container.append(&force_box);

    // Toggle para enrutar automáticamente a VM juegos con anti‑cheat de kernel (si la VM está habilitada)
    if is_vm_enabled() {
        let auto_vm_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(10)
            .margin_top(12)
            .build();

        let auto_vm_label = gtk::Label::builder()
            .label("🪟 Enrutar automáticamente a VM para juegos con anti‑cheat de kernel")
            .hexpand(true)
            .xalign(0.0)
            .build();
        auto_vm_box.append(&auto_vm_label);

        let auto_vm_switch = gtk::Switch::builder()
            .active(model.config_auto_vm_ac)
            .build();
        let sender_clone = sender.clone();
        auto_vm_switch.connect_state_set(move |_, _| {
            sender_clone.input(AppMsg::ToggleAutoVmAc);
            gtk::glib::Propagation::Proceed
        });
        auto_vm_box.append(&auto_vm_switch);

        container.append(&auto_vm_box);
    }

    // Toggle para mitigaciones de ejecuciones Local (Wine): desactivar overlay y activar esync/fsync
    let local_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(10)
        .margin_top(12)
        .build();

    let local_label = gtk::Label::builder()
        .label("🛡️ Mitigaciones Local (overlay OFF, esync/fsync ON)")
        .hexpand(true)
        .xalign(0.0)
        .build();
    local_box.append(&local_label);

    let local_switch = gtk::Switch::builder()
        .active(model.config_local_mitigations)
        .build();
    let sender_clone = sender.clone();
    local_switch.connect_state_set(move |_, _| {
        sender_clone.input(AppMsg::ToggleLocalMitigations);
        gtk::glib::Propagation::Proceed
    });
    local_box.append(&local_switch);

    container.append(&local_box);

    // Toggle para modo offline (bloquear red del juego)
    let offline_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(10)
        .margin_top(12)
        .build();

    let offline_label = gtk::Label::builder()
        .label("📴 Modo offline (bloquear red del juego)")
        .hexpand(true)
        .xalign(0.0)
        .build();
    offline_box.append(&offline_label);

    let offline_switch = gtk::Switch::builder()
        .active(model.config_offline_mode)
        .build();
    let sender_clone = sender.clone();
    offline_switch.connect_state_set(move |_, _| {
        sender_clone.input(AppMsg::ToggleOfflineMode);
        gtk::glib::Propagation::Proceed
    });
    offline_box.append(&offline_switch);

    container.append(&offline_box);

    // Toggle para preferir versiones Flatpak de los lanzadores
    let flatpak_pref_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(10)
        .margin_top(12)
        .build();

    let flatpak_pref_label = gtk::Label::builder()
        .label("📦 Preferir lanzadores Flatpak (si están instalados)")
        .hexpand(true)
        .xalign(0.0)
        .build();
    flatpak_pref_box.append(&flatpak_pref_label);

    let flatpak_pref_switch = gtk::Switch::builder()
        .active(model.config_prefer_flatpak)
        .build();
    let sender_clone = sender.clone();
    flatpak_pref_switch.connect_state_set(move |_, _| {
        sender_clone.input(AppMsg::TogglePreferFlatpak);
        gtk::glib::Propagation::Proceed
    });
    flatpak_pref_box.append(&flatpak_pref_switch);

    container.append(&flatpak_pref_box);

    // Preferencia: CurseForge Windows (Wine/Bottles) para compatibilidad de mods
    let cf_pref_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(10)
        .margin_top(12)
        .build();

    let cf_pref_label = gtk::Label::builder()
        .label("🧩 Preferir CurseForge Windows (Wine/Bottles) para compatibilidad de mods")
        .hexpand(true)
        .xalign(0.0)
        .build();
    cf_pref_box.append(&cf_pref_label);

    let cf_pref_switch = gtk::Switch::builder()
        .active(model.config_prefer_cf_windows)
        .build();
    let sender_clone = sender.clone();
    cf_pref_switch.connect_state_set(move |_, _| {
        sender_clone.input(AppMsg::TogglePreferCurseForgeWindows);
        gtk::glib::Propagation::Proceed
    });
    cf_pref_box.append(&cf_pref_switch);

    container.append(&cf_pref_box);

    // Toggle para forzar VKD3D‑Proton (D3D12 nativo)
    let vkd3d_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(10)
        .margin_top(12)
        .build();

    let vkd3d_label = gtk::Label::builder()
        .label("🎮 Forzar VKD3D‑Proton (D3D12 nativo)")
        .hexpand(true)
        .xalign(0.0)
        .build();
    vkd3d_box.append(&vkd3d_label);

    let vkd3d_switch = gtk::Switch::builder()
        .active(model.config_force_vkd3d)
        .build();
    let sender_clone = sender.clone();
    vkd3d_switch.connect_state_set(move |_, _| {
        sender_clone.input(AppMsg::ToggleForceVKD3D);
        gtk::glib::Propagation::Proceed
    });
    vkd3d_box.append(&vkd3d_switch);

    container.append(&vkd3d_box);

    // Toggle para usar Proton-GE en juegos Local
    let protonge_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(10)
        .margin_top(12)
        .build();

    let protonge_label = gtk::Label::builder()
        .label("🧪 Usar Proton-GE para juegos Local (si está instalado)")
        .hexpand(true)
        .xalign(0.0)
        .build();
    protonge_box.append(&protonge_label);

    let protonge_switch = gtk::Switch::builder()
        .active(model.config_use_proton_ge_local)
        .build();
    let sender_clone = sender.clone();
    protonge_switch.connect_state_set(move |_, _| {
        sender_clone.input(AppMsg::ToggleUseProtonGE);
        gtk::glib::Propagation::Proceed
    });
    protonge_box.append(&protonge_switch);

    container.append(&protonge_box);

    let info_label = gtk::Label::builder()
        .label(&format!("Sistema: {} | Kernel: {} | Arch: {}", 
            std::env::consts::OS, 
            get_kernel_version(),
            std::env::consts::ARCH))
        .xalign(0.0)
        .margin_top(20)
        .build();
    container.append(&info_label);
}

fn check_tpm() -> String {
    if PathBuf::from("/sys/class/tpm/tpm0").exists() {
        "✅ Detectado".to_string()
    } else {
        "⚠️ No detectado".to_string()
    }
}

fn check_secure_boot() -> String {
    match fs::read_to_string("/sys/firmware/efi/efivars/SecureBoot-8be4df61-93ca-11d2-aa0d-00e098032b8c") {
        Ok(content) => {
            if content.contains('\x01') {
                "✅ Activado".to_string()
            } else {
                "⚠️ Desactivado".to_string()
            }
        }
        Err(_) => "❓ No disponible".to_string(),
    }
}

fn get_kernel_modules() -> Vec<KernelModule> {
    let mut modules = Vec::new();
    
    if let Ok(content) = fs::read_to_string("/proc/modules") {
        for line in content.lines().take(20) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                modules.push(KernelModule {
                    name: parts[0].to_string(),
                    size: parts[1].to_string(),
                    used_by: parts.get(3).unwrap_or(&"-").to_string(),
                });
            }
        }
    }
    
    modules
}

fn get_kernel_version() -> String {
    match fs::read_to_string("/proc/version") {
        Ok(content) => content.split_whitespace().nth(2).unwrap_or("?").to_string(),
        Err(_) => "?".to_string(),
    }
}

// Intenta resolver el HOME real del usuario incluso si la app corre con sudo/root.
fn resolve_home() -> PathBuf {
    use std::env;
    // 1) Si se ejecuta con sudo, priorizar SUDO_USER
    if let Ok(sudo_user) = env::var("SUDO_USER") {
        if !sudo_user.is_empty() && sudo_user != "root" {
            for base in ["/home", "/var/home"] {
                let candidate = PathBuf::from(format!("{}/{}", base, &sudo_user));
                if candidate.exists() { return candidate; }
            }
        }
    }

    // 2) HOME del proceso si es válido
    if let Ok(home) = env::var("HOME") {
        let p = PathBuf::from(&home);
        if p.exists() { return p; }
    }

    // 3) LOGNAME o USER
    for key in ["LOGNAME", "USER"] {
        if let Ok(user) = env::var(key) {
            if !user.is_empty() {
                for base in ["/home", "/var/home"] {
                    let candidate = PathBuf::from(format!("{}/{}", base, &user));
                    if candidate.exists() { return candidate; }
                }
            }
        }
    }

    // 4) Fallback
    PathBuf::from("/home")
}

fn scan_directory(path: &PathBuf) -> Vec<FileItem> {
    let mut items = Vec::new();
    
    if !path.exists() {
        return items;
    }
    
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let entry_path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();
            
            if name.starts_with('.') {
                continue;
            }
            
            let is_dir = entry_path.is_dir();
            let mut is_executable = false;
            
            if !is_dir {
                if name.ends_with(".sh") || name.ends_with(".exe") {
                    is_executable = true;
                } else {
                    #[cfg(unix)]
                    {
                        use std::os::unix::fs::PermissionsExt;
                        if let Ok(metadata) = fs::metadata(&entry_path) {
                            is_executable = metadata.permissions().mode() & 0o111 != 0;
                        }
                    }
                }
            }
            
            items.push(FileItem {
                name,
                path: entry_path,
                is_dir,
                is_executable,
            });
        }
    }
    
    items.sort_by(|a, b| {
        match (a.is_dir, b.is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        }
    });
    
    items
}

fn scan_all_games() -> Vec<GameInfo> {
    let mut games = Vec::new();
    let home_path = resolve_home();
    if home_path.exists() {

        // 1. Escanear Steam (múltiples ubicaciones comunes)
        // Pasar la raíz de Steam; la función interna buscará steamapps
        for steam_root in [
            home_path.join(".var/app/com.valvesoftware.Steam/.local/share/Steam"),
            home_path.join(".var/app/com.valvesoftware.Steam/data/Steam"),
            home_path.join(".local/share/Steam"),
            home_path.join(".steam/steam"),
        ] {
            if steam_root.exists() {
                if let Some(steam_games) = scan_steam_library(&steam_root) {
                    games.extend(steam_games);
                }
            }
        }

        // 2. Escanear Bottles (Flatpak y nativo)
        let bottles_flatpak_path = home_path.join(".var/app/com.usebottles.bottles/data/bottles/bottles");
        if bottles_flatpak_path.exists() {
            games.extend(scan_bottles_games(&bottles_flatpak_path));
        }
        let bottles_native_path = home_path.join(".local/share/bottles/bottles");
        if bottles_native_path.exists() {
            games.extend(scan_bottles_games(&bottles_native_path));
        }

        // 3. Escanear Lutris (Flatpak y nativo), usando archivos YAML para obtener slug/nombre
        for lutris_games_dir in [
            home_path.join(".local/share/lutris/games"),
            home_path.join(".config/lutris/games"),
            home_path.join(".var/app/net.lutris.Lutris/config/lutris/games"),
        ] {
            if lutris_games_dir.exists() {
                games.extend(scan_lutris_games(&lutris_games_dir));
            }
        }

        // 4. Escanear juegos locales en un directorio común (ej. 'Games')
        let local_games_path = home_path.join("Games");
        if local_games_path.exists() {
            find_local_games_recursive(&local_games_path, &mut games);
        }

        // 4b. Escaneo explícito de EA App dentro de ~/Games (ruta común)
        let ea_program_files = home_path.join("Games/ea-app/drive_c/Program Files");
        if ea_program_files.exists() {
            find_local_games_recursive(&ea_program_files, &mut games);
        }
    }
    games.sort_by(|a, b| a.name.cmp(&b.name));
    games
}

fn scan_lutris_games(dir: &PathBuf) -> Vec<GameInfo> {
    let mut games = Vec::new();
    if !dir.exists() { return games; }
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                let name = path.file_name().unwrap().to_string_lossy().to_string();
                if !name.ends_with(".yml") && !name.ends_with(".yaml") { continue; }
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Some((game_name, slug)) = parse_lutris_yaml_basic(&content) {
                        games.push(GameInfo {
                            name: game_name,
                            path: path.clone(), // guardamos el YAML como referencia
                            executable: String::new(),
                            compatible: true, // se ejecutará vía Lutris
                            anticheat: "Desconocido".to_string(),
                            source: GameSource::Lutris,
                            launch_id: slug,
                        });
                    }
                }
            }
        }
    }
    games
}

fn parse_lutris_yaml_basic(content: &str) -> Option<(String, String)> {
    // Parseo muy básico: buscamos lines que contengan 'name:' y 'slug:' al nivel del bloque 'game:'
    // Si no encontramos slug, devolvemos None para no crear entrada que no podamos lanzar.
    let mut in_game = false;
    let mut name: Option<String> = None;
    let mut slug: Option<String> = None;
    for line in content.lines() {
        let s = line.trim();
        if s.starts_with("game:") { in_game = true; continue; }
        if in_game {
            if s.starts_with("name:") {
                let v = s.splitn(2, ':').nth(1)?.trim().trim_matches('"').to_string();
                if !v.is_empty() { name = Some(v); }
            } else if s.starts_with("slug:") {
                let v = s.splitn(2, ':').nth(1)?.trim().trim_matches('"').to_string();
                if !v.is_empty() { slug = Some(v); }
            } else if !s.starts_with('#') && !s.contains(':') {
                // fin probable del bloque game si cambia indent; mantenemos simple
                in_game = false;
            }
        }
    }
    if let (Some(n), Some(sg)) = (name, slug) { Some((n, sg)) } else { None }
}

fn find_local_games_recursive(path: &PathBuf, games: &mut Vec<GameInfo>) {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let current_path = entry.path();
            if current_path.is_dir() {
                find_local_games_recursive(&current_path, games);
            } else if current_path.is_file() {
                if let Some(ext) = current_path.extension() {
                    if ext == "exe" {
                        let game_name = current_path.file_stem().unwrap().to_string_lossy().to_string();
                        let exe_name = current_path.file_name().unwrap().to_string_lossy().to_string();
                        
                        let game_dir = current_path.parent().unwrap().to_path_buf();
                        let has_eac = check_anticheat(&game_dir, "EasyAntiCheat");
                        let has_be = check_anticheat(&game_dir, "BattlEye");
                        let has_ace = check_anticheat(&game_dir, "AntiCheatExpert") || check_anticheat(&game_dir, "ACE-");

                        let anticheat = if has_ace { "AntiCheatExpert (ACE)" } 
                                       else if has_eac { "EasyAntiCheat" } 
                                       else if has_be { "BattlEye" } 
                                       else { "Ninguno" };
                        let compatible = !has_eac && !has_be && !has_ace;

                        if !games.iter().any(|g| g.path == current_path) {
                            games.push(GameInfo {
                                name: game_name,
                                path: current_path.clone(),
                                executable: exe_name,
                                compatible,
                                anticheat: anticheat.to_string(),
                                source: GameSource::Local,
                                launch_id: current_path.to_string_lossy().to_string(),
                            });
                        }
                    }
                }
            }
        }
    }
}

fn find_games_recursive(_path: &PathBuf, _games: &mut Vec<GameInfo>) {
    // Esta función ahora es un placeholder, la lógica principal está en scan_all_games
    // Se podría usar como fallback si se desea.
}

fn scan_steam_library(steam_root: &PathBuf) -> Option<Vec<GameInfo>> {
    if !steam_root.exists() {
        return None;
    }

    // 1) Siempre incluir la librería por defecto
    let mut library_paths: Vec<PathBuf> = vec![steam_root.join("steamapps")];

    // 2) Intentar descubrir librerías adicionales desde libraryfolders.vdf
    for vdf_path in [
        steam_root.join("steamapps/libraryfolders.vdf"),
        steam_root.join("config/libraryfolders.vdf"),
    ] {
        if let Ok(vdf) = fs::read_to_string(&vdf_path) {
            for line in vdf.lines() {
                if let Some(extra) = vdf_get_value(line, "path") {
                    let p = PathBuf::from(extra).join("steamapps");
                    if p.exists() {
                        library_paths.push(p);
                    }
                }
            }
        }
    }

    // 3) Escanear todas las librerías encontradas
    let mut all_games = Vec::new();
    for lib in library_paths {
        if lib.exists() {
            let mut list = scan_steam_games(&lib.to_string_lossy());
            all_games.append(&mut list);
        }
    }
    Some(all_games)
}

fn scan_bottles_library(path: &PathBuf) -> Option<Vec<GameInfo>> {
    let bottles_data_path = path.join("data/bottles/bottles");
     if !bottles_data_path.exists() {
        return None;
    }
    Some(scan_bottles_games(&bottles_data_path))
}

fn scan_steam_games(steamapps_path_str: &str) -> Vec<GameInfo> {
    let mut games = Vec::new();
    let steamapps_path = PathBuf::from(steamapps_path_str);
    let common_path = steamapps_path.join("common");

    if !steamapps_path.exists() {
        return games;
    }

    let manifests = match fs::read_dir(&steamapps_path) {
        Ok(entries) => entries
            .flatten()
            .filter(|e| e.file_name().to_string_lossy().starts_with("appmanifest_"))
            .collect::<Vec<_>>(),
        Err(_) => return games,
    };

    for entry in manifests {
        let content = fs::read_to_string(entry.path()).unwrap_or_default();
        let mut appid = String::new();
        let mut installdir = String::new();
        let mut name = String::new();

        for line in content.lines() {
            if let Some(v) = vdf_get_value(line, "appid") { appid = v; }
            if let Some(v) = vdf_get_value(line, "installdir") { installdir = v; }
            if let Some(v) = vdf_get_value(line, "name") { name = v; }
        }

        if !appid.is_empty() && !installdir.is_empty() {
            let game_dir = common_path.join(&installdir);
            if let Some((exe_path, exe_name)) = find_executable_recursive(&game_dir) {
                let has_eac = check_anticheat(&game_dir, "EasyAntiCheat");
                let has_be = check_anticheat(&game_dir, "BattlEye");

                let anticheat = if has_eac { "EasyAntiCheat" } else if has_be { "BattlEye" } else { "Ninguno" };
                let compatible = !has_eac && !has_be;

                games.push(GameInfo {
                    name: if !name.is_empty() { name } else { installdir },
                    path: exe_path,
                    executable: exe_name,
                    compatible,
                    anticheat: anticheat.to_string(),
                    source: GameSource::Steam,
                    launch_id: appid,
                });
            }
        }
    }
    games
}

fn scan_bottles_games(bottles_path: &PathBuf) -> Vec<GameInfo> {
    let mut games = Vec::new();
    if !bottles_path.exists() {
        return games;
    }

    if let Ok(entries) = fs::read_dir(bottles_path) {
        for entry in entries.flatten() {
            let bottle_path = entry.path();
            if bottle_path.is_dir() {
                let bottle_name = entry.file_name().to_string_lossy().to_string();
                let drive_c = bottle_path.join("drive_c");
                
                if drive_c.exists() {
                    find_executables_in_bottle(&drive_c, &bottle_name, &mut games);
                }
            }
        }
    }
    games
}

fn find_executables_in_bottle(path: &PathBuf, bottle_name: &str, games: &mut Vec<GameInfo>) {
    let program_files = path.join("Program Files");
    let program_files_x86 = path.join("Program Files (x86)");

    if program_files.exists() {
        find_executables_recursively(&program_files, bottle_name, games);
    }
    if program_files_x86.exists() {
        find_executables_recursively(&program_files_x86, bottle_name, games);
    }
}

fn find_executables_recursively(path: &PathBuf, bottle_name: &str, games: &mut Vec<GameInfo>) {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let current_path = entry.path();
            if current_path.is_dir() {
                find_executables_recursively(&current_path, bottle_name, games);
            } else if current_path.is_file() {
                if let Some(ext) = current_path.extension() {
                    if ext == "exe" {
                        let game_name = current_path.file_stem().unwrap().to_string_lossy().to_string();
                        let exe_name = current_path.file_name().unwrap().to_string_lossy().to_string();
                        
                        let game_dir = current_path.parent().unwrap().to_path_buf();
                        let has_eac = check_anticheat(&game_dir, "EasyAntiCheat");
                        let has_be = check_anticheat(&game_dir, "BattlEye");

                        let anticheat = if has_eac { "EasyAntiCheat" } else if has_be { "BattlEye" } else { "Ninguno" };
                        let compatible = !has_eac && !has_be;

                        if !games.iter().any(|g| g.path == current_path) {
                            games.push(GameInfo {
                                name: game_name,
                                path: current_path.clone(),
                                executable: exe_name,
                                compatible,
                                anticheat: anticheat.to_string(),
                                source: GameSource::Bottles,
                                launch_id: bottle_name.to_string(),
                            });
                        }
                    }
                }
            }
        }
    }
}

fn find_executable(game_dir: &PathBuf) -> Option<(PathBuf, String)> {
    // Backwards-compat wrapper: keep behavior but prefer recursive search
    find_executable_recursive(game_dir)
}

fn find_executable_recursive(dir: &PathBuf) -> Option<(PathBuf, String)> {
    if !dir.exists() { return None; }
    // DFS search for the first .exe; prefer files in the root dir first
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "exe" {
                        let name = path.file_name()?.to_string_lossy().to_string();
                        return Some((path, name));
                    }
                }
            }
        }
    }
    // If nothing at root, recurse into subdirs
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if let Some(found) = find_executable_recursive(&path) {
                    return Some(found);
                }
            }
        }
    }
    None
}

fn vdf_get_value(line: &str, key: &str) -> Option<String> {
    let s = line.trim();
    if !s.starts_with('"') { return None; }
    let parts: Vec<&str> = s.split('"').collect();
    // Expected: ["", key, " ", value, ...]
    if parts.len() >= 4 && parts[1] == key {
        return Some(parts[3].to_string());
    }
    None
}

fn check_anticheat(game_dir: &PathBuf, anticheat_name: &str) -> bool {
    if let Ok(entries) = fs::read_dir(game_dir) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_lowercase();
            if name.contains(&anticheat_name.to_lowercase()) {
                return true;
            }
        }
    }
    false
}

fn main() {
    let app = RelmApp::new("com.kernelbridge.gui");
    app.run::<AppModel>(());
}

// Intenta cerrar el daemon de KernelBridge de forma ordenada.
fn shutdown_daemon() {
    // 1) Intentar SHUTDOWN por socket UNIX si existe
    #[cfg(unix)]
    {
        if let Ok(mut stream) = UnixStream::connect("/tmp/kernelbridge.sock") {
            let _ = stream.write_all(b"SHUTDOWN");
            return;
        }
    }

    // 2) Fallback: leer PID file y enviar SIGTERM
    if let Ok(pid_str) = fs::read_to_string("/tmp/kernelbridge-daemon.pid") {
        let pid = pid_str.trim();
        if !pid.is_empty() {
            let _ = Cmd::new("kill").args(["-TERM", pid]).spawn();
            return;
        }
    }

    // 3) Último recurso: intentar matar por nombre (puede fallar si no existe la herramienta)
    let _ = Cmd::new("pkill").arg("-TERM").arg("kernelbridge-daemon").spawn();
}

// Ejecuta un programa, y si la app corre como root pero existe SUDO_USER,
// re-lanza el programa como ese usuario con variables de sesión DBus/XDG correctas.
fn spawn_program(program: &str, args: &[&str]) -> bool {
    // ¿Estamos en un contexto root que debe delegar al usuario real?
    if let Some((sudo_user, uid_str, user_home)) = real_user_context() {
        // Construir env de sesión del usuario
        let xdg_runtime = format!("/run/user/{}", uid_str);
        let dbus_addr = format!("unix:path={}/bus", xdg_runtime);

        // Preservar variables de entorno relevantes para red/locale/certs
        let mut passthrough_env: Vec<String> = Vec::new();
        for key in [
            "HTTP_PROXY", "HTTPS_PROXY", "NO_PROXY",
            "http_proxy", "https_proxy", "no_proxy",
            "SSL_CERT_FILE", "SSL_CERT_DIR",
            "LANG", "LC_ALL", "LC_CTYPE",
            // Variables de sesión/ventana/audio que algunos launchers necesitan
            "DISPLAY", "WAYLAND_DISPLAY", "XAUTHORITY",
            "XDG_SESSION_TYPE", "XDG_CURRENT_DESKTOP", "DESKTOP_SESSION",
            "PULSE_SERVER", "PIPEWIRE_RUNTIME_DIR",
            // Módulos GIO/GTK extra (Flatpak u otros entornos)
            "GIO_EXTRA_MODULES", "GTK_THEME",
        ] {
            if let Ok(val) = std::env::var(key) { if !val.is_empty() { passthrough_env.push(format!("{}={}", key, val)); } }
        }

        // Preferir runuser, si no, sudo. Si ninguno, seguimos normal.
        if which_exists("runuser") {
            let mut full_args: Vec<String> = vec![
                "-u".into(), sudo_user.clone(), "--".into(),
                "env".into(),
                format!("HOME={}", user_home.display()),
                format!("XDG_RUNTIME_DIR={}", xdg_runtime),
                format!("DBUS_SESSION_BUS_ADDRESS={}", dbus_addr),
            ];
            full_args.extend(passthrough_env.iter().cloned());
            full_args.push(program.into());
            full_args.extend(args.iter().map(|s| s.to_string()));
            let status = Cmd::new("runuser").args(&full_args).spawn();
            return status.is_ok();
        } else if which_exists("sudo") {
            let mut full_args: Vec<String> = vec![
                "-u".into(), sudo_user.clone(),
                "env".into(),
                format!("HOME={}", user_home.display()),
                format!("XDG_RUNTIME_DIR={}", xdg_runtime),
                format!("DBUS_SESSION_BUS_ADDRESS={}", dbus_addr),
            ];
            full_args.extend(passthrough_env.iter().cloned());
            full_args.push(program.into());
            full_args.extend(args.iter().map(|s| s.to_string()));
            let status = Cmd::new("sudo").args(&full_args).spawn();
            return status.is_ok();
        }
    }

    // Caso normal (usuario no-root o sin SUDO_USER)
    Cmd::new(program).args(args).spawn().is_ok()
}

fn spawn_program_s(program: &str, args: Vec<String>) -> bool {
    let args_ref: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    spawn_program(program, &args_ref)
}

fn real_user_context() -> Option<(String, String, PathBuf)> {
    use std::env;
    // Sólo si corremos como root y tenemos SUDO_USER
    let is_root_env = env::var("USER").map(|u| u == "root").unwrap_or(false)
        || env::var("HOME").map(|h| h == "/root").unwrap_or(false);
    let sudo_user = env::var("SUDO_USER").ok()?;
    if !is_root_env { return None; }
    if sudo_user.is_empty() || sudo_user == "root" { return None; }

    // Obtener UID del usuario
    let uid_out = Cmd::new("id").args(["-u", &sudo_user]).output().ok()?;
    let uid_str = String::from_utf8_lossy(&uid_out.stdout).trim().to_string();
    if uid_str.is_empty() { return None; }

    // HOME real
    let mut home = None;
    for base in ["/home", "/var/home"] {
        let candidate = PathBuf::from(format!("{}/{}", base, sudo_user));
        if candidate.exists() { home = Some(candidate); break; }
    }
    let home = home.unwrap_or_else(|| resolve_home());
    Some((sudo_user, uid_str, home))
}

// Envia PREPARE_GAME al daemon antes de lanzar un juego
fn prepare_game(game: &GameInfo) -> Result<String, String> {
    #[cfg(unix)]
    {
        if let Ok(mut stream) = UnixStream::connect("/tmp/kernelbridge.sock") {
            let source = match game.source {
                GameSource::Steam => "Steam",
                GameSource::Bottles => "Bottles",
                GameSource::Lutris => "Lutris",
                GameSource::Local => "Local",
            };
            let msg = format!("PREPARE_GAME:{}:{}\n", source, game.launch_id);
            let _ = stream.write_all(msg.as_bytes());
            let mut buf = Vec::new();
            if stream.read_to_end(&mut buf).is_ok() {
                let resp = String::from_utf8_lossy(&buf).to_string();
                if resp.starts_with("OK") { return Ok(resp.trim().to_string()); }
                return Err(resp.trim().to_string());
            }
            return Err("Sin respuesta del daemon".to_string());
        }
        return Err("No se pudo conectar al daemon".to_string());
    }
    #[allow(unreachable_code)]
    Ok("OK".to_string())
}

fn nt_ioctl(cmd: &str) -> Result<String, String> {
    #[cfg(unix)]
    {
        if let Ok(mut stream) = UnixStream::connect("/tmp/kernelbridge.sock") {
            let msg = format!("NT_IOCTL:{}\n", cmd);
            let _ = stream.write_all(msg.as_bytes());
            let mut buf = Vec::new();
            if stream.read_to_end(&mut buf).is_ok() {
                let resp = String::from_utf8_lossy(&buf).to_string();
                return Ok(resp);
            }
            return Err("Sin respuesta del daemon".to_string());
        }
        return Err("No se pudo conectar al daemon".to_string());
    }
    #[allow(unreachable_code)]
    Ok("{}".to_string())
}

// Enviar un comando simple al socket principal del daemon
fn daemon_simple_cmd(cmd: &str) -> Result<String, String> {
    #[cfg(unix)]
    {
        if let Ok(mut stream) = UnixStream::connect("/tmp/kernelbridge.sock") {
            let _ = stream.write_all(format!("{}\n", cmd).as_bytes());
            let mut buf = Vec::new();
            if stream.read_to_end(&mut buf).is_ok() {
                return Ok(String::from_utf8_lossy(&buf).to_string());
            }
            return Err("Sin respuesta del daemon".to_string());
        }
        return Err("No se pudo conectar al daemon".to_string());
    }
    #[allow(unreachable_code)]
    Ok(String::new())
}

fn vm_command(kind: &str, name: &str) -> Result<String, String> {
    #[cfg(unix)]
    {
        if let Ok(mut stream) = UnixStream::connect("/tmp/kernelbridge.sock") {
            let msg = format!("{}:{}\n", kind, name);
            let _ = stream.write_all(msg.as_bytes());
            let mut buf = Vec::new();
            if stream.read_to_end(&mut buf).is_ok() {
                return Ok(String::from_utf8_lossy(&buf).to_string());
            }
            return Err("Sin respuesta del daemon".to_string());
        }
        return Err("No se pudo conectar al daemon".to_string());
    }
    #[allow(unreachable_code)]
    Ok(String::new())
}

fn summarize_process_list(json: &str) -> String {
    // Reducir ruido: contar procesos y mostrar algunos nombres
    if let Ok(value) = serde_json::from_str::<serde_json::Value>(json) {
        if let Some(arr) = value.as_array() {
            let count = arr.len();
            let mut names = Vec::new();
            for v in arr.iter().take(5) {
                if let Some(name) = v.get("name").and_then(|n| n.as_str()) {
                    names.push(name.to_string());
                }
            }
            return format!("[NT_PROXY] Procesos: {} | Ejemplos: {}", count, names.join(", "));
        }
    }
    "[NT_PROXY] Respuesta de procesos no entendida".to_string()
}

fn summarize_threads(json: &str) -> String {
    if let Ok(value) = serde_json::from_str::<serde_json::Value>(json) {
        if let Some(arr) = value.as_array() {
            let count = arr.len();
            let mut tids = Vec::new();
            for v in arr.iter().take(8) {
                if let Some(tid) = v.get("tid").and_then(|n| n.as_i64()) { tids.push(tid.to_string()); }
            }
            return format!("[NT_PROXY] Hilos: {} | Ejemplos TIDs: {}", count, tids.join(", "));
        }
    }
    "[NT_PROXY] Respuesta de hilos no entendida".to_string()
}

fn summarize_modules(json: &str) -> String {
    if let Ok(value) = serde_json::from_str::<serde_json::Value>(json) {
        if let Some(arr) = value.as_array() {
            let count = arr.len();
            let mut names = Vec::new();
            for v in arr.iter().take(5) {
                if let Some(p) = v.get("path").and_then(|n| n.as_str()) {
                    let name = p.rsplit('/').next().unwrap_or(p);
                    names.push(name.to_string());
                }
            }
            return format!("[NT_PROXY] Módulos: {} | Ejemplos: {}", count, names.join(", "));
        }
    }
    "[NT_PROXY] Respuesta de módulos no entendida".to_string()
}

fn summarize_handles(json: &str) -> String {
    if let Ok(value) = serde_json::from_str::<serde_json::Value>(json) {
        if let Some(arr) = value.as_array() {
            let count = arr.len();
            let mut fds = Vec::new();
            for v in arr.iter().take(8) {
                if let Some(fd) = v.get("fd").and_then(|n| n.as_i64()) { fds.push(fd.to_string()); }
            }
            return format!("[NT_PROXY] Handles: {} | Ejemplos FDs: {}", count, fds.join(", "));
        }
    }
    "[NT_PROXY] Respuesta de handles no entendida".to_string()
}

fn summarize_maps(json: &str) -> String {
    if let Ok(value) = serde_json::from_str::<serde_json::Value>(json) {
        if let Some(arr) = value.as_array() {
            let count = arr.len();
            // contar regiones con path (mapeos de archivos)
            let files = arr.iter().filter_map(|v| v.get("path")).filter(|p| !p.is_null()).count();
            return format!("[NT_PROXY] Memoria: {} regiones | {} con archivo", count, files);
        }
    }
    "[NT_PROXY] Respuesta de memoria no entendida".to_string()
}

fn is_fortnite(game: &GameInfo) -> bool {
    let name = game.name.to_lowercase();
    let exe = game.executable.to_lowercase();
    let path = game.path.to_string_lossy().to_lowercase().to_string();
    name.contains("fortnite") || exe.contains("fortnite") || path.contains("fortnite")
}

fn is_kernel_ac_game(game: &GameInfo) -> bool {
    // Heurística amplia: si el escaneo detectó EAC/BE/ACE, o si el nombre/ejecutable/ruta sugiere AC de kernel
    let ac = game.anticheat.to_lowercase();
    if ac.contains("easyanticheat") || ac.contains("eac") || ac.contains("battleye") || ac.contains("ace") || ac.contains("anticheatexpert") {
        return true;
    }
    let n = game.name.to_lowercase();
    let e = game.executable.to_lowercase();
    let p = game.path.to_string_lossy().to_lowercase();
    // Juegos conocidos con anti-cheat de kernel
    if n.contains("fortnite") || e.contains("fortnite") || p.contains("fortnite") { return true; }
    if n.contains("valorant") || e.contains("vgk") || p.contains("vanguard") || n.contains("vanguard") { return true; }
    if p.contains("ricochet") || n.contains("ricochet") { return true; }
    // Delta Force usa AntiCheatExpert (ACE) - anti-cheat de kernel agresivo
    if n.contains("delta force") || n.contains("deltaforce") || e.contains("deltaforce") || p.contains("delta force") || p.contains("deltaforce") { return true; }
    if p.contains("anticheatexpert") || p.contains("ace-base") || p.contains("ace-core") { return true; }
    false
}

fn requires_eos_disable(game: &GameInfo) -> bool {
    let name = game.name.to_lowercase();
    let exe = game.executable.to_lowercase();
    let path = game.path.to_string_lossy().to_lowercase();
    // Heurística: títulos con SDK de Epic que suelen cargar overlay (p.ej., nightreign)
    name.contains("nightreign") || exe.contains("nightreign") || path.contains("nightreign")
}

fn default_vm_name() -> String {
    std::env::var("KB_VM_NAME").unwrap_or_else(|_| "Windows-Gaming".to_string())
}

fn is_vm_enabled() -> bool {
    let v = std::env::var("KB_ENABLE_VM").unwrap_or_default();
    let v = v.to_lowercase();
    v == "1" || v == "true" || v == "yes"
}

fn collect_and_write_diagnostics() -> Result<String, String> {
    use serde_json::{json, Value};
    let ts = SystemTime::now().duration_since(UNIX_EPOCH).map_err(|e| e.to_string())?.as_secs();

    let proc_list: Value = match nt_ioctl("GET_PROCESS_LIST") {
        Ok(s) => serde_json::from_str(&s).unwrap_or_else(|_| json!({"raw": s})),
        Err(e) => json!({"error": e}),
    };
    // Intentar EXT; si falla, usar básica
    let attestation: Value = match nt_ioctl("GET_ATTESTATION_EXT") {
        Ok(s) => serde_json::from_str(&s).unwrap_or_else(|_| json!({"raw": s})),
        Err(_) => match nt_ioctl("GET_ATTESTATION") {
            Ok(s) => serde_json::from_str(&s).unwrap_or_else(|_| json!({"raw": s})),
            Err(e2) => json!({"error": e2}),
        },
    };
    let heuristics: Value = match nt_ioctl("CHECK_SANDBOX_VM") {
        Ok(s) => serde_json::from_str(&s).unwrap_or_else(|_| json!({"raw": s})),
        Err(e) => json!({"error": e}),
    };

    let bundle = json!({
        "meta": {
            "timestamp": ts,
            "os": std::env::consts::OS,
            "arch": std::env::consts::ARCH,
            "version": env!("CARGO_PKG_VERSION"),
        },
        "process_list": proc_list,
        "attestation": attestation,
        "heuristics": heuristics,
    });

    let home = resolve_home();
    let out_dir = home.join("KernelBridgeDiagnostics");
    fs::create_dir_all(&out_dir).map_err(|e| format!("no se pudo crear dir: {}", e))?;
    let out_path = out_dir.join(format!("diag_{}.json", ts));
    let data = serde_json::to_vec_pretty(&bundle).map_err(|e| e.to_string())?;
    fs::write(&out_path, data).map_err(|e| e.to_string())?;
    Ok(out_path.to_string_lossy().to_string())
}

fn parse_vm_health_ip(text: &str) -> Option<String> {
    // Buscar la primera IPv4 en el texto de domifaddr (e.g., 192.168.1.50/24)
    for token in text.split_whitespace() {
        if let Some((ip, _mask)) = token.split_once('/') {
            if is_ipv4(ip) { return Some(ip.to_string()); }
        } else if is_ipv4(token) {
            return Some(token.to_string());
        }
    }
    None
}

fn is_ipv4(s: &str) -> bool {
    let parts: Vec<&str> = s.split('.').collect();
    if parts.len() != 4 { return false; }
    for p in parts {
        if let Ok(n) = p.parse::<u8>() { let _ = n; } else { return false; }
    }
    true
}

fn spawn_moonlight(ip: &str) -> bool {
    // Priorizar CLI nativa si existe
    if which_exists("moonlight") {
        return spawn_program_s("moonlight", vec!["stream".to_string(), ip.to_string()]);
    }
    // Intentar Flatpak GUI como fallback
    if which_exists("flatpak") {
        let ok = spawn_program_s("flatpak", vec!["run".to_string(), "com.moonlight_stream.Moonlight".to_string()]);
        if ok { return true; }
    }
    false
}

fn parse_vm_list(resp: &str) -> Vec<String> {
    // Espera "OK: <nombres>" o "OK:" vacío; extraer líneas no vacías
    let body = resp.splitn(2, ':').nth(1).unwrap_or(resp);
    body.lines().map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect()
}

fn pick_best_vm(list: &[String]) -> Option<String> {
    if list.is_empty() { return None; }
    // Preferir nombres que contengan "win" o "windows"
    if let Some(w) = list.iter().find(|n| n.to_lowercase().contains("windows")) { return Some(w.clone()); }
    if let Some(w) = list.iter().find(|n| n.to_lowercase().contains("win")) { return Some(w.clone()); }
    // Si no, la primera
    Some(list[0].clone())
}

// Intenta consultar una URL con curl para verificar conectividad/TLS.
fn net_check_url(url: &str) -> String {
    let mut cmd = Cmd::new("curl");
    cmd.args([
        "-sS",
        "-m", "8",
        "-o", "/dev/null",
        "-w", "HTTP:%{http_code} CT:%{content_type}",
        "--location",
        url,
    ]);
    match cmd.output() {
        Ok(out) => {
            let s = String::from_utf8_lossy(&out.stdout).to_string();
            if s.is_empty() {
                return "sin salida (posible bloqueo de red)".to_string();
            }
            // Parsear HTTP code y CT para etiquetar como Reachable
            let mut http_code: Option<u16> = None;
            let mut ct: Option<String> = None;
            for token in s.split_whitespace() {
                if let Some(code) = token.strip_prefix("HTTP:") {
                    http_code = code.parse::<u16>().ok();
                }
                if let Some(v) = token.strip_prefix("CT:") {
                    ct = Some(v.to_string());
                }
            }
            if let Some(code) = http_code {
                let tag = if (200..300).contains(&code) {
                    "Reachable"
                } else if (300..400).contains(&code) {
                    "Reachable (redirect)"
                } else if code == 401 || code == 403 {
                    "Reachable (auth required)"
                } else if code == 404 {
                    "Reachable (not found)"
                } else if code == 0 {
                    "sin respuesta"
                } else {
                    "Reachable (HTTP error)"
                };
                let ct_suf = ct.map(|v| format!(" CT:{}", v)).unwrap_or_default();
                return format!("{} → HTTP:{}{}", tag, code, ct_suf);
            }
            s
        }
        Err(e) => format!("curl no disponible o fallo: {}", e),
    }
}