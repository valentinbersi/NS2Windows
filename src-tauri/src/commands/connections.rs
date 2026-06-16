use crate::communication::communicator::LedPatten;
use crate::data::ns_controller_kind::NsControllerKind;
use crate::decode::decoder::Decoder;
use crate::state::app_state::AppState;
use crate::state::ns_controller::NsController;
use btleplug::api::Peripheral;
use futures::StreamExt;
use std::time::Duration;
use tauri::{AppHandle, Emitter, State};
use tokio::time::sleep;
use uuid::Uuid;

#[tauri::command]
pub async fn connect_controller(
    state: State<'_, AppState>,
    app: AppHandle,
    kind: NsControllerKind,
) -> Result<Uuid, String> {
    let id = Uuid::now_v7();

    // Alert waiting of connection
    app.emit("waiting-connection", (id, kind))
        .map_err(|err| err.to_string())?;

    // Wait for a controller
    let device = state
        .connector
        .wait_for_controller()
        .await
        .map_err(|err| err.to_string())?;

    // Alert start of connection configuration
    app.emit("configuring-connection", (id, kind))
        .map_err(|err| err.to_string())?;

    // Suscribe to input listening
    device
        .peripheral
        .subscribe(&device.input_char)
        .await
        .map_err(|err| err.to_string())?;

    state
        .communicator
        .send_custom_command(&device)
        .await
        .map_err(|err| err.to_string())?;

    sleep(Duration::from_millis(200)).await;

    // Set controller led to ■□□□
    state
        .communicator
        .set_device_led(&device, LedPatten::Led1)
        .await
        .map_err(|err| err.to_string())?;

    // Make the controller emit a sound
    state
        .communicator
        .emit_sound(&device)
        .await
        .map_err(|err| err.to_string())?;

    // Start input detection
    let input_informer_device = device.clone();
    let input_informer = tauri::async_runtime::spawn(async move {
        // Suscribe to input notifications
        let mut notifications = input_informer_device
            .peripheral
            .notifications()
            .await
            .unwrap();

        let decoder = Decoder;

        // Poll notifications from input characteristic until stream fail
        while let Some(notification) = notifications.next().await {
            if notification.uuid != input_informer_device.input_char.uuid {
                return;
            }

            // Decode input buffer
            let buffer = notification.value;
            let input = match kind {
                NsControllerKind::LeftJoyCon => decoder.decode_left_joycon(&buffer),
                NsControllerKind::RightJoyCon => decoder.decode_right_joycon(&buffer),
                NsControllerKind::ProController => decoder.decode_pro_controller(&buffer),
                NsControllerKind::NsoGcController => decoder.decode_gc_controller(&buffer),
            };

            // Report
            let _ = app.emit("update-input", (id, input));
        }
    });

    let controller = NsController::new(kind, device, input_informer);

    state.insert_ns_controller(id, controller).await;

    Ok(id)
}

#[tauri::command]
pub async fn disconnect_controller(state: State<'_, AppState>, id: Uuid) -> Result<(), String> {
    state
        .remove_connected_controller(&id)
        .await
        .map_err(|err| err.to_string())?
        .ok_or_else(|| format!("Could not find controller with id {id}"))
}
