use crate::communication::communicator::LedPatten;
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
        .suscribe_inputs()
        .await
        .map_err(|err| err.to_string())?;

    state
        .communicator
        .send_custom_command(&controller)
        .await
        .map_err(|err| err.to_string())?;

    sleep(Duration::from_millis(200)).await;

    // Set controller led to ■□□□
    state
        .communicator
        .set_device_led(&controller, LedPatten::Led1)
        .await
        .map_err(|err| err.to_string())?;

    // Make the controller emit a sound
    state
        .communicator
        .emit_sound(&controller)
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
    unique_receiver: Receiver<Arc<Vec<u8>>>,
    common_receiver: Receiver<Arc<Vec<u8>>>,
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

            let unique_buffer = unique_receiver.borrow().clone();
            let common_buffer = common_receiver.borrow().clone();

            if common_buffer.is_empty() || unique_buffer.is_empty() {
                continue;
            }

            let input = match kind {
                NsControllerKind::LeftJoyCon => {
                    decoder.decode_left_joy_con(&unique_buffer, &common_buffer)
                }

                NsControllerKind::RightJoyCon => {
                    decoder.decode_right_joy_con(&unique_buffer, &common_buffer)
                }

                NsControllerKind::ProController => {
                    decoder.decode_pro_controller(&unique_buffer, &common_buffer)
                }

                NsControllerKind::NsoGcController => {
                    decoder.decode_nso_gc_controller(&unique_buffer, &common_buffer)
                }
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

    let (common_sender, common_receiver) = watch::channel(Arc::new(vec![]));
    let common_input_listener = start_input_listening(
        device.controller(),
        device.common_input_uuid(),
        common_sender,
    )
    .await?;

    let (unique_sender, unique_receiver) = watch::channel(Arc::new(vec![]));
    let unique_input_listener = start_input_listening(
        device.controller(),
        device.unique_input_uuid(),
        unique_sender,
    )
    .await?;

    let input_reporter = start_input_reporting(
        &state,
        app,
        id,
        device.kind(),
        unique_receiver,
        common_receiver,
    );

    let controller = NsController::new(
        device,
        unique_input_listener,
        common_input_listener,
        input_reporter,
    );

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
