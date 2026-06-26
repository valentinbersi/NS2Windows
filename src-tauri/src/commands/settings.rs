use crate::state::app_state::AppState;
use std::sync::atomic::Ordering;
use tauri::{AppHandle, State};
use tauri_plugin_store::StoreExt;

#[tauri::command]
pub fn update_display_frequency(
    state: State<'_, AppState>,
    app: AppHandle,
    new_frequency: u16,
) -> Result<(), String> {
    let store = app.store("settings.json").map_err(|err| err.to_string())?;
    store.set("display_frequency", new_frequency);

    state
        .display_frequency
        .store(new_frequency, Ordering::Relaxed);

    Ok(())
}

#[tauri::command]
pub async fn update_emulation_frequency(
    state: State<'_, AppState>,
    app: AppHandle,
    new_frequency: u16,
) -> Result<(), String> {
    let store = app.store("settings.json").map_err(|err| err.to_string())?;
    store.set("emulation_frequency", new_frequency);

    state
        .emulation_frequency
        .store(new_frequency, Ordering::Relaxed);

    Ok(())
}
