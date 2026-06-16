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
use btleplug::api::{Characteristic, Peripheral as PeripheralApi};
use btleplug::platform::Peripheral;
use futures::StreamExt;
use std::sync::Arc;
use std::time::Duration;
use tauri::async_runtime::JoinHandle;
use tauri::State;
use tokio::sync::Mutex;
use tokio::time::sleep;
use uuid::Uuid;
use vigem_rust::client::ClientError;
use vigem_rust::target::{DualShock4, Xbox360};
use vigem_rust::TargetHandle;

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

    let peripheral = controller.peripheral();
    let input = controller.input();

    let task = tauri::async_runtime::spawn(async move {
        let decoder = Decoder;
        let evaluator = Evaluator;

        let mut stream = peripheral.notifications().await.unwrap();
        while let Some(notification) = stream.next().await {
            if notification.uuid != input.uuid {
                continue;
            }

            let buffer = notification.value;
            let input = match controller.kind() {
                NsControllerKind::LeftJoyCon => decoder.decode_left_joycon(&buffer),
                NsControllerKind::RightJoyCon => decoder.decode_right_joycon(&buffer),
                NsControllerKind::ProController => decoder.decode_pro_controller(&buffer),
                NsControllerKind::NsoGcController => decoder.decode_gc_controller(&buffer),
            };

            let output = evaluator.evaluate_profile(&profile, &input);
            let _ = virtual_controller.update(output);
        }
    });

    let task = EmulatedControllerTask::new_single_controller(task);

    let id = Uuid::now_v7();

    state.insert_emulated_controller(id, task).await;

    Ok(id)
}

fn spawn_input_task(
    peripheral: Peripheral,
    input: Arc<Characteristic>,
    output: Arc<Mutex<Arc<Vec<u8>>>>,
) -> JoinHandle<()> {
    tauri::async_runtime::spawn(async move {
        let mut notifications = peripheral.notifications().await.unwrap();
        while let Some(notification) = notifications.next().await {
            if notification.uuid != input.uuid {
                continue;
            }

            let buffer = Arc::new(notification.value);
            *output.lock().await = buffer;
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

    let left_buffer = Arc::new(Mutex::new(Arc::new(vec![])));
    let right_buffer = Arc::new(Mutex::new(Arc::new(vec![])));

    let left_task = spawn_input_task(left.peripheral(), left.input(), left_buffer.clone());
    let right_task = spawn_input_task(right.peripheral(), right.input(), right_buffer.clone());

    let output_task = tauri::async_runtime::spawn(async move {
        let decoder = Decoder;
        let evaluator = Evaluator;

        loop {
            let left_buffer = left_buffer.lock().await.clone();
            let right_buffer = right_buffer.lock().await.clone();

            if left_buffer.is_empty() || right_buffer.is_empty() {
                sleep(Duration::from_millis(5)).await;
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
