use crate::data::ns_controller_kind::NsControllerKind;
use crate::data::output_data::OutputData;
use crate::data::profile_kind::ProfileKind;
use crate::decode::decoder::Decoder;
use crate::dtos::emulated_controller::EmulatedController;
use crate::dtos::motion_source::MotionSource;
use crate::dtos::ns_connected_controller::{DualJoyCon, NsConnectedController, SingleController};
use crate::encode::ds4_encoder::Ds4Encoder;
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
use tokio::time::Interval;
use uuid::Uuid;
use vigem_rust::TargetHandle;
use vigem_rust::client::ClientError;
use vigem_rust::target::{DualShock4, Xbox360};

#[derive(Clone)]
enum VirtualController {
    Xbox360(TargetHandle<Xbox360>),
    DualShock4(TargetHandle<DualShock4>),
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
    unique_receiver: Receiver<Arc<Vec<u8>>>,
    common_receiver: Receiver<Arc<Vec<u8>>>,
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

            let unique_buffer = unique_receiver.borrow().clone();
            let common_buffer = common_receiver.borrow().clone();

            if unique_buffer.is_empty() || common_buffer.is_empty() {
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

    let (unique_sender, unique_receiver) = watch::channel(Arc::new(vec![0_u8; 0]));
    let unique_input_listener = start_input_listening(
        device.controller(),
        device.unique_input_uuid(),
        unique_sender,
    )
    .await?;

    let (common_sender, common_receiver) = watch::channel(Arc::new(vec![0_u8; 0]));
    let common_input_listener = start_input_listening(
        device.controller(),
        device.common_input_uuid(),
        common_sender,
    )
    .await?;

    let output_emulator = start_single_controller_emulation(
        &state,
        controller.device().kind(),
        profile,
        virtual_controller,
        unique_receiver,
        common_receiver,
    );

    let task = EmulatedControllerTask::new_single_controller(
        unique_input_listener,
        common_input_listener,
        output_emulator,
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
    left_unique_receiver: Receiver<Arc<Vec<u8>>>,
    left_common_receiver: Receiver<Arc<Vec<u8>>>,
    right_unique_receiver: Receiver<Arc<Vec<u8>>>,
    right_common_receiver: Receiver<Arc<Vec<u8>>>,
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

            let left_unique_buffer = left_unique_receiver.borrow().clone();
            let left_common_buffer = left_common_receiver.borrow().clone();
            let right_unique_buffer = right_unique_receiver.borrow().clone();
            let right_common_buffer = right_common_receiver.borrow().clone();

            if left_unique_buffer.is_empty()
                || left_common_buffer.is_empty()
                || right_unique_buffer.is_empty()
                || right_common_buffer.is_empty()
            {
                continue;
            }

            let input = decoder.decode_dual_joy_cons(
                &left_unique_buffer,
                &left_common_buffer,
                &right_unique_buffer,
                &right_common_buffer,
                motion_source,
            );

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

    let (left_unique_sender, left_unique_receiver) = watch::channel(Arc::new(vec![0_u8; 0]));
    let (left_common_sender, left_common_receiver) = watch::channel(Arc::new(vec![0_u8; 0]));

    let (right_unique_sender, right_unique_receiver) = watch::channel(Arc::new(vec![0_u8; 0]));
    let (right_common_sender, right_common_receiver) = watch::channel(Arc::new(vec![0_u8; 0]));

    let left_unique_input_listener = start_input_listening(
        left_device.controller(),
        left_device.unique_input_uuid(),
        left_unique_sender,
    )
    .await?;

    let left_common_input_listener = start_input_listening(
        left_device.controller(),
        left_device.common_input_uuid(),
        left_common_sender,
    )
    .await?;

    let right_unique_input_listener = start_input_listening(
        right_device.controller(),
        right_device.unique_input_uuid(),
        right_unique_sender,
    )
    .await?;

    let right_common_input_listener = start_input_listening(
        right_device.controller(),
        right_device.common_input_uuid(),
        right_common_sender,
    )
    .await?;

    let output_emulator = start_dual_joy_con_emulation(
        &state,
        profile,
        dual_joy_con.motion_source,
        virtual_controller,
        left_unique_receiver,
        left_common_receiver,
        right_unique_receiver,
        right_common_receiver,
    );

    let id = Uuid::now_v7();
    let task = EmulatedControllerTask::new_dual_joy_con(
        left_unique_input_listener,
        left_common_input_listener,
        right_unique_input_listener,
        right_common_input_listener,
        output_emulator,
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
