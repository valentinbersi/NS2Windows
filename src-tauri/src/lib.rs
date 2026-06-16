use crate::commands::connections::{connect_controller, disconnect_controller};
use crate::commands::controllers::{start_controller, stop_controller};
use crate::commands::profiles::{
    delete_profile, find_profile_by_name, profile_names, save_profile,
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
use tauri::{App, AppHandle, Manager as TManager, RunEvent};
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

    app.manage(AppState::new(
        ProfileRepository::new(db),
        BluetoothConnector::new(adapter),
        BluetoothCommunicator,
        vigem_client,
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
        .invoke_handler(tauri::generate_handler![
            connect_controller,
            disconnect_controller,
            start_controller,
            stop_controller,
            save_profile,
            delete_profile,
            find_profile_by_name,
            profile_names,
        ])
        .setup(setup)
        .build(tauri::generate_context!())?
        .run(event_loop);

    Ok(())
}
