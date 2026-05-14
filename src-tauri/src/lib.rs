use crate::commands::connections::{connect_controller, get_connections, remove_connection};
use crate::commands::profiles::{
    delete_profile, find_profile_by_name, profile_names, save_profile,
};
use crate::communication::communicator::BluetoothCommunicator;
use crate::connection::connector::BluetoothConnector;
use crate::repositories::profile_repository::ProfileRepository;
use crate::state::AppState;
use btleplug::api::Manager as BManager;
use maplit::hashmap;
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DbErr};
use tauri::Manager as TManager;
use tokio::sync::RwLock;

pub mod commands;
pub mod communication;
pub mod connection;
pub mod data;
pub mod decode;
pub mod dtos;
pub mod encode;
pub mod entities;
pub mod profiles;
pub mod repositories;
pub mod state;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> tauri::Result<()> {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            save_profile,
            delete_profile,
            find_profile_by_name,
            profile_names,
            connect_controller,
            get_connections,
            remove_connection,
        ])
        .setup(|app| {
            let db = tauri::async_runtime::block_on(async {
                let db = Database::connect("sqlite://app.db").await?;
                Migrator::up(&db, None).await?;
                Ok::<sea_orm::DatabaseConnection, DbErr>(db)
            })?;

            let adapter = tauri::async_runtime::block_on(async {
                let manager = btleplug::platform::Manager::new().await?;
                let adapters = manager.adapters().await?;
                Ok::<Option<btleplug::platform::Adapter>, btleplug::Error>(
                    adapters.first().cloned(),
                )
            })?
            .ok_or("No Bluetooth adapters found")?;

            app.manage(AppState {
                profile_repository: ProfileRepository::new(db),
                connector: BluetoothConnector::new(adapter),
                communicator: BluetoothCommunicator,
                connected_controllers: RwLock::new(hashmap!()),
            });

            Ok(())
        })
        .run(tauri::generate_context!())
}
