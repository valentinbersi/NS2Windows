use crate::communication::communicator::LedPatten;
use crate::data::ns_controller_kind::NsControllerKind;
use crate::decode::decoder::Decoder;
use crate::state::app_state::AppState;
use crate::state::ns_controller::NsController;
use btleplug::api::Peripheral;
use futures::StreamExt;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Duration;
use tauri::{AppHandle, Emitter, State};
use tokio::sync::watch;
use tokio::time;
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

    let (sender, receiver) = watch::channel(Arc::new(vec![]));

    let mut notifications = device
        .peripheral
        .notifications()
        .await
        .map_err(|err| err.to_string())?;

    let input_uuid = device.input_char.uuid;

    let input_pooler = tauri::async_runtime::spawn(async move {
        while let Some(notification) = notifications.next().await {
            if notification.uuid != input_uuid {
                continue;
            }

            let _ = sender.send(Arc::new(notification.value));
        }
    });

    let decoder = Decoder;
    let display_frequency = state.display_frequency.clone();
    let mut previous_display_frequency = display_frequency.load(Ordering::Relaxed);
    let mut interval = time::interval(Duration::from_secs_f64(
        1_f64 / previous_display_frequency as f64,
    ));
    let input_informer = tauri::async_runtime::spawn(async move {
        loop {
            let display_frequency = display_frequency.load(Ordering::Relaxed);

            if previous_display_frequency != display_frequency {
                interval =
                    time::interval(Duration::from_secs_f64(1_f64 / display_frequency as f64));
            }

            previous_display_frequency = display_frequency;

            interval.tick().await;

            let buffer = receiver.borrow().clone();

            if buffer.is_empty() {
                continue;
            }

            let input = match kind {
                NsControllerKind::LeftJoyCon => decoder.decode_left_joycon(&buffer),
                NsControllerKind::RightJoyCon => decoder.decode_right_joycon(&buffer),
                NsControllerKind::ProController => decoder.decode_pro_controller(&buffer),
                NsControllerKind::NsoGcController => decoder.decode_gc_controller(&buffer),
            };

            let _ = app.emit("update-input", (id, input));
        }
    });

    let controller = NsController::new(kind, device, input_pooler, input_informer);

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
