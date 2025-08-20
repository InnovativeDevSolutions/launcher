use serde::{Deserialize, Serialize};
use std::fs;
use tauri::{AppHandle, Manager};

#[derive(Serialize, Deserialize, Clone)]
struct AppSettings {
    arma3_directory: String,
    steam_library_path: String,
    custom_mods_directory: String,
    server_address: String,
    server_password: String,
    battle_eye_enabled: bool,
    startup_parameters: String,
    auto_update_enabled: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            arma3_directory: String::new(),
            steam_library_path: String::new(),
            custom_mods_directory: String::new(),
            server_address: "127.0.0.1:2302".to_string(),
            server_password: String::new(),
            battle_eye_enabled: true, // Default to enabled
            startup_parameters: String::new(),
            auto_update_enabled: false, // Default to disabled for safety
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ModPreset {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub mods: Vec<String>, // Mod paths
    pub created_at: String,
    pub updated_at: String,
}

impl ModPreset {
    pub fn new(name: String, description: Option<String>, mods: Vec<String>) -> Self {
        let now = chrono::Utc::now()
            .format("%Y-%m-%d %H:%M:%S UTC")
            .to_string();
        let id = uuid::Uuid::new_v4().to_string();

        Self {
            id,
            name,
            description,
            mods,
            created_at: now.clone(),
            updated_at: now,
        }
    }
}

#[tauri::command]
fn query_server(address: &str) -> String {
    get_server_info(address)
}

#[tauri::command]
fn select_directory(app_handle: tauri::AppHandle) -> Result<Option<String>, String> {
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;
    use tauri_plugin_dialog::DialogExt;

    let result = Arc::new(Mutex::new(None));
    let result_clone = Arc::clone(&result);

    app_handle
        .dialog()
        .file()
        .set_title("Select Arma 3 Root Directory")
        .pick_folder(move |folder_path| {
            let mut result = result_clone.lock().unwrap();
            *result = folder_path.map(|p| p.to_string());
        });

    // Wait for the dialog to complete (simple polling approach)
    for _ in 0..100 {
        // Wait up to 10 seconds
        thread::sleep(Duration::from_millis(100));
        let result_guard = result.lock().unwrap();
        if result_guard.is_some() {
            return Ok(result_guard.clone());
        }
    }

    Ok(None)
}

fn get_settings_path(app_handle: &AppHandle) -> Result<std::path::PathBuf, String> {
    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;

    // Create the directory if it doesn't exist
    if !app_data_dir.exists() {
        fs::create_dir_all(&app_data_dir)
            .map_err(|e| format!("Failed to create app data directory: {}", e))?;
    }

    Ok(app_data_dir.join("settings.json"))
}

fn get_presets_path(app_handle: &AppHandle) -> Result<std::path::PathBuf, String> {
    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;

    // Create the directory if it doesn't exist
    if !app_data_dir.exists() {
        fs::create_dir_all(&app_data_dir)
            .map_err(|e| format!("Failed to create app data directory: {}", e))?;
    }

    Ok(app_data_dir.join("mod_presets.json"))
}

fn get_mod_registry_path(app_handle: &AppHandle) -> Result<std::path::PathBuf, String> {
    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;

    // Create the directory if it doesn't exist
    if !app_data_dir.exists() {
        fs::create_dir_all(&app_data_dir)
            .map_err(|e| format!("Failed to create app data directory: {}", e))?;
    }

    Ok(app_data_dir.join("mod_registry.json"))
}

#[tauri::command]
fn save_settings(app_handle: AppHandle, settings: AppSettings) -> Result<(), String> {
    let settings_path = get_settings_path(&app_handle)?;
    let settings_json = serde_json::to_string_pretty(&settings)
        .map_err(|e| format!("Failed to serialize settings: {}", e))?;

    fs::write(&settings_path, settings_json)
        .map_err(|e| format!("Failed to write settings file: {}", e))?;

    Ok(())
}

#[tauri::command]
async fn load_settings(app_handle: AppHandle) -> Result<AppSettings, String> {
    let settings_path = get_settings_path(&app_handle)?;

    if !settings_path.exists() {
        return Ok(AppSettings::default());
    }

    let settings_content = tokio::fs::read_to_string(&settings_path)
        .await
        .map_err(|e| format!("Failed to read settings file: {}", e))?;

    let settings: AppSettings = serde_json::from_str(&settings_content)
        .map_err(|e| format!("Failed to parse settings: {}", e))?;

    Ok(settings)
}

mod mods;
use mods::*;

#[tauri::command]
async fn scan_mods(
    arma3_directory: String,
    steam_library_path: Option<String>,
    custom_mods_directory: Option<String>,
) -> Result<Vec<ModInfo>, String> {
    // Run the potentially slow mod scanning operation in a separate thread
    tokio::task::spawn_blocking(move || {
        mods::scan_mods(arma3_directory, steam_library_path, custom_mods_directory)
    })
    .await
    .map_err(|e| format!("Failed to spawn mod scanning task: {}", e))?
}

// Mod Preset Management Commands

#[tauri::command]
fn save_mod_preset(
    app_handle: AppHandle,
    name: String,
    description: Option<String>,
    mod_paths: Vec<String>,
) -> Result<ModPreset, String> {
    let presets_path = get_presets_path(&app_handle)?;

    // Load existing presets
    let mut presets: Vec<ModPreset> = if presets_path.exists() {
        let presets_content = fs::read_to_string(&presets_path)
            .map_err(|e| format!("Failed to read presets file: {}", e))?;
        serde_json::from_str(&presets_content)
            .map_err(|e| format!("Failed to parse presets: {}", e))?
    } else {
        Vec::new()
    };

    // Create new preset
    let new_preset = ModPreset::new(name, description, mod_paths);
    presets.push(new_preset.clone());

    // Use an immutable reference to serialize the presets
    let presets_json = serde_json::to_string_pretty(&presets.clone())
        .map_err(|e| format!("Failed to serialize presets: {}", e))?;

    fs::write(&presets_path, presets_json)
        .map_err(|e| format!("Failed to write presets file: {}", e))?;

    Ok(new_preset)
}

#[tauri::command]
async fn load_mod_presets(app_handle: AppHandle) -> Result<Vec<ModPreset>, String> {
    let presets_path = get_presets_path(&app_handle)?;

    if !presets_path.exists() {
        return Ok(Vec::new());
    }

    let presets_content = tokio::fs::read_to_string(&presets_path)
        .await
        .map_err(|e| format!("Failed to read presets file: {}", e))?;

    let presets: Vec<ModPreset> = serde_json::from_str(&presets_content)
        .map_err(|e| format!("Failed to parse presets: {}", e))?;

    Ok(presets)
}

#[tauri::command]
fn update_mod_preset(
    app_handle: AppHandle,
    preset_id: String,
    name: Option<String>,
    description: Option<String>,
    mod_paths: Option<Vec<String>>,
) -> Result<ModPreset, String> {
    let presets_path = get_presets_path(&app_handle)?;

    if !presets_path.exists() {
        return Err("No presets file found".to_string());
    }

    let presets_content = fs::read_to_string(&presets_path)
        .map_err(|e| format!("Failed to read presets file: {}", e))?;

    let mut presets: Vec<ModPreset> = serde_json::from_str(&presets_content)
        .map_err(|e| format!("Failed to parse presets: {}", e))?;

    // Find preset index first
    let preset_index = presets
        .iter()
        .position(|p| p.id == preset_id)
        .ok_or("Preset not found")?;

    // Update the preset
    {
        let preset = &mut presets[preset_index];

        if let Some(new_name) = name {
            preset.name = new_name;
        }
        if let Some(new_description) = description {
            preset.description = Some(new_description);
        }
        if let Some(new_mod_paths) = mod_paths {
            preset.mods = new_mod_paths;
        }

        // Update timestamp
        preset.updated_at = chrono::Utc::now()
            .format("%Y-%m-%d %H:%M:%S UTC")
            .to_string();
    }

    // Save updated presets
    let presets_json = serde_json::to_string_pretty(&presets)
        .map_err(|e| format!("Failed to serialize presets: {}", e))?;

    fs::write(&presets_path, presets_json)
        .map_err(|e| format!("Failed to write presets file: {}", e))?;

    // Return a clone of the updated preset
    Ok(presets[preset_index].clone())
}

#[tauri::command]
fn delete_mod_preset(app_handle: AppHandle, preset_id: String) -> Result<(), String> {
    let presets_path = get_presets_path(&app_handle)?;

    if !presets_path.exists() {
        return Err("No presets file found".to_string());
    }

    let presets_content = fs::read_to_string(&presets_path)
        .map_err(|e| format!("Failed to read presets file: {}", e))?;

    let mut presets: Vec<ModPreset> = serde_json::from_str(&presets_content)
        .map_err(|e| format!("Failed to parse presets: {}", e))?;

    // Remove the preset
    let original_len = presets.len();
    presets.retain(|p| p.id != preset_id);

    if presets.len() == original_len {
        return Err("Preset not found".to_string());
    }

    // Save updated presets
    let presets_json = serde_json::to_string_pretty(&presets)
        .map_err(|e| format!("Failed to serialize presets: {}", e))?;

    fs::write(&presets_path, presets_json)
        .map_err(|e| format!("Failed to write presets file: {}", e))?;

    Ok(())
}

#[tauri::command]
fn apply_mod_preset(preset: ModPreset) -> Result<Vec<String>, String> {
    // This command returns the mod paths from the preset so the frontend can apply them
    Ok(preset.mods)
}

mod app_updater;
mod launcher;
mod mod_updater;

use app_updater::*;
use mod_updater::*;

#[tauri::command]
async fn update_single_mod(
    app_handle: AppHandle,
    arma3_directory: String,
    mod_info: ModUpdateInfo,
) -> Result<String, String> {
    let registry_path = get_mod_registry_path(&app_handle)?;
    match mod_updater::download_and_install_mod(
        &registry_path,
        &arma3_directory,
        &mod_info,
        Some(app_handle),
    )
    .await
    {
        Ok(()) => {
            let action = if mod_info.is_new_mod {
                "installed"
            } else {
                "updated"
            };
            Ok(format!(
                "{} {} to version {}",
                action, mod_info.mod_name, mod_info.remote_version
            ))
        }
        Err(e) => Err(e),
    }
}

#[tauri::command]
async fn update_all_mods(
    app_handle: AppHandle,
    arma3_directory: String,
) -> Result<Vec<String>, String> {
    let registry_path = get_mod_registry_path(&app_handle)?;
    mod_updater::update_all_mods(&registry_path, &arma3_directory, Some(app_handle)).await
}

#[tauri::command]
async fn fetch_remote_mod_versions(
) -> Result<std::collections::HashMap<String, RemoteModVersion>, String> {
    mod_updater::fetch_remote_mod_versions().await
}

#[tauri::command]
async fn check_mod_updates_with_notifications(
    app_handle: AppHandle,
    arma3_directory: String,
) -> Result<Vec<ModUpdateInfo>, String> {
    let registry_path = get_mod_registry_path(&app_handle)?;
    mod_updater::check_mod_updates_with_notifications(
        &registry_path,
        &arma3_directory,
        Some(app_handle),
    )
    .await
}

#[tauri::command]
fn launch_arma3_enhanced(
    arma3_directory: String,
    server_address: Option<String>,
    server_password: Option<String>,
    enabled_mods: Vec<ModInfo>,
    startup_parameters: String,
    battle_eye_enabled: bool,
) -> Result<String, String> {
    launcher::launch_arma3_enhanced(
        arma3_directory,
        server_address,
        server_password,
        enabled_mods,
        startup_parameters,
        battle_eye_enabled,
    )
}

#[tauri::command]
fn launch_arma3_with_mods(
    arma3_directory: String,
    server_address: Option<String>,
    server_password: Option<String>,
    mod_paths: Vec<String>,
    startup_parameters: String,
    battle_eye_enabled: bool,
) -> Result<String, String> {
    launcher::launch_arma3_with_mods(
        arma3_directory,
        server_address,
        server_password,
        mod_paths,
        startup_parameters,
        battle_eye_enabled,
    )
}

#[tauri::command]
fn launch_arma3(
    arma3_directory: String,
    server_address: Option<String>,
    server_password: Option<String>,
    battle_eye_enabled: bool,
) -> Result<String, String> {
    launcher::launch_arma3(
        arma3_directory,
        server_address,
        server_password,
        battle_eye_enabled,
    )
}

fn get_server_info(address: &str) -> String {
    use valve_server_query::Server;

    match Server::new(address) {
        Ok(server) => {
            let mut result = String::new();

            // Get server info
            match server.info() {
                Ok(info) => {
                    // Simple format: Name Game (Players: current/max)
                    result.push_str(&format!(
                        "{} {} (Players: {}/{})",
                        info.name(),
                        info.game(),
                        info.player_count(),
                        info.player_max()
                    ));
                }
                Err(_e) => result.push_str("Server is Offline"),
            }
            result
        }
        Err(e) => {
            let error_msg = format!("{}", e);
            error_msg
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            #[cfg(desktop)]
            {
                app.handle()
                    .plugin(tauri_plugin_updater::Builder::new().build())?;
                app.manage(app_updater::init_updater_state());
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            query_server,
            select_directory,
            save_settings,
            load_settings,
            launch_arma3,
            scan_mods,
            launch_arma3_with_mods,
            launch_arma3_enhanced,
            save_mod_preset,
            load_mod_presets,
            update_mod_preset,
            delete_mod_preset,
            apply_mod_preset,
            check_mod_updates_with_notifications,
            update_single_mod,
            update_all_mods,
            fetch_remote_mod_versions,
            #[cfg(desktop)]
            check_for_update,
            #[cfg(desktop)]
            download_and_install_update
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
