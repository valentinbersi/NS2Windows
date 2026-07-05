use crate::connection::connected_controller::ConnectedController;
use crate::data::ns_controller_kind::NsControllerKind;
use crate::data::output_data::OutputData;
use crate::data::profile_kind::ProfileKind;
use crate::decode::decoder::Decoder;
use crate::dtos::emulated_controller::EmulatedController;
use crate::dtos::motion_source::MotionSource;
use crate::dtos::ns_connected_controller::{DualJoyCon, NsConnectedController, SingleController};
use crate::encode::ds4_encoder::Ds4Encoder;
use crate::encode::rumble_encoder::RumbleEncoder;
use crate::encode::xbox_encoder::XboxEncoder;
use crate::evaluation::evaluator::Evaluator;
use crate::profiles::profile::Profile;
use crate::state::app_state::AppState;
use crate::state::emulated_controller_task::EmulatedControllerTask;
use btleplug::api::Peripheral as PeripheralApi;
use btleplug::platform::Peripheral;
use futures::StreamExt;
use std::sync::Arc;
use std::sync::atomic::{AtomicU16, Ordering};
use std::time::Duration;
use tauri::State;
use tauri::async_runtime::JoinHandle;
use tokio::sync::watch;
use tokio::sync::watch::{Receiver, Sender};
use tokio::time;
use tokio::time::{Interval, MissedTickBehavior};
use uuid::Uuid;
use vigem_rust::client::ClientError;
use vigem_rust::target::{DualShock4, Xbox360};
use vigem_rust::{Ds4Notification, TargetHandle, X360Notification};

const RUMBLE_REFRESH_INTERVAL: Duration = Duration::from_millis(8);

#[derive(Clone)]
enum VirtualController {
    Xbox360(TargetHandle<Xbox360>),
    DualShock4(TargetHandle<DualShock4>),
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct RumbleFeedback {
    large: u8,
    small: u8,
}

impl From<X360Notification> for RumbleFeedback {
    fn from(value: X360Notification) -> Self {
        Self {
            large: value.large_motor,
            small: value.small_motor,
        }
    }
}

impl From<Ds4Notification> for RumbleFeedback {
    fn from(value: Ds4Notification) -> Self {
        Self {
            large: value.large_motor,
            small: value.small_motor,
        }
    }
}

impl VirtualController {
    fn wait_for_ready(&self) -> Result<(), ClientError> {
        match self {
            VirtualController::Xbox360(target) => target.wait_for_ready(),
            VirtualController::DualShock4(target) => target.wait_for_ready(),
        }
    }

    fn update(&self, output: OutputData) -> Result<(), ClientError> {
        match self {
            VirtualController::Xbox360(target) => {
                let encoder = XboxEncoder;
                let report = encoder.encode(&output);
                target.update(&report)
            }

            VirtualController::DualShock4(target) => {
                let encoder = Ds4Encoder;
                let report = encoder.encode(&output);
                target.update_ex(&report)
            }
        }
    }

