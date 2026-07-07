use crate::communication::communicator::LedPattern;
use crate::connection::connected_controller::ConnectedController;
use crate::data::ns_controller_kind::NsControllerKind;
use crate::decode::decoder::Decoder;
use crate::state::app_state::AppState;
use crate::state::ns_controller::NsController;
use btleplug::api::Peripheral as PeripheralApi;
use btleplug::platform::Peripheral;
use futures::StreamExt;
use std::sync::Arc;
use std::sync::atomic::{AtomicU16, Ordering};
use std::time::Duration;
use tauri::async_runtime::JoinHandle;
use tauri::{AppHandle, Emitter, State};
use tokio::sync::watch;
use tokio::sync::watch::{Receiver, Sender};
use tokio::time;
use tokio::time::{Interval, sleep};
use uuid::Uuid;

async fn wait_and_connect(
    state: &State<'_, AppState>,
    app: &AppHandle,
    id: &Uuid,
) -> Result<ConnectedController, String> {
    // Alert waiting of connection
    app.emit("waiting-connection", id)
        .map_err(|err| err.to_string())?;

    // Wait for a controller
    state
        .connector
        .wait_for_controller()
        .await
        .map_err(|err| err.to_string())
}

async fn configure_connection(
    state: &State<'_, AppState>,
    app: &AppHandle,
    id: &Uuid,
    controller: &ConnectedController,
) -> Result<(), String> {
    // Alert start of connection configuration
    app.emit("configuring-connection", (id, controller.kind()))
        .map_err(|err| err.to_string())?;

    // Suscribe to input listening
    controller
        .suscribe_input()
        .await
        .map_err(|err| err.to_string())?;

    state
        .communicator
        .set_feature_mask(controller)
        .await
        .map_err(|err| err.to_string())?;

    state
        .communicator
        .initialize_rumble(controller)
        .await
        .map_err(|err| err.to_string())?;

    state
        .communicator
        .configure_motion(controller)
        .await
        .map_err(|err| err.to_string())?;

    state
        .communicator
        .enable_features(controller)
        .await
        .map_err(|err| err.to_string())?;

    sleep(Duration::from_millis(200)).await;

    // Set controller led to ■□□□
    state
        .communicator
        .set_device_led(controller, LedPattern::Led1)
        .await
        .map_err(|err| err.to_string())?;

    // Make the controller emit a sound
    state
        .communicator
        .emit_sound(controller)
        .await
        .map_err(|err| err.to_string())
}

async fn start_input_listening(
    controller: Peripheral,
    input_uuid: Uuid,
    sender: Sender<Arc<Vec<u8>>>,
) -> Result<JoinHandle<()>, String> {
    let notifications = controller
        .notifications()
        .await
        .map_err(|err| err.to_string())?;

    let input_listener = tauri::async_runtime::spawn(async move {
        notifications
            .filter(|notification| {
                let notification_uuid = notification.uuid;
                async move { notification_uuid == input_uuid }
            })
            .map(|notification| notification.value)
            .map(Arc::new)
            .for_each(|buffer| async {
                let _ = sender.send(buffer);
            })
            .await;
    });

    Ok(input_listener)
}

async fn tick_interval(
    display_frequency: &Arc<AtomicU16>,
    previous_display_frequency: &mut u16,
    interval: &mut Interval,
) {
    let display_frequency = display_frequency.load(Ordering::Relaxed);

    if *previous_display_frequency != display_frequency {
        *interval = time::interval(Duration::from_secs_f64(1_f64 / display_frequency as f64));
    }

    *previous_display_frequency = display_frequency;

    interval.tick().await;
}

fn start_input_reporting(
    state: &State<'_, AppState>,
    app: AppHandle,
    id: Uuid,
    kind: NsControllerKind,
    receiver: Receiver<Arc<Vec<u8>>>,
) -> JoinHandle<()> {
    let decoder = Decoder;

    let display_frequency = state.display_frequency.clone();
    let mut previous_display_frequency = display_frequency.load(Ordering::Relaxed);

    let mut interval = time::interval(Duration::from_secs_f64(
        1_f64 / previous_display_frequency as f64,
    ));

    tauri::async_runtime::spawn(async move {
        loop {
            tick_interval(
                &display_frequency,
                &mut previous_display_frequency,
                &mut interval,
            )
            .await;

            let buffer = receiver.borrow().clone();

            if buffer.is_empty() {
                continue;
            }

            let input = match kind {
                NsControllerKind::LeftJoyCon => decoder.decode_left_joy_con(&buffer),
                NsControllerKind::RightJoyCon => decoder.decode_right_joy_con(&buffer),
                NsControllerKind::ProController => decoder.decode_pro_controller(&buffer),
                NsControllerKind::NsoGcController => decoder.decode_nso_gc_controller(&buffer),
            };

            let _ = app.emit("update-input", (id, input));
        }
    })
}

#[tauri::command]
pub async fn connect_controller(
    state: State<'_, AppState>,
    app: AppHandle,
) -> Result<Uuid, String> {
    let id = Uuid::now_v7();

    let device = wait_and_connect(&state, &app, &id).await?;
    configure_connection(&state, &app, &id, &device).await?;

    let (sender, receiver) = watch::channel(Arc::new(vec![]));
    let input_listener =
        start_input_listening(device.controller(), device.input_uuid(), sender).await?;

    let input_reporter = start_input_reporting(&state, app, id, device.kind(), receiver);

    let controller = NsController::new(device, input_listener, input_reporter);

    state.insert_ns_controller(id, controller).await;

    Ok(id)
}

#[tauri::command]
pub async fn set_controller_led(
    state: State<'_, AppState>,
    id: Uuid,
    led_pattern: LedPattern,
) -> Result<(), String> {
    let device = state
        .get_ns_controller(&id)
        .await
        .ok_or_else(|| format!("Could not find controller with id {id}"))?;

    let communicator = state.communicator;

    communicator
        .set_device_led(device.device(), led_pattern)
        .await
        .map_err(|err| err.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn disconnect_controller(state: State<'_, AppState>, id: Uuid) -> Result<(), String> {
    state
        .remove_connected_controller(&id)
        .await
        .map_err(|err| err.to_string())?
        .ok_or_else(|| format!("Could not find controller with id {id}"))
}
