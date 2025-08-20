use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ModInfo {
    pub name: String,
    pub path: String,
    pub enabled: bool,
    pub mod_type: String, // "workshop", "local", "dlc"
    pub workshop_id: Option<String>,
    pub size: Option<u64>,
    pub last_modified: Option<String>,
    pub extensions: Vec<String>, // Paths to DLL/SO extension files
}

fn parse_mod(mod_path: &Path, mod_type: &str, use_meta_name: bool) -> Option<ModInfo> {
    let name = mod_path.file_name()?.to_str()?.to_string();

    // Determine workshop_id based on mod_type and name pattern
    let workshop_id = if mod_type == "workshop" && name.chars().all(|c| c.is_ascii_digit()) {
        Some(name.clone())
    } else {
        None
    };

    // Get display name based on mod type
    let display_name = if use_meta_name {
        get_mod_name_from_meta(mod_path).unwrap_or(name.clone())
    } else {
        name.clone()
    };

    let size = None;
    let last_modified = get_last_modified(mod_path);
    let extensions = scan_extensions(mod_path);

    Some(ModInfo {
        name: display_name,
        path: mod_path.to_string_lossy().to_string(),
        enabled: false,
        mod_type: mod_type.to_string(),
        workshop_id,
        size,
        last_modified,
        extensions,
    })
}

pub fn parse_workshop_mod(mod_path: &Path) -> Option<ModInfo> {
    parse_mod(mod_path, "workshop", true)
}

pub fn parse_local_mod(mod_path: &Path) -> Option<ModInfo> {
    parse_mod(mod_path, "local", true)
}

pub fn parse_dlc_mod(mod_path: &Path) -> Option<ModInfo> {
    parse_mod(mod_path, "dlc", false)
}

pub fn parse_custom_workshop_mod(mod_path: &Path) -> Option<ModInfo> {
    parse_mod(mod_path, "workshop", true)
}

pub fn parse_custom_local_mod(mod_path: &Path) -> Option<ModInfo> {
    parse_mod(mod_path, "local", true)
}

fn get_mod_name_from_meta(mod_path: &Path) -> Option<String> {
    // Helper function to extract name from a config file
    let extract_name_from_file = |file_path: &Path| -> Option<String> {
        if let Ok(content) = fs::read_to_string(file_path) {
            for line in content.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with("name") {
                    if let Some(start) = trimmed.find('"') {
                        if let Some(end) = trimmed.rfind('"') {
                            if start < end {
                                return Some(trimmed[start + 1..end].to_string());
                            }
                        }
                    }
                }
            }
        }
        None
    };

    // First try meta.cpp
    let meta_path = mod_path.join("meta.cpp");
    if meta_path.exists() {
        if let Some(name) = extract_name_from_file(&meta_path) {
            return Some(name);
        }
    }

    // Fallback to mod.cpp if meta.cpp doesn't exist or doesn't contain a name
    let mod_cpp_path = mod_path.join("mod.cpp");
    if mod_cpp_path.exists() {
        if let Some(name) = extract_name_from_file(&mod_cpp_path) {
            return Some(name);
        }
    }

    None
}

fn get_last_modified(path: &Path) -> Option<String> {
    let metadata = fs::metadata(path).ok()?;
    let modified = metadata.modified().ok()?;
    let datetime = chrono::DateTime::<chrono::Utc>::from(modified);
    Some(datetime.format("%Y-%m-%d %H:%M:%S").to_string())
}

// Scan for 64-bit DLL/SO extensions in mod directories
fn scan_extensions(mod_path: &Path) -> Vec<String> {
    let mut extensions = Vec::new();

    // Common extension directories to scan (expanded list)
    let extension_dirs = [
        "addons",
        "extensions",
        "dll",
        "bin",
        "plugins",
        "lib",
        "libs",
        "Addons",
        "Extensions",
        "DLL",
        "Bin",
    ];

    // Helper function to process extension files
    let process_extension_file = |path: &Path, extensions: &mut Vec<String>| {
        if let Some(extension) = path.extension() {
            let ext_str = extension.to_str().unwrap_or("");
            if ext_str == "dll" {
                // Only include 64-bit DLLs for x64 Arma 3
                if is_64bit_dll(path) {
                    extensions.push(path.to_string_lossy().to_string());
                }
            } else if ext_str == "so" {
                // Include .so files (Linux shared libraries)
                // For .so files, we'll assume they're compatible since ELF parsing is more complex
                extensions.push(path.to_string_lossy().to_string());
            }
        }
    };

    // First check the mod root directory for extension files
    for entry in WalkDir::new(mod_path).max_depth(1).into_iter().flatten() {
        if entry.file_type().is_file() {
            process_extension_file(entry.path(), &mut extensions);
        }
    }

    // Then check common extension directories
    for dir_name in &extension_dirs {
        let dir_path = mod_path.join(dir_name);
        if dir_path.exists() && dir_path.is_dir() {
            // Recursively scan for .dll and .so files
            for entry in WalkDir::new(&dir_path).into_iter().flatten() {
                if entry.file_type().is_file() {
                    process_extension_file(entry.path(), &mut extensions);
                }
            }
        }
    }

    extensions
}

