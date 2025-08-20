use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::io::Cursor;
use std::path::Path;
use tauri::{AppHandle, Emitter};
use tokio_stream::StreamExt;

const BUCKET_BASE_URL: &str = "https://mod.innovativedevsolutions.org";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RemoteModVersion {
    pub name: String,
    pub version: String,
    pub download_url: String,
    pub checksum: Option<String>,
    pub size: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LocalModVersion {
    pub name: String,
    pub version: String,
    pub checksum: Option<String>,
    pub installed_path: String,
    pub last_updated: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ModUpdateInfo {
    pub mod_name: String,
    pub current_version: Option<String>,
    pub remote_version: String,
    pub needs_update: bool,
    pub is_new_mod: bool,
    pub download_url: String,
    pub size: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LocalModRegistry {
    pub mods: HashMap<String, LocalModVersion>,
    pub last_check: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ModUpdateProgress {
    pub mod_name: String,
    pub step: String, // "downloading", "extracting", "installing"
    pub current: u64,
    pub total: u64,
    pub percentage: f64,
    pub message: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ModNotification {
    pub notification_type: String, // "up-to-date", "updates-available", "error"
    pub title: String,
    pub message: String,
    pub count: Option<u32>, // For updates-available, how many updates
}

impl Default for LocalModRegistry {
    fn default() -> Self {
        Self {
            mods: HashMap::new(),
            last_check: chrono::Utc::now()
                .format("%Y-%m-%d %H:%M:%S UTC")
                .to_string(),
        }
    }
}

/// Load the local mod registry from file
fn load_local_mod_registry(registry_path: &Path) -> Result<LocalModRegistry, String> {
    if !registry_path.exists() {
        return Ok(LocalModRegistry::default());
    }

    let content = fs::read_to_string(registry_path)
        .map_err(|e| format!("Failed to read mod registry: {}", e))?;

    let registry: LocalModRegistry = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse mod registry: {}", e))?;

    Ok(registry)
}

/// Save the local mod registry to file
fn save_local_mod_registry(
    registry_path: &Path,
    registry: &LocalModRegistry,
) -> Result<(), String> {
    // Create parent directory if it doesn't exist
    if let Some(parent) = registry_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create registry directory: {}", e))?;
    }

    let content = serde_json::to_string_pretty(registry)
        .map_err(|e| format!("Failed to serialize mod registry: {}", e))?;

    fs::write(registry_path, content)
        .map_err(|e| format!("Failed to write mod registry: {}", e))?;

    Ok(())
}

/// Fetch remote mod versions from the bucket
pub async fn fetch_remote_mod_versions() -> Result<HashMap<String, RemoteModVersion>, String> {
    let client = reqwest::Client::new();
    let mut remote_versions = HashMap::new();

    // Define the mod version files to check
    let version_files = [
        ("forge_client", "forge_client_version.json"),
        ("forge_mod", "forge_mod_version.json"),
        ("forge_phone", "forge_phone_version.json"),
        ("forge_server", "forge_server_version.json"),
    ];

    for (mod_name, version_file) in &version_files {
        let url = format!("{}/{}", BUCKET_BASE_URL, version_file);

        match client.get(&url).send().await {
            Ok(response) if response.status().is_success() => {
                match response.text().await {
                    Ok(version_text) => {
                        // Parse version (assuming it's just a simple string or JSON)
                        let version = version_text.trim().trim_matches('"').to_string();

                        // Construct download URL for the corresponding ZIP file
                        let download_url = match *mod_name {
                            "forge_client" => format!("{}/forge_client.zip", BUCKET_BASE_URL),
                            "forge_mod" => format!("{}/forge_mod.zip", BUCKET_BASE_URL),
                            "forge_phone" => format!("{}/forge_phone.zip", BUCKET_BASE_URL),
                            "forge_server" => format!("{}/forge_server.zip", BUCKET_BASE_URL),
                            _ => continue,
                        };

                        remote_versions.insert(
                            mod_name.to_string(),
                            RemoteModVersion {
                                name: mod_name.to_string(),
                                version,
                                download_url,
                                checksum: None, // We'll calculate this after download if needed
                                size: None,     // We'll get this from response headers if available
                            },
                        );
                    }
                    Err(e) => {
                        eprintln!("Failed to read version for {}: {}", mod_name, e);
                    }
                }
            }
            Ok(_) => {
                eprintln!("Failed to fetch version for {}: HTTP error", mod_name);
            }
            Err(e) => {
                eprintln!("Network error fetching version for {}: {}", mod_name, e);
            }
        }
    }

    Ok(remote_versions)
}

/// Check which mods need updates
pub async fn check_mod_updates(
    registry_path: &Path,
    _arma3_directory: &str,
) -> Result<Vec<ModUpdateInfo>, String> {
    let local_registry = load_local_mod_registry(registry_path)?;
    let remote_versions = fetch_remote_mod_versions().await?;

    let mut update_info = Vec::new();

    for (mod_name, remote_version) in &remote_versions {
        let current_version = local_registry.mods.get(mod_name).map(|m| m.version.clone());
        let needs_update = match &current_version {
            Some(local_ver) => local_ver != &remote_version.version,
            None => true, // New mod
        };

        let is_new_mod = current_version.is_none();

        update_info.push(ModUpdateInfo {
            mod_name: mod_name.clone(),
            current_version,
            remote_version: remote_version.version.clone(),
            needs_update,
            is_new_mod,
            download_url: remote_version.download_url.clone(),
            size: remote_version.size,
        });
    }

    Ok(update_info)
}

/// Download and extract a mod with progress tracking
pub async fn download_and_install_mod(
    registry_path: &Path,
    arma3_directory: &str,
    mod_info: &ModUpdateInfo,
    app_handle: Option<AppHandle>,
) -> Result<(), String> {
    let client = reqwest::Client::new();

    // Emit initial progress
    if let Some(ref handle) = app_handle {
        let _ = handle.emit(
            "mod-update-progress",
            ModUpdateProgress {
                mod_name: mod_info.mod_name.clone(),
                step: "downloading".to_string(),
                current: 0,
                total: 0,
                percentage: 0.0,
                message: format!("Starting download of {}...", mod_info.mod_name),
            },
        );
    }

    // Download the mod ZIP file
    let response = client
        .get(&mod_info.download_url)
        .send()
        .await
        .map_err(|e| format!("Failed to download mod {}: {}", mod_info.mod_name, e))?;

    if !response.status().is_success() {
        return Err(format!(
            "Failed to download mod {}: HTTP {}",
            mod_info.mod_name,
            response.status()
        ));
    }

    let total_size = response.content_length().unwrap_or(0);

    // Download with progress tracking
    let mut downloaded = 0u64;
    let mut bytes_vec = Vec::new();
    let mut stream = response.bytes_stream();
    let mut hasher = Sha256::new();

    while let Some(chunk) = stream.next().await {
        let chunk =
            chunk.map_err(|e| format!("Download error for {}: {}", mod_info.mod_name, e))?;

        bytes_vec.extend_from_slice(&chunk);
        hasher.update(&chunk);
        downloaded += chunk.len() as u64;

        let percentage = if total_size > 0 {
            (downloaded as f64 / total_size as f64) * 100.0
        } else {
            0.0
        };

        // Emit progress updates
        if let Some(ref handle) = app_handle {
            if downloaded % (512 * 1024) == 0 || downloaded == total_size {
                // Every 512KB
                let _ = handle.emit(
                    "mod-update-progress",
                    ModUpdateProgress {
                        mod_name: mod_info.mod_name.clone(),
                        step: "downloading".to_string(),
                        current: downloaded,
                        total: total_size,
                        percentage,
                        message: format!(
                            "Downloaded {} / {} MB",
                            downloaded / (1024 * 1024),
                            total_size / (1024 * 1024)
                        ),
                    },
                );
            }
        }
    }

    let bytes = bytes_vec;

    // Calculate checksum of downloaded file (already calculated during download)
    let checksum = format!("{:x}", hasher.finalize());

    // Emit extraction start
    if let Some(ref handle) = app_handle {
        let _ = handle.emit(
            "mod-update-progress",
            ModUpdateProgress {
                mod_name: mod_info.mod_name.clone(),
                step: "extracting".to_string(),
                current: 0,
                total: 100,
                percentage: 0.0,
                message: format!("Extracting {}...", mod_info.mod_name),
            },
        );
    }

    // Create mod directory in Arma 3 folder
    let mod_folder_name = format!("@{}", mod_info.mod_name);
    let mod_path = Path::new(arma3_directory).join(&mod_folder_name);

    // Remove existing mod directory if it exists
    if mod_path.exists() {
        fs::remove_dir_all(&mod_path).map_err(|e| {
            format!(
                "Failed to remove existing mod directory {}: {}",
                mod_folder_name, e
            )
        })?;
    }

    // Extract the ZIP file
    extract_zip(
        &bytes,
        &mod_path,
        mod_info.mod_name.clone(),
        app_handle.clone(),
    )?;

    // Emit installation progress
    if let Some(ref handle) = app_handle {
        let _ = handle.emit(
            "mod-update-progress",
            ModUpdateProgress {
                mod_name: mod_info.mod_name.clone(),
                step: "installing".to_string(),
                current: 90,
                total: 100,
                percentage: 90.0,
                message: format!("Finalizing installation of {}...", mod_info.mod_name),
            },
        );
    }

    // Update local registry
    let mut registry = load_local_mod_registry(registry_path)?;
    registry.mods.insert(
        mod_info.mod_name.clone(),
        LocalModVersion {
            name: mod_info.mod_name.clone(),
            version: mod_info.remote_version.clone(),
            checksum: Some(checksum),
            installed_path: mod_path.to_string_lossy().to_string(),
            last_updated: chrono::Utc::now()
                .format("%Y-%m-%d %H:%M:%S UTC")
                .to_string(),
        },
    );

    registry.last_check = chrono::Utc::now()
        .format("%Y-%m-%d %H:%M:%S UTC")
        .to_string();

    save_local_mod_registry(registry_path, &registry)?;

    // Emit completion
    if let Some(ref handle) = app_handle {
        let action = if mod_info.is_new_mod {
            "installed"
        } else {
            "updated"
        };
        let _ = handle.emit(
            "mod-update-progress",
            ModUpdateProgress {
                mod_name: mod_info.mod_name.clone(),
                step: "complete".to_string(),
                current: 100,
                total: 100,
                percentage: 100.0,
                message: format!(
                    "Successfully {} {} to version {}",
                    action, mod_info.mod_name, mod_info.remote_version
                ),
            },
        );
    }

    Ok(())
}

/// Extract a ZIP file to the specified directory with progress tracking
fn extract_zip(
    zip_data: &[u8],
    extract_to: &Path,
    mod_name: String,
    app_handle: Option<AppHandle>,
) -> Result<(), String> {
    let cursor = Cursor::new(zip_data);
    let mut archive =
        zip::ZipArchive::new(cursor).map_err(|e| format!("Failed to read ZIP archive: {}", e))?;

    // Create the destination directory
    fs::create_dir_all(extract_to).map_err(|e| format!("Failed to create mod directory: {}", e))?;

    // First, check if all files are nested under a common root directory
    let mut common_root = None;
    for i in 0..archive.len() {
        let file = archive
            .by_index(i)
            .map_err(|e| format!("Failed to read file from ZIP: {}", e))?;
        let file_path = file.mangled_name();

        // Skip empty paths
        if file_path.as_os_str().is_empty() {
            continue;
        }

        // Get the first component of the path
        if let Some(first_component) = file_path.components().next() {
            let root_name = first_component.as_os_str().to_string_lossy().to_string();

            match &common_root {
                None => common_root = Some(root_name),
                Some(existing_root) => {
                    if existing_root != &root_name {
                        // Files are not all under the same root, extract normally
                        common_root = None;
                        break;
                    }
                }
            }
        }
    }

    // Extract files with progress tracking
    let total_files = archive.len();
    for i in 0..archive.len() {
        // Emit progress every 10 files or at completion
        if let Some(ref handle) = app_handle {
            if i % 10 == 0 || i == total_files - 1 {
                let percentage = (i as f64 / total_files as f64) * 100.0;
                let _ = handle.emit(
                    "mod-update-progress",
                    ModUpdateProgress {
                        mod_name: mod_name.clone(),
                        step: "extracting".to_string(),
                        current: i as u64,
                        total: total_files as u64,
                        percentage,
                        message: format!("Extracting file {} of {}...", i + 1, total_files),
                    },
                );
            }
        }
        let mut file = archive
            .by_index(i)
            .map_err(|e| format!("Failed to read file from ZIP: {}", e))?;

        let original_path = file.mangled_name();

        // Skip empty paths
        if original_path.as_os_str().is_empty() {
            continue;
        }

        // Determine the final path
        let final_path = if let Some(ref root) = common_root {
            // Strip the common root directory from the path
            let stripped_path = original_path.strip_prefix(root).unwrap_or(&original_path);
            extract_to.join(stripped_path)
        } else {
            // Extract normally
            extract_to.join(original_path)
        };

        if file.is_dir() {
            // Only create directory if it's not empty after stripping
            if final_path != extract_to {
                fs::create_dir_all(&final_path)
                    .map_err(|e| format!("Failed to create directory: {}", e))?;
            }
        } else {
            // Create parent directories if needed
            if let Some(parent) = final_path.parent() {
                if parent != extract_to {
                    fs::create_dir_all(parent)
                        .map_err(|e| format!("Failed to create parent directory: {}", e))?;
                }
            }

            // Extract the file
            let mut outfile = fs::File::create(&final_path)
                .map_err(|e| format!("Failed to create file: {}", e))?;

            std::io::copy(&mut file, &mut outfile)
                .map_err(|e| format!("Failed to extract file: {}", e))?;
        }
    }

    Ok(())
}

/// Check for mod updates and emit appropriate notifications
pub async fn check_mod_updates_with_notifications(
    registry_path: &Path,
    arma3_directory: &str,
    app_handle: Option<AppHandle>,
) -> Result<Vec<ModUpdateInfo>, String> {
    let update_info = check_mod_updates(registry_path, arma3_directory).await?;
    let updates_available: Vec<_> = update_info
        .iter()
        .filter(|info| info.needs_update)
        .collect();

    if let Some(ref handle) = app_handle {
        if updates_available.is_empty() {
            // All mods are up to date - emit notification
            let _ = handle.emit(
                "mod-notification",
                ModNotification {
                    notification_type: "up-to-date".to_string(),
                    title: "All mods are up to date!".to_string(),
                    message: "Your mods are current with the latest versions.".to_string(),
                    count: None,
                },
            );
        } else {
            // Updates available - emit notification
            let count = updates_available.len() as u32;
            let plural = if count > 1 { "s" } else { "" };
            let _ = handle.emit(
                "mod-notification",
                ModNotification {
                    notification_type: "updates-available".to_string(),
                    title: format!("{} mod update{} available!", count, plural),
                    message: "Visit the Mods section to review and apply updates.".to_string(),
                    count: Some(count),
                },
            );
        }
    }

    Ok(update_info)
}

/// Update all mods that need updating
pub async fn update_all_mods(
    registry_path: &Path,
    arma3_directory: &str,
    app_handle: Option<AppHandle>,
) -> Result<Vec<String>, String> {
    let update_info = check_mod_updates(registry_path, arma3_directory).await?;
    let mut updated_mods = Vec::new();

    for mod_info in update_info.iter().filter(|info| info.needs_update) {
        match download_and_install_mod(registry_path, arma3_directory, mod_info, app_handle.clone())
            .await
        {
            Ok(()) => {
                let action = if mod_info.is_new_mod {
                    "installed"
                } else {
                    "updated"
                };
                updated_mods.push(format!(
                    "{} {} to version {}",
                    action, mod_info.mod_name, mod_info.remote_version
                ));
            }
            Err(e) => {
                eprintln!("Failed to update {}: {}", mod_info.mod_name, e);
            }
        }
    }

    Ok(updated_mods)
}
