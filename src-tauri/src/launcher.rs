use crate::mods::ModInfo;
use std::fs;
use std::path::Path;
use std::process::Command;

// Setup extensions from enabled mods into Arma 3's extension directory
fn setup_extensions(enabled_mods: &[ModInfo], arma3_path: &Path) -> Result<(), String> {
    let extensions_dir = arma3_path.join("Extensions");

    // Create Extensions directory if it doesn't exist
    if !extensions_dir.exists() {
        fs::create_dir_all(&extensions_dir)
            .map_err(|e| format!("Failed to create Extensions directory: {}", e))?;
    }

    // Collect all extensions from enabled mods
    let mut extension_count = 0;
    for mod_info in enabled_mods {
        if mod_info.enabled {
            for extension_path in &mod_info.extensions {
                let extension_file = Path::new(extension_path);
                if let Some(filename) = extension_file.file_name() {
                    let target_path = extensions_dir.join(filename);

                    // Copy extension file (DLL/SO) to Arma 3 Extensions directory
                    if let Err(e) = fs::copy(extension_path, &target_path) {
                        eprintln!(
                            "Warning: Failed to copy extension {}: {}",
                            extension_path, e
                        );
                    } else {
                        extension_count += 1;
                        println!(
                            "Copied extension: {} -> {}",
                            extension_path,
                            target_path.display()
                        );
                    }
                }
            }
        }
    }

    if extension_count > 0 {
        println!("Setup {} extension(s) for Arma 3", extension_count);
    }

    Ok(())
}

pub fn launch_arma3_enhanced(
    arma3_directory: String,
    server_address: Option<String>,
    server_password: Option<String>,
    enabled_mods: Vec<ModInfo>,
    startup_parameters: String,
    battle_eye_enabled: bool,
) -> Result<String, String> {
    if arma3_directory.is_empty() {
        return Err("Arma 3 directory not set".to_string());
    }

    let arma3_path = Path::new(&arma3_directory);

    // Setup extensions from enabled mods
    setup_extensions(&enabled_mods, arma3_path)?;
    let exe_name = if battle_eye_enabled {
        "arma3battleye_x64.exe"
    } else {
        "arma3_x64.exe"
    };
    let exe_path = arma3_path.join(exe_name);

    if !exe_path.exists() {
        return Err(format!(
            "Arma 3 executable ({}) not found in the specified directory",
            exe_name
        ));
    }

    let mut cmd = Command::new(&exe_path);

    let mut mod_paths: Vec<String> = enabled_mods
        .into_iter()
        .map(|mod_info| mod_info.path)
        .collect();

    // Auto-load Enoch when Contact is enabled
    let has_contact = mod_paths
        .iter()
        .any(|path| path.ends_with("Contact") || path.ends_with("Contact\\"));

    if has_contact {
        // Add Enoch DLC path if Contact is enabled
        let arma3_path = Path::new(&arma3_directory);
        let enoch_path = arma3_path.join("Enoch");
        if enoch_path.exists() {
            let enoch_path_str = enoch_path.to_string_lossy().to_string();
            if !mod_paths.contains(&enoch_path_str) {
                mod_paths.push(enoch_path_str);
            }
        }
    }

    if !mod_paths.is_empty() {
        let mod_string = mod_paths.join(";");
        cmd.arg(format!("-mod={}", mod_string));
    }

    if !startup_parameters.trim().is_empty() {
        for param in startup_parameters.split_whitespace() {
            cmd.arg(param);
        }
    }

    if let Some(address) = server_address {
        if !address.is_empty() {
            cmd.arg(format!("-connect={}", address));

            if let Some(password) = server_password {
                if !password.is_empty() {
                    cmd.arg(format!("-password={}", password));
                }
            }
        }
    }

    cmd.current_dir(arma3_path);

    match cmd.spawn() {
        Ok(_) => {
            let mut result = "Arma 3 launched successfully".to_string();
            if !mod_paths.is_empty() {
                result.push_str(&format!(" with {} mod(s)", mod_paths.len()));
            }
            Ok(result)
        }
        Err(e) => Err(format!("Failed to launch Arma 3: {}", e)),
    }
}

pub fn launch_arma3_with_mods(
    arma3_directory: String,
    server_address: Option<String>,
    server_password: Option<String>,
    mod_paths: Vec<String>,
    startup_parameters: String,
    battle_eye_enabled: bool,
) -> Result<String, String> {
    if arma3_directory.is_empty() {
        return Err("Arma 3 directory not set".to_string());
    }

    let arma3_path = Path::new(&arma3_directory);

    let exe_name = if battle_eye_enabled {
        "arma3battleye.exe"
    } else {
        "arma3.exe"
    };
    let exe_path = arma3_path.join(exe_name);

    if !exe_path.exists() {
        return Err(format!(
            "Arma 3 executable ({}) not found in the specified directory",
            exe_name
        ));
    }

    let mut cmd = Command::new(&exe_path);

    if !mod_paths.is_empty() {
        let mod_string = mod_paths.join(";");
        cmd.arg(format!("-mod={}", mod_string));
    }

    if !startup_parameters.trim().is_empty() {
        for param in startup_parameters.split_whitespace() {
            cmd.arg(param);
        }
    }

    if let Some(address) = server_address {
        if !address.is_empty() {
            cmd.arg(format!("-connect={}", address));

            if let Some(password) = server_password {
                if !password.is_empty() {
                    cmd.arg(format!("-password={}", password));
                }
            }
        }
    }

    match cmd.spawn() {
        Ok(_) => Ok("Arma 3 launched successfully with mods".to_string()),
        Err(e) => Err(format!("Failed to launch Arma 3: {}", e)),
    }
}

pub fn launch_arma3(
    arma3_directory: String,
    server_address: Option<String>,
    server_password: Option<String>,
    battle_eye_enabled: bool,
) -> Result<String, String> {
    if arma3_directory.is_empty() {
        return Err("Arma 3 directory not set".to_string());
    }

    let arma3_path = Path::new(&arma3_directory);

    let exe_name = if battle_eye_enabled {
        "arma3battleye.exe"
    } else {
        "arma3.exe"
    };
    let exe_path = arma3_path.join(exe_name);

    if !exe_path.exists() {
        return Err(format!(
            "Arma 3 executable ({}) not found in the specified directory",
            exe_name
        ));
    }

    let mut cmd = Command::new(&exe_path);

    if let Some(address) = server_address {
        if !address.is_empty() {
            cmd.arg(format!("-connect={}", address));

            if let Some(password) = server_password {
                if !password.is_empty() {
                    cmd.arg(format!("-password={}", password));
                }
            }
        }
    }

    match cmd.spawn() {
        Ok(_) => Ok("Arma 3 launched successfully".to_string()),
        Err(e) => Err(format!("Failed to launch Arma 3: {}", e)),
    }
}