// Check if a DLL is 64-bit by examining its PE header
fn is_64bit_dll(dll_path: &Path) -> bool {
    if let Ok(file_data) = fs::read(dll_path) {
        // Basic PE header check for 64-bit
        if file_data.len() > 64 {
            // Check DOS header signature "MZ"
            if file_data[0] == 0x4D && file_data[1] == 0x5A {
                // Get PE header offset from DOS header
                if let Some(pe_offset) = file_data.get(60..64) {
                    let pe_offset = u32::from_le_bytes([
                        pe_offset[0],
                        pe_offset[1],
                        pe_offset[2],
                        pe_offset[3],
                    ]) as usize;

                    // Check if we have enough data for PE header
                    if file_data.len() > pe_offset + 24 {
                        // Check PE signature "PE\0\0"
                        if file_data[pe_offset] == 0x50
                            && file_data[pe_offset + 1] == 0x45
                            && file_data[pe_offset + 2] == 0x00
                            && file_data[pe_offset + 3] == 0x00
                        {
                            // Check machine type (offset 4 from PE signature)
                            if let Some(machine_type) = file_data.get(pe_offset + 4..pe_offset + 6)
                            {
                                let machine =
                                    u16::from_le_bytes([machine_type[0], machine_type[1]]);
                                // 0x8664 = IMAGE_FILE_MACHINE_AMD64 (64-bit)
                                // 0x014c = IMAGE_FILE_MACHINE_I386 (32-bit)
                                return machine == 0x8664;
                            }
                        }
                    }
                }
            }
        }
    }

    // Default to false if we can't determine (safer for compatibility)
    false
}