    fn start_feedback_bridge(&self) -> Result<Receiver<RumbleFeedback>, ClientError> {
        fn send_notifications<T, E>(
            sender: Sender<RumbleFeedback>,
            notifications: std::sync::mpsc::Receiver<Result<T, E>>,
        ) where
            T: Into<RumbleFeedback> + Send + 'static,
            E: Send + 'static,
        {
            std::thread::spawn(move || {
                while let Ok(Ok(notification)) = notifications.recv() {
                    let _ = sender.send(notification.into());
                }
            });
        }

        let (sender, receiver) = watch::channel(RumbleFeedback::default());

        match self {
            VirtualController::Xbox360(target) => {
                let notifications = target.register_notification()?;
                send_notifications(sender, notifications);
            }

            VirtualController::DualShock4(target) => {
                let notifications = target.register_notification()?;
                send_notifications(sender, notifications);
            }
        };

        Ok(receiver)
    }
}

fn start_rumble_forwarding(
    virtual_controller: &VirtualController,
    mut devices: Vec<ConnectedController>,
) -> Result<JoinHandle<()>, String> {
    // NSO GameCube rumble is intentionally disabled until its output format is verified.
    // Do not subscribe to ViGEm feedback or write to either GC output characteristic.
    devices.retain(|device| device.kind() != NsControllerKind::NsoGcController);

    if devices.is_empty() {
        return Ok(tauri::async_runtime::spawn(std::future::pending()));
    }

    let mut feedback = virtual_controller
        .start_feedback_bridge()
        .map_err(|error| error.to_string())?;

    Ok(tauri::async_runtime::spawn(async move {
        let mut interval = tokio::time::interval(RUMBLE_REFRESH_INTERVAL);
        interval.set_missed_tick_behavior(MissedTickBehavior::Skip);
        interval.tick().await;

        let mut outputs = devices
            .into_iter()
            .map(|device| (device, RumbleEncoder::default()))
            .collect::<Vec<_>>();

        let mut amplitude = 0;

        loop {
            tokio::select! {
                biased;

                changed = feedback.changed() => {
                    if changed.is_err() {
                        break;
                    }

                    let next_feedback = *feedback.borrow_and_update();
                    let next_amplitude = next_feedback.large.max(next_feedback.small);

                    if next_amplitude == amplitude {
                        continue;
                    }

                    amplitude = next_amplitude;

                    for (device, encoder) in &mut outputs {
                        let packet = encoder.packet(device.kind(), amplitude);

                        if let Err(error) = device.write_rumble(&packet).await {
                            eprintln!("Failed to update rumble: {error}");
                        }
                    }
                }

                _ = interval.tick(), if amplitude != 0 => {
                    for (device, encoder) in &mut outputs {
                        let packet = encoder.packet(device.kind(), amplitude);

                        if let Err(error) = device.write_rumble(&packet).await {
                            eprintln!("Failed to refresh rumble: {error}");
                        }
                    }
                }
            }
        }
    }))
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
    frequency: &Arc<AtomicU16>,
    previous_frequency: &mut u16,
    interval: &mut Interval,
) {
    let frequency = frequency.load(Ordering::Relaxed);

    if *previous_frequency != frequency {
        *interval = time::interval(Duration::from_secs_f64(1_f64 / frequency as f64));
    }

    *previous_frequency = frequency;

    interval.tick().await;
}

fn start_single_controller_emulation(
    state: &State<'_, AppState>,
    kind: NsControllerKind,
    profile: Profile,
    virtual_controller: VirtualController,
    receiver: Receiver<Arc<Vec<u8>>>,
) -> JoinHandle<()> {
    let decoder = Decoder;
    let evaluator = Evaluator;

    let emulation_frequency = state.emulation_frequency.clone();
    let mut previous_emulation_frequency = emulation_frequency.load(Ordering::Relaxed);

    let mut interval = time::interval(Duration::from_secs_f64(
        1_f64 / previous_emulation_frequency as f64,
    ));

    tauri::async_runtime::spawn(async move {
        loop {
            tick_interval(
                &emulation_frequency,
                &mut previous_emulation_frequency,
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

            let output = evaluator.evaluate_profile(&profile, &input);
            let _ = virtual_controller.update(output);
        }
    })
}

async fn start_single_controller(
    state: State<'_, AppState>,
    profile: Profile,
    single_controller: SingleController,
    virtual_controller: VirtualController,
) -> Result<Uuid, String> {
    let controller = state
        .get_ns_controller(&single_controller.id)
        .await
        .ok_or_else(|| format!("Could not find controller with id {}", single_controller.id))?;

    let device = controller.device();

    let (sender, receiver) = watch::channel(Arc::new(vec![]));
    let input_listener =
        start_input_listening(device.controller(), device.input_uuid(), sender).await?;

    let rumble_output = start_rumble_forwarding(&virtual_controller, vec![device.clone()])?;

    let output_emulator = start_single_controller_emulation(
        &state,
        device.kind(),
        profile,
        virtual_controller,
        receiver,
    );

    let task = EmulatedControllerTask::new_single_controller(
        input_listener,
        output_emulator,
        rumble_output,
    );

    let id = Uuid::now_v7();

    state.insert_emulated_controller(id, task).await;

    Ok(id)
}

#[allow(clippy::too_many_arguments)]
fn start_dual_joy_con_emulation(
    state: &State<'_, AppState>,
    profile: Profile,
    motion_source: MotionSource,
    virtual_controller: VirtualController,
    left_receiver: Receiver<Arc<Vec<u8>>>,
    right_receiver: Receiver<Arc<Vec<u8>>>,
) -> JoinHandle<()> {
    let decoder = Decoder;
    let evaluator = Evaluator;

    let emulation_frequency = state.emulation_frequency.clone();
    let mut previous_emulation_frequency = emulation_frequency.load(Ordering::Relaxed);

    let mut interval = time::interval(Duration::from_secs_f64(
        1_f64 / previous_emulation_frequency as f64,
    ));

    tauri::async_runtime::spawn(async move {
        loop {
            tick_interval(
                &emulation_frequency,
                &mut previous_emulation_frequency,
                &mut interval,
            )
            .await;

            let left_buffer = left_receiver.borrow().clone();
            let right_buffer = right_receiver.borrow().clone();

            if left_buffer.is_empty() || right_buffer.is_empty() {
                continue;
            }

            let input = decoder.decode_dual_joy_cons(&left_buffer, &right_buffer, motion_source);
            let output = evaluator.evaluate_profile(&profile, &input);
            let _ = virtual_controller.update(output);
        }
    })
}

async fn start_dual_joy_con(
    state: State<'_, AppState>,
    profile: Profile,
    dual_joy_con: DualJoyCon,
    virtual_controller: VirtualController,
) -> Result<Uuid, String> {
    let (left, right) = state
        .get_dual_ns_controllers(&dual_joy_con.left_id, &dual_joy_con.right_id)
        .await
        .map_err(|id| format!("Could not found a controller with id {id}"))?;

    let left_device = left.device();
    let right_device = right.device();

    let (left_sender, left_receiver) = watch::channel(Arc::new(vec![]));
    let (right_sender, right_receiver) = watch::channel(Arc::new(vec![]));

    let left_input_listener = start_input_listening(
        left_device.controller(),
        left_device.input_uuid(),
        left_sender,
    )
    .await?;

    let right_input_listener = start_input_listening(
        right_device.controller(),
        right_device.input_uuid(),
        right_sender,
    )
    .await?;

    let rumble_output = start_rumble_forwarding(
        &virtual_controller,
        vec![left_device.clone(), right_device.clone()],
    )?;

    let output_emulator = start_dual_joy_con_emulation(
        &state,
        profile,
        dual_joy_con.motion_source,
        virtual_controller,
        left_receiver,
        right_receiver,
    );

    let id = Uuid::now_v7();
    let task = EmulatedControllerTask::new_dual_joy_con(
        left_input_listener,
        right_input_listener,
        output_emulator,
        rumble_output,
    );

    state.insert_emulated_controller(id, task).await;

    Ok(id)
}

#[tauri::command]
pub async fn start_controller(
    state: State<'_, AppState>,
    controller: EmulatedController,
) -> Result<Uuid, String> {
    let profile = state
        .profile_repository
        .find_profile_by_name(&controller.profile_name)
        .await
        .map_err(|err| err.to_string())?
        .ok_or_else(|| {
            format!(
                "Could not find profile with name {}",
                controller.profile_name
            )
        })?;

    let virtual_controller = match profile.kind {
        ProfileKind::Ps4 => VirtualController::DualShock4(
            state
                .vigem_client
                .new_ds4_target()
                .plugin()
                .map_err(|err| err.to_string())?,
        ),
        ProfileKind::Xbox360 => VirtualController::Xbox360(
            state
                .vigem_client
                .new_x360_target()
                .plugin()
                .map_err(|err| err.to_string())?,
        ),
    };

    virtual_controller
        .wait_for_ready()
        .map_err(|err| err.to_string())?;

    match controller.connected_controller {
        NsConnectedController::SingleController(single_controller) => {
            start_single_controller(state, profile, single_controller, virtual_controller).await
        }

        NsConnectedController::DualJoyCon(dual_joy_con) => {
            start_dual_joy_con(state, profile, dual_joy_con, virtual_controller).await
        }
    }
}

#[tauri::command]
pub async fn stop_controller(
    state: State<'_, AppState>,
    emulated_controller_id: Uuid,
) -> Result<(), String> {
    state
        .remove_emulated_controller(&emulated_controller_id)
        .await
        .ok_or_else(|| {
            format!("Could not find emulated controller with id {emulated_controller_id}")
        })
}
