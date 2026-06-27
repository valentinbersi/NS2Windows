use crate::commands::connections::{connect_controller, disconnect_controller};
use crate::commands::controllers::{start_controller, stop_controller};
use crate::commands::profiles::{
    delete_profile, find_profile_by_name, profile_names, save_profile,
};
use crate::commands::settings::{update_display_frequency, update_emulation_frequency};
use crate::communication::communicator::BluetoothCommunicator;
use crate::connection::connector::BluetoothConnector;
use crate::repositories::profile_repository::ProfileRepository;
use crate::state::app_state::AppState;
use btleplug::api::Manager as BManager;
use migration::{Migrator, MigratorTrait};
use sea_orm::Database;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::sync::atomic::AtomicU16;
use std::sync::Arc;
use tauri::{App, AppHandle, Manager as TManager, RunEvent};
use tauri_plugin_store::StoreExt;
use vigem_rust::Client;

pub mod commands;
pub mod communication;
pub mod connection;
pub mod data;
pub mod decode;
pub mod dtos;
pub mod encode;
pub mod entities;
pub mod evaluation;
pub mod profiles;
pub mod repositories;
pub mod state;

fn setup(app: &mut App) -> Result<(), Box<dyn Error>> {
    let app_data_dir = app.path().app_data_dir()?;

    if !app_data_dir.exists() {
        fs::create_dir_all(&app_data_dir)?;
    }

    let db_path = app_data_dir.join("profiles.db");

    if !db_path.exists() {
        File::create(&db_path)?;
    }

    let db = tauri::async_runtime::block_on(async {
        let db = Database::connect(format!("sqlite://{}", db_path.to_string_lossy())).await?;
        Migrator::up(&db, None).await?;
        Ok::<sea_orm::DatabaseConnection, Box<dyn Error>>(db)
    })?;

    let adapter = tauri::async_runtime::block_on(async {
        let manager = btleplug::platform::Manager::new().await?;
        let adapters = manager.adapters().await?;
        Ok::<Option<btleplug::platform::Adapter>, btleplug::Error>(adapters.first().cloned())
    })?
    .ok_or("No Bluetooth adapters found")?;

    let vigem_client = Client::connect()?;

    let store = app.store("settings.json")?;

    let display_frequency = store
        .get("display_frequency")
        .and_then(|value| value.as_u64())
        .take_if(|value| *value <= u16::MAX as u64)
        .map(|value| value as u16)
        .map(AtomicU16::new)
        .map(Arc::new)
        .unwrap_or_else(|| {
            store.set("display_frequency", 60);
            Arc::new(AtomicU16::new(60))
        });

    let emulation_frequency = store
        .get("emulation_frequency")
        .and_then(|value| value.as_u64())
        .take_if(|value| *value <= u16::MAX as u64)
        .map(|value| value as u16)
        .map(AtomicU16::new)
        .map(Arc::new)
        .unwrap_or_else(|| {
            store.set("emulation_frequency", 60);
            Arc::new(AtomicU16::new(60))
        });

    app.manage(AppState::new(
        ProfileRepository::new(db),
        BluetoothConnector::new(adapter),
        BluetoothCommunicator,
        vigem_client,
        display_frequency,
        emulation_frequency,
    ));

    Ok(())
}

fn event_loop(app_handle: &AppHandle, event: RunEvent) {
    let state = app_handle.state::<AppState>();

    match event {
        RunEvent::Exit => {}
        RunEvent::ExitRequested { .. } => {
            let _ = tauri::async_runtime::block_on(state.cleanup());
        }
        RunEvent::WindowEvent { .. } => {}
        RunEvent::WebviewEvent { .. } => {}
        RunEvent::Ready => {}
        RunEvent::Resumed => {}
        RunEvent::MainEventsCleared => {}
        RunEvent::MenuEvent(_) => {}
        _ => {}
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> tauri::Result<()> {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            connect_controller,
            disconnect_controller,
            start_controller,
            stop_controller,
            save_profile,
            delete_profile,
            find_profile_by_name,
            profile_names,
            update_display_frequency,
            update_emulation_frequency,
        ])
        .setup(setup)
        .build(tauri::generate_context!())?
        .run(event_loop);

    Ok(())
}