pub fn scan_mods(
    arma3_directory: String,
    steam_library_path: Option<String>,
    custom_mods_directory: Option<String>,
) -> Result<Vec<ModInfo>, String> {
    if arma3_directory.is_empty() {
        return Err("Arma 3 directory not set".to_string());
    }

    let arma3_path = Path::new(&arma3_directory);
    if !arma3_path.exists() {
        return Err("Arma 3 directory does not exist".to_string());
    }

    let mut mods = Vec::new();

    // Scan Workshop mods
    // First try the !Workshop directory (if it exists in Arma 3 folder)
    let workshop_path = arma3_path.join("!Workshop");
    if workshop_path.exists() {
        for entry in WalkDir::new(&workshop_path)
            .max_depth(1)
            .into_iter()
            .flatten()
        {
            if entry.file_type().is_dir() && entry.path() != workshop_path {
                // Filter out Steam metadata folder
                if let Some(name) = entry.file_name().to_str() {
                    if name == "!DO_NOT_CHANGE_FILES_IN_THESE_FOLDERS" {
                        continue;
                    }
                }
                if let Some(mod_info) = parse_workshop_mod(entry.path()) {
                    mods.push(mod_info);
                }
            }
        }
    }

    // Scan Steam Workshop directory - use provided Steam library path if available
    if let Some(steam_workshop_mods) = scan_steam_workshop_mods_with_path(steam_library_path) {
        mods.extend(steam_workshop_mods);
    }

    // Scan local mods (directories starting with @)
    for entry in WalkDir::new(arma3_path).max_depth(1).into_iter().flatten() {
        if entry.file_type().is_dir() {
            if let Some(name) = entry.file_name().to_str() {
                if name.starts_with('@') && entry.path() != arma3_path {
                    if let Some(mod_info) = parse_local_mod(entry.path()) {
                        mods.push(mod_info);
                    }
                }
            }
        }
    }

    // Scan DLCs (individual DLC directories in Arma 3 root that contain addons folder)
    // Exclude default/free DLCs that are automatically loaded by Arma 3
    let default_dlcs = [
        "Argo",      // Argo and Malden DLC
        "Mark",      // Marksmen DLC
        "Heli",      // Helicopters DLC
        "Kart",      // Karts DLC
        "Curator",   // Zeus DLC
        "Expansion", // Apex DLC (actually paid, but often comes bundled)
        "Orange",    // Laws of War DLC
        "AoW",       // Art of War DLC
        "Tank",      // Tanks DLC
        "Jets",      // Jets DLC
        "Tacops",    // Tac-Ops Mission Pack
        "Enoch",     // Enoch DLC (dependency for Contact, don't show separately)
    ];

    for entry in WalkDir::new(arma3_path).max_depth(1).into_iter().flatten() {
        if entry.file_type().is_dir() && entry.path() != arma3_path {
            if let Some(name) = entry.file_name().to_str() {
                // Skip mod directories (starting with @) and common non-DLC folders
                if !name.starts_with('@')
                    && !name.starts_with('!')
                    && name != "Addons"
                    && name != "Dta"
                    && name != "DirectX"
                    && name != "BattlEye"
                    && !default_dlcs.contains(&name)
                {
                    // Skip default DLCs
                    // Check if this directory contains an addons folder (DLC indicator)
                    let addons_path = entry.path().join("addons");
                    if addons_path.exists() && addons_path.is_dir() {
                        if let Some(mut mod_info) = parse_dlc_mod(entry.path()) {
                            // Map DLC directory names to full names
                            mod_info.name = match name {
                                "Contact" => "Contact".to_string(),
                                "CSLA" => "CSLA Iron Curtain".to_string(),
                                "vn" => "S.O.G. Prairie Fire".to_string(),
                                "GM" => "Global Mobilization".to_string(),
                                "WS" => "Western Sahara".to_string(),
                                "EF" => "Expeditionary Forces".to_string(),
                                "RF" => "Reaction Forces".to_string(),
                                "SPE" => "Spearhead 1944".to_string(),
                                _ => name.to_string(),
                            };
                            mods.push(mod_info);
                        }
                    }
                }
            }
        }
    }

    // Scan custom mods directory (if provided)
    if let Some(custom_dir) = custom_mods_directory {
        if !custom_dir.is_empty() {
            let custom_path = Path::new(&custom_dir);
            if custom_path.exists() {
                for entry in WalkDir::new(custom_path).max_depth(1).into_iter().flatten() {
                    if entry.file_type().is_dir() && entry.path() != custom_path {
                        if let Some(name) = entry.file_name().to_str() {
                            // Check if it's a mod directory (starts with @ or is a numeric workshop ID)
                            if name.starts_with('@') || name.chars().all(|c| c.is_ascii_digit()) {
                                // Determine if it's a workshop mod or local mod based on name
                                let mod_info = if name.chars().all(|c| c.is_ascii_digit()) {
                                    parse_custom_workshop_mod(entry.path())
                                } else {
                                    parse_custom_local_mod(entry.path())
                                };

                                if let Some(mod_info) = mod_info {
                                    mods.push(mod_info);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(mods)
}

fn scan_steam_workshop_mods_with_path(steam_library_path: Option<String>) -> Option<Vec<ModInfo>> {
    let mut possible_steam_paths = Vec::new();

    // If user provided a Steam library path, prioritize it
    if let Some(user_path) = steam_library_path {
        if !user_path.is_empty() {
            possible_steam_paths.push(user_path);
        }
    }

    // Add common Steam installation paths as fallbacks
    possible_steam_paths.extend([
        "C:\\Program Files (x86)\\Steam".to_string(),
        "C:\\Program Files\\Steam".to_string(),
    ]);

    for steam_path in possible_steam_paths {
        let workshop_path = Path::new(&steam_path)
            .join("steamapps")
            .join("workshop")
            .join("content")
            .join("107410"); // Arma 3's Steam app ID

        if workshop_path.exists() {
            let mut mods = Vec::new();

            for entry in WalkDir::new(&workshop_path)
                .max_depth(1)
                .into_iter()
                .flatten()
            {
                if entry.file_type().is_dir() && entry.path() != workshop_path {
                    // Filter out Steam metadata folder
                    if let Some(name) = entry.file_name().to_str() {
                        if name == "!DO_NOT_CHANGE_FILES_IN_THESE_FOLDERS" {
                            continue;
                        }
                    }
                    if let Some(mod_info) = parse_workshop_mod(entry.path()) {
                        mods.push(mod_info);
                    }
                }
            }

            if !mods.is_empty() {
                return Some(mods);
            }
        }
    }

    None
}
