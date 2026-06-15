use crate::connection::connected_controller::ConnectedController;
use crate::connection::joy_con_side::JoyConSide;
use crate::connection::motion_source::MotionSource;
use crate::data::profile_kind::ProfileKind;
use crate::decode::decoder::Decoder;
use crate::encode::encoder::Encoder;
use crate::evaluation::evaluator::Evaluator;
use crate::state::AppState;
use btleplug::api::Peripheral;
use futures::StreamExt;
use std::sync::Arc;
use std::time::Duration;
use tauri::State;
use tokio::sync::Mutex;
use tokio::time::sleep;
use uuid::Uuid;
use vigem_rust::TargetHandle;
use vigem_rust::target::{DualShock4, Xbox360};

#[derive(Clone)]
enum VirtualController {
    Xbox360(TargetHandle<Xbox360>),
    DualShock4(TargetHandle<DualShock4>),
}

impl VirtualController {
    fn get_360(&self) -> &TargetHandle<Xbox360> {
        match self {
            VirtualController::Xbox360(controller) => controller,
            VirtualController::DualShock4(_) => panic!(),
        }
    }

    fn get_ps4(&self) -> &TargetHandle<DualShock4> {
        match self {
            VirtualController::Xbox360(_) => panic!(),
            VirtualController::DualShock4(controller) => controller,
        }
    }
}

#[tauri::command]
pub async fn start_controllers(
    state: State<'_, AppState>,
    controllers: Vec<(String, Uuid)>,
) -> Result<(), String> {
    for (profile, connection) in controllers {
        let profile = state
            .profile_repository
            .find_profile_by_name(&profile)
            .await
            .map_err(|err| err.to_string())?
            .map(|profile| Arc::new(profile));

        let connection = state
            .connected_controllers
            .read()
            .await
            .get(&connection)
            .cloned();

        if let Some(profile) = profile
            && let Some(connection) = connection
        {
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

            match &virtual_controller {
                VirtualController::Xbox360(target) => target.wait_for_ready(),
                VirtualController::DualShock4(target) => target.wait_for_ready(),
            }
            .map_err(|err| err.to_string())?;

            match &*connection.clone() {
                ConnectedController::SingleJoyCon(joy_con) => {
                    let peripheral = joy_con.device.peripheral.clone();
                    let input_char = joy_con.device.input_char.clone();
                    let joy_con_side = joy_con.joy_con_side;

                    tauri::async_runtime::spawn(async move {
                        let decoder = Decoder;
                        let evaluator = Evaluator;
                        let encoder = Encoder::new();

                        let mut stream = peripheral.notifications().await.unwrap();
                        while let Some(notification) = stream.next().await {
                            if notification.uuid != input_char.uuid {
                                continue;
                            }

                            let data = notification.value;
                            let input = match joy_con_side {
                                JoyConSide::Left => decoder.decode_left_joycon(&data),
                                JoyConSide::Right => decoder.decode_right_joycon(&data),
                            };

                            let output = evaluator.evaluate_profile(&profile, &input);

                            match profile.kind {
                                ProfileKind::Ps4 => {
                                    let report = encoder.encode_ps4(&output);
                                    let _ = virtual_controller.get_ps4().update_ex(&report);
                                }
                                ProfileKind::Xbox360 => {
                                    let report = encoder.encode_xbox(&output);
                                    let _ = virtual_controller.get_360().update(&report);
                                }
                            }
                        }
                    });
                }

                ConnectedController::DualJoyCon(joy_cons) => {
                    let left_buffer = Arc::new(Mutex::new(Arc::new(vec![])));
                    let right_buffer = Arc::new(Mutex::new(Arc::new(vec![])));

                    let left_left_buffer = left_buffer.clone();
                    let left_input_char = joy_cons.left.input_char.clone();
                    let left_peripheral = joy_cons.left.peripheral.clone();

                    tauri::async_runtime::spawn(async move {
                        let mut stream = left_peripheral.notifications().await.unwrap();
                        while let Some(notification) = stream.next().await {
                            if notification.uuid != left_input_char.uuid {
                                continue;
                            }

                            let data = Arc::new(notification.value);
                            *left_left_buffer.lock().await = data;
                        }
                    });

                    let right_right_buffer = right_buffer.clone();
                    let right_peripheral = joy_cons.right.peripheral.clone();
                    let right_input_char = joy_cons.right.input_char.clone();

                    tauri::async_runtime::spawn(async move {
                        let mut stream = right_peripheral.notifications().await.unwrap();
                        while let Some(notification) = stream.next().await {
                            if notification.uuid != right_input_char.uuid {
                                continue;
                            }

                            let data = Arc::new(notification.value);
                            *right_right_buffer.lock().await = data;
                        }
                    });

                    let profile = profile.clone();
                    let virtual_controller = virtual_controller.clone();

                    tauri::async_runtime::spawn(async move {
                        let decoder = Decoder;
                        let evaluator = Evaluator;
                        let encoder = Encoder::new();

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
                                MotionSource::Right,
                            );

                            let output = evaluator.evaluate_profile(&profile, &input);

                            match profile.kind {
                                ProfileKind::Ps4 => {
                                    let report = encoder.encode_ps4(&output);
                                    let _ = virtual_controller.get_ps4().update_ex(&report);
                                }
                                ProfileKind::Xbox360 => {
                                    let report = encoder.encode_xbox(&output);
                                    let _ = virtual_controller.get_360().update(&report);
                                }
                            }
                        }
                    });
                }

                ConnectedController::ProController(controller) => {
                    let peripheral = controller.device.peripheral.clone();
                    let input_char = controller.device.input_char.clone();

                    tauri::async_runtime::spawn(async move {
                        let decoder = Decoder;
                        let evaluator = Evaluator;
                        let encoder = Encoder::new();

                        let mut stream = peripheral.notifications().await.unwrap();
                        while let Some(notification) = stream.next().await {
                            if notification.uuid != input_char.uuid {
                                continue;
                            }

                            let data = notification.value;
                            let input = decoder.decode_pro_controller(&data);

                            let output = evaluator.evaluate_profile(&profile, &input);

                            match profile.kind {
                                ProfileKind::Ps4 => {
                                    let report = encoder.encode_ps4(&output);
                                    let _ = virtual_controller.get_ps4().update_ex(&report);
                                }
                                ProfileKind::Xbox360 => {
                                    let report = encoder.encode_xbox(&output);
                                    let _ = virtual_controller.get_360().update(&report);
                                }
                            }
                        }
                    });
                }

                ConnectedController::NsoGcController(controller) => {
                    let peripheral = controller.device.peripheral.clone();
                    let input_char = controller.device.input_char.clone();

                    tauri::async_runtime::spawn(async move {
                        let decoder = Decoder;
                        let evaluator = Evaluator;
                        let encoder = Encoder::new();

                        let mut stream = peripheral.notifications().await.unwrap();
                        while let Some(notification) = stream.next().await {
                            if notification.uuid != input_char.uuid {
                                continue;
                            }

                            let data = notification.value;
                            let input = decoder.decode_gc_controller(&data);

                            let output = evaluator.evaluate_profile(&profile, &input);

                            match profile.kind {
                                ProfileKind::Ps4 => {
                                    let report = encoder.encode_ps4(&output);
                                    let _ = virtual_controller.get_ps4().update_ex(&report);
                                }
                                ProfileKind::Xbox360 => {
                                    let report = encoder.encode_xbox(&output);
                                    let _ = virtual_controller.get_360().update(&report);
                                }
                            }
                        }
                    });
                }
            }
        }
    }

    Ok(())
}
