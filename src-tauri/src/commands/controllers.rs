use crate::data::ns_controller_kind::NsControllerKind;
use crate::data::output_data::OutputData;
use crate::data::profile_kind::ProfileKind;
use crate::decode::decoder::Decoder;
use crate::dtos::emulated_controller::EmulatedController;
use crate::dtos::ns_connected_controller::{DualJoyCon, NsConnectedController, SingleController};
use crate::encode::ds4_encoder::Ds4Encoder;
use crate::encode::xbox_encoder::XboxEncoder;
use crate::evaluation::evaluator::Evaluator;
use crate::profiles::profile::Profile;
use crate::state::app_state::AppState;
use crate::state::emulated_controller_task::EmulatedControllerTask;
use crate::state::ns_controller::NsController;
use futures::StreamExt;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::time::Duration;
use tauri::State;
use tauri::async_runtime::JoinHandle;
use tokio::sync::watch;
use tokio::sync::watch::Sender;
use tokio::time;
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

    let (sender, receiver) = watch::channel(Arc::new(vec![0_u8; 0]));

    let mut notifications = controller
        .notifications()
        .await
        .map_err(|err| err.to_string())?;

    let input_uuid = controller.input_uuid();

    let input_pooler = tauri::async_runtime::spawn(async move {
        while let Some(notification) = notifications.next().await {
            if notification.uuid != input_uuid {
                continue;
            }

            let buffer = Arc::new(notification.value);
            let _ = sender.send(buffer);
        }
    });

    let decoder = Decoder;
    let evaluator = Evaluator;
    let controller_kind = controller.kind();
    let emulation_frequency = state.emulation_frequency.clone();
    let mut previous_emulation_frequency = emulation_frequency.load(Ordering::Relaxed);
    let mut interval = time::interval(Duration::from_secs_f64(
        1_f64 / previous_emulation_frequency as f64,
    ));
    let output_emulator = tauri::async_runtime::spawn(async move {
        loop {
            let emulation_frequency = emulation_frequency.load(Ordering::Relaxed);

            if previous_emulation_frequency != emulation_frequency {
                interval =
                    time::interval(Duration::from_secs_f64(1_f64 / emulation_frequency as f64));
            }

            previous_emulation_frequency = emulation_frequency;

            interval.tick().await;

            let buffer = receiver.borrow().clone();

            if buffer.is_empty() {
                continue;
            }

            let input = match controller_kind {
                NsControllerKind::LeftJoyCon => decoder.decode_left_joycon(&buffer),
                NsControllerKind::RightJoyCon => decoder.decode_right_joycon(&buffer),
                NsControllerKind::ProController => decoder.decode_pro_controller(&buffer),
                NsControllerKind::NsoGcController => decoder.decode_gc_controller(&buffer),
            };

            let output = evaluator.evaluate_profile(&profile, &input);
            let _ = virtual_controller.update(output);
        }
    });

    let task = EmulatedControllerTask::new_single_controller(input_pooler, output_emulator);

    let id = Uuid::now_v7();

    state.insert_emulated_controller(id, task).await;

    Ok(id)
}

async fn spawn_input_task(
    controller: Arc<NsController>,
    sender: Sender<Arc<Vec<u8>>>,
) -> Result<JoinHandle<()>, String> {
    let input_uuid = controller.input_uuid();
    let mut notifications = controller
        .notifications()
        .await
        .map_err(|err| err.to_string())?;

    Ok(tauri::async_runtime::spawn(async move {
        while let Some(notification) = notifications.next().await {
            if notification.uuid != input_uuid {
                continue;
            }

            let buffer = Arc::new(notification.value);
            let _ = sender.send(buffer);
        }
    }))
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

    let (left_sender, left_receiver) = watch::channel(Arc::new(vec![0_u8; 0]));
    let (right_sender, right_receiver) = watch::channel(Arc::new(vec![0_u8; 0]));

    let left_task = spawn_input_task(left, left_sender).await?;
    let right_task = spawn_input_task(right, right_sender).await?;

    let decoder = Decoder;
    let evaluator = Evaluator;
    let emulation_frequency = state.emulation_frequency.clone();
    let mut previous_emulation_frequency = emulation_frequency.load(Ordering::Relaxed);
    let mut interval = time::interval(Duration::from_secs_f64(
        1_f64 / previous_emulation_frequency as f64,
    ));
    let output_task = tauri::async_runtime::spawn(async move {
        loop {
            let emulation_frequency = emulation_frequency.load(Ordering::Relaxed);

            if previous_emulation_frequency != emulation_frequency {
                interval =
                    time::interval(Duration::from_secs_f64(1_f64 / emulation_frequency as f64));
            }

            previous_emulation_frequency = emulation_frequency;

            interval.tick().await;

            let left_buffer = left_receiver.borrow().clone();
            let right_buffer = right_receiver.borrow().clone();

            if left_buffer.is_empty() || right_buffer.is_empty() {
                continue;
            }

            let input = decoder.decode_dual_joycons(
                &left_buffer,
                &right_buffer,
                dual_joy_con.motion_source,
            );

            let output = evaluator.evaluate_profile(&profile, &input);

            let _ = virtual_controller.update(output);
        }
    });

    let id = Uuid::now_v7();
    let task = EmulatedControllerTask::new_dual_joy_con(left_task, right_task, output_task);

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
