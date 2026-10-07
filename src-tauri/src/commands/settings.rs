use crate::cemuhook::endpoint;
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
    if new_frequency == 0 {
        return Err("Display frequency must be at least 1 Hz.".into());
    }
    let store = app.store("settings.json").map_err(|err| err.to_string())?;
    store.set("display_frequency", new_frequency);
    store.save().map_err(|err| err.to_string())?;

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
    if new_frequency == 0 {
        return Err("Emulation frequency must be at least 1 Hz.".into());
    }
    let store = app.store("settings.json").map_err(|err| err.to_string())?;
    store.set("emulation_frequency", new_frequency);
    store.save().map_err(|err| err.to_string())?;

    state
        .emulation_frequency
        .store(new_frequency, Ordering::Relaxed);

    Ok(())
}

#[tauri::command]
pub fn update_cemuhook_settings(
    state: State<'_, AppState>,
    app: AppHandle,
    address: String,
    port: u16,
) -> Result<(), String> {
    let endpoint = endpoint(&address, port)?;
    let store = app
        .store("settings.json")
        .map_err(|error| error.to_string())?;
    state.cemuhook.configure(endpoint, || {
        let previous_address = store.get("cemuhook_address");
        let previous_port = store.get("cemuhook_port");
        store.set("cemuhook_address", endpoint.ip().to_string());
        store.set("cemuhook_port", port);
        if let Err(error) = store.save() {
            if let Some(value) = previous_address {
                store.set("cemuhook_address", value);
            } else {
                store.delete("cemuhook_address");
            }
            if let Some(value) = previous_port {
                store.set("cemuhook_port", value);
            } else {
                store.delete("cemuhook_port");
            }
            return Err(error.to_string());
        }
        Ok(())
    })
}
