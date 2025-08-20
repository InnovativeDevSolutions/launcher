use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, State};
use tauri_plugin_updater::{Update, UpdaterExt};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateMetadata {
    pub version: String,
    pub current_version: String,
    pub notes: Option<String>,
    pub date: Option<String>,
}

pub struct PendingUpdate(pub Arc<Mutex<Option<Update>>>);

#[tauri::command]
pub async fn check_for_update(
    app: AppHandle,
    pending_update: State<'_, PendingUpdate>,
) -> Result<Option<UpdateMetadata>, String> {
    println!("Checking for updates...");

    match app.updater() {
        Ok(updater) => {
            match updater.check().await {
                Ok(update) => {
                    let update_metadata = update.as_ref().map(|update| UpdateMetadata {
                        version: update.version.clone(),
                        current_version: update.current_version.clone(),
                        notes: update.body.clone(),
                        date: update.date.map(|d| d.to_string()),
                    });

                    // Store the update for later installation
                    *pending_update.0.lock().unwrap() = update;

                    println!("Update check completed: {:?}", update_metadata);
                    Ok(update_metadata)
                }
                Err(e) => Err(format!("Failed to check for updates: {}", e)),
            }
        }
        Err(e) => Err(format!("Failed to create updater: {}", e)),
    }
}

#[tauri::command]
pub async fn download_and_install_update(
    pending_update: State<'_, PendingUpdate>,
) -> Result<(), String> {
    println!("Starting update download and installation...");

    let update = {
        let mut pending = pending_update.0.lock().unwrap();
        pending.take()
    };

    let Some(update) = update else {
        return Err("No pending update available".to_string());
    };

    match update
        .download_and_install(
            |chunk_length, _content_length| {
                println!("Downloaded chunk: {} bytes", chunk_length);
            },
            || {
                println!("Update download finished");
            },
        )
        .await
    {
        Ok(_) => {
            println!("Update installation completed");
            Ok(())
        }
        Err(e) => Err(format!("Failed to download and install update: {}", e)),
    }
}

pub fn init_updater_state() -> PendingUpdate {
    PendingUpdate(Arc::new(Mutex::new(None)))
}
