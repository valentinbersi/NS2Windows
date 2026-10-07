use crate::cemuhook::{CemuHookServer, DEFAULT_ADDRESS, DEFAULT_PORT, endpoint};
use crate::commands::connections::{connect_controller, disconnect_controller, set_controller_led};
use crate::commands::controllers::{start_controller, stop_controller};
use crate::commands::profiles::{
    delete_profile, find_profile_by_name, profile_names, save_profile,
};
use crate::commands::settings::{
    update_cemuhook_settings, update_display_frequency, update_emulation_frequency,
};
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
use std::sync::Arc;
use std::sync::atomic::AtomicU16;
use tauri::{App, AppHandle, Manager as TManager, RunEvent};
use tauri_plugin_store::StoreExt;
use vigem_rust::Client;

pub mod cemuhook;
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
    let profile_repository = ProfileRepository::new(db);
    let backfill_completed = store
        .get("xbox_motion_backfill_version")
        .and_then(|value| value.as_u64())
        .is_some_and(|version| version >= 1);
    if tauri::async_runtime::block_on(
        profile_repository.backfill_xbox_motion_defaults(backfill_completed),
    )? {
        store.set("xbox_motion_backfill_version", 1);
        store.save()?;
    }

    let address = store
        .get("cemuhook_address")
        .and_then(|value| value.as_str().map(str::to_owned))
        .unwrap_or_else(|| DEFAULT_ADDRESS.into());
    let port = store
        .get("cemuhook_port")
        .and_then(|value| value.as_u64())
        .filter(|port| (1..=65535).contains(port))
        .map(|port| port as u16)
        .unwrap_or(DEFAULT_PORT);
    let bind_endpoint = endpoint(&address, port)
        .unwrap_or_else(|_| endpoint(DEFAULT_ADDRESS, DEFAULT_PORT).unwrap());
    store.set("cemuhook_address", bind_endpoint.ip().to_string());
    store.set("cemuhook_port", bind_endpoint.port());

    let display_frequency = store
        .get("display_frequency")
        .and_then(|value| value.as_u64())
        .take_if(|value| (1..=u16::MAX as u64).contains(value))
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
        .take_if(|value| (1..=u16::MAX as u64).contains(value))
        .map(|value| value as u16)
        .map(AtomicU16::new)
        .map(Arc::new)
        .unwrap_or_else(|| {
            store.set("emulation_frequency", 60);
            Arc::new(AtomicU16::new(60))
        });

    app.manage(AppState::new(
        profile_repository,
        BluetoothConnector::new(adapter),
        BluetoothCommunicator,
        vigem_client,
        display_frequency,
        emulation_frequency,
        CemuHookServer::new(bind_endpoint),
    ));

    store.save()?;

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
            set_controller_led,
            start_controller,
            stop_controller,
            save_profile,
            delete_profile,
            find_profile_by_name,
            profile_names,
            update_display_frequency,
            update_emulation_frequency,
            update_cemuhook_settings,
        ])
        .setup(setup)
        .build(tauri::generate_context!())?
        .run(event_loop);

    Ok(())
}
