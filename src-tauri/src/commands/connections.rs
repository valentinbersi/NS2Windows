use crate::communication::communicator::LedPatten;
use crate::data::ns_controller_kind::NsControllerKind;
use crate::decode::decoder::Decoder;
use crate::ns_controller::NsController;
use crate::state::AppState;
use btleplug::api::Peripheral;
use futures::StreamExt;
use std::sync::Arc;
use std::time::Duration;
use tauri::{AppHandle, Emitter, State};
use tokio::task::id;
use tokio::time::sleep;
use uuid::Uuid;
// async fn connect_dual_joy_con(
//     state: &State<'_, AppState>,
//     app: &AppHandle,
// ) -> Result<Arc<NsController>, String> {
//     app.emit("waiting_connection", ControllerKind::LeftJoyCon)
//         .map_err(|err| err.to_string())?;
//
//     let left = state
//         .connector
//         .wait_for_controller()
//         .await
//         .map_err(|err| err.to_string())?;
//
//     app.emit("waiting_connection", ControllerKind::RightJoyCon)
//         .map_err(|err| err.to_string())?;
//
//     let right = state
//         .connector
//         .wait_for_controller()
//         .await
//         .map_err(|err| err.to_string())?;
//
//     let connected_controller = Arc::new(NsController::DualJoyCon(ConnectedDualJoyCon {
//         left,
//         right,
//         motion_source: MotionSource::Right,
//     }));
//
//     Ok(connected_controller)
// }
//
// async fn connect_single_controller(
//     state: &State<'_, AppState>,
//     app: &AppHandle,
//     controller_kind: ControllerKind,
// ) -> Result<Arc<NsController>, String> {
//     app.emit("waiting_connection", controller_kind)
//         .map_err(|err| err.to_string())?;
//
//     let device = state
//         .connector
//         .wait_for_controller()
//         .await
//         .map_err(|err| err.to_string())?;
//
//     let connected_controller = match controller_kind {
//         ControllerKind::LeftJoyCon => NsController::SingleJoyCon(ConnectedSingleJoyCon {
//             device,
//             joy_con_side: JoyConSide::Left,
//         }),
//         ControllerKind::RightJoyCon => NsController::SingleJoyCon(ConnectedSingleJoyCon {
//             device,
//             joy_con_side: JoyConSide::Right,
//         }),
//         ControllerKind::ProController => {
//             NsController::ProController(ConnectedProController { device })
//         }
//         ControllerKind::NsoGcController => {
//             NsController::NsoGcController(ConnectedNsoGcController { device })
//         }
//         _ => return Err("Invalid State".to_string()),
//     };
//
//     let connected_controller = Arc::new(connected_controller);
//
//     Ok(connected_controller)
// }
//
// #[tauri::command]
// pub async fn connect_controller(
//     state: State<'_, AppState>,
//     app: AppHandle,
//     controller_kind: ControllerKind,
// ) -> Result<(), String> {
//     let connected_controller = if let ControllerKind::DualJoyCons = controller_kind {
//         connect_dual_joy_con(&state, &app).await
//     } else {
//         connect_single_controller(&state, &app, controller_kind).await
//     }?;
//
//     let id = Uuid::now_v7();
//
//     state
//         .connected_controllers
//         .write()
//         .await
//         .insert(id, connected_controller.clone());
//
//     app.emit("finishing_connection", controller_kind)
//         .map_err(|err| err.to_string())?;
//
//     match &*connected_controller {
//         NsController::SingleJoyCon(joy_con) => {
//             joy_con
//                 .device
//                 .peripheral
//                 .subscribe(&joy_con.device.input_char)
//                 .await
//                 .map_err(|err| err.to_string())?;
//
//             state
//                 .communicator
//                 .send_custom_command(&joy_con.device)
//                 .await
//                 .map_err(|err| err.to_string())?;
//
//             sleep(Duration::from_millis(200)).await;
//
//             state
//                 .communicator
//                 .set_device_led(&joy_con.device, LedPatten::Led1)
//                 .await
//                 .map_err(|err| err.to_string())?;
//
//             state
//                 .communicator
//                 .emit_sound(&joy_con.device)
//                 .await
//                 .map_err(|err| err.to_string())?;
//         }
//
//         NsController::DualJoyCon(joy_cons) => {
//             joy_cons
//                 .left
//                 .peripheral
//                 .subscribe(&joy_cons.left.input_char)
//                 .await
//                 .map_err(|err| err.to_string())?;
//
//             state
//                 .communicator
//                 .send_custom_command(&joy_cons.left)
//                 .await
//                 .map_err(|err| err.to_string())?;
//
//             sleep(Duration::from_millis(200)).await;
//
//             state
//                 .communicator
//                 .set_device_led(&joy_cons.left, LedPatten::Led1)
//                 .await
//                 .map_err(|err| err.to_string())?;
//
//             state
//                 .communicator
//                 .emit_sound(&joy_cons.left)
//                 .await
//                 .map_err(|err| err.to_string())?;
//
//             joy_cons
//                 .right
//                 .peripheral
//                 .subscribe(&joy_cons.right.input_char)
//                 .await
//                 .map_err(|err| err.to_string())?;
//
//             state
//                 .communicator
//                 .send_custom_command(&joy_cons.right)
//                 .await
//                 .map_err(|err| err.to_string())?;
//
//             sleep(Duration::from_millis(200)).await;
//
//             state
//                 .communicator
//                 .set_device_led(&joy_cons.right, LedPatten::Led1)
//                 .await
//                 .map_err(|err| err.to_string())?;
//
//             state
//                 .communicator
//                 .emit_sound(&joy_cons.right)
//                 .await
//                 .map_err(|err| err.to_string())?;
//         }
//
//         NsController::ProController(controller) => {
//             controller
//                 .device
//                 .peripheral
//                 .subscribe(&controller.device.input_char)
//                 .await
//                 .map_err(|err| err.to_string())?;
//
//             state
//                 .communicator
//                 .send_custom_command(&controller.device)
//                 .await
//                 .map_err(|err| err.to_string())?;
//
//             sleep(Duration::from_millis(200)).await;
//
//             state
//                 .communicator
//                 .set_device_led(&controller.device, LedPatten::Led1)
//                 .await
//                 .map_err(|err| err.to_string())?;
//
//             state
//                 .communicator
//                 .emit_sound(&controller.device)
//                 .await
//                 .map_err(|err| err.to_string())?;
//         }
//
//         NsController::NsoGcController(controller) => {
//             controller
//                 .device
//                 .peripheral
//                 .subscribe(&controller.device.input_char)
//                 .await
//                 .map_err(|err| err.to_string())?;
//
//             state
//                 .communicator
//                 .send_custom_command(&controller.device)
//                 .await
//                 .map_err(|err| err.to_string())?;
//
//             sleep(Duration::from_millis(200)).await;
//
//             state
//                 .communicator
//                 .set_device_led(&controller.device, LedPatten::Led1)
//                 .await
//                 .map_err(|err| err.to_string())?;
//
//             state
//                 .communicator
//                 .emit_sound(&controller.device)
//                 .await
//                 .map_err(|err| err.to_string())?;
//         }
//     }
//
//     Ok(())
// }

#[tauri::command]
pub async fn connect_controller(
    state: State<'_, AppState>,
    app: AppHandle,
    kind: NsControllerKind,
) -> Result<Uuid, String> {
    let id = Uuid::now_v7();

    // Alert waiting of connection
    app.emit("waiting connection", (id, kind))
        .map_err(|err| err.to_string())?;

    // Wait for a controller
    let device = state
        .connector
        .wait_for_controller()
        .await
        .map_err(|err| err.to_string())?;

    // Alert start of connection configuration
    app.emit("configuring connection", (id, kind))
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
            let _ = app.emit("update input", (id, input));
        }
    });

    let controller = Arc::new(NsController::new(kind, device, input_informer));

    state
        .connected_controllers
        .write()
        .await
        .entry(id)
        .insert_entry(controller);

    Ok(id)
}

// #[tauri::command]
// pub async fn get_connections(state: State<'_, AppState>) -> Result<Vec<Connection>, ()> {
//     Ok(state
//         .connected_controllers
//         .read()
//         .await
//         .iter()
//         .map(|(id, connected_controller)| Connection {
//             id: *id,
//             controller_kind: match &**connected_controller {
//                 NsController::SingleJoyCon(joy_con) => match joy_con.joy_con_side {
//                     JoyConSide::Left => ControllerKind::LeftJoyCon,
//                     JoyConSide::Right => ControllerKind::RightJoyCon,
//                 },
//                 NsController::DualJoyCon(_) => ControllerKind::DualJoyCons,
//                 NsController::ProController(_) => ControllerKind::ProController,
//                 NsController::NsoGcController(_) => ControllerKind::NsoGcController,
//             },
//         })
//         .collect())
// }
//
// #[tauri::command]
// pub async fn remove_connection(state: State<'_, AppState>, id: Uuid) -> Result<(), String> {
//     let controller = state.connected_controllers.write().await.remove(&id);
//
//     if let Some(controller) = controller {
//         match &*controller {
//             NsController::SingleJoyCon(joy_con) => joy_con
//                 .device
//                 .peripheral
//                 .disconnect()
//                 .await
//                 .map_err(|err| err.to_string())?,
//
//             NsController::DualJoyCon(joy_cons) => {
//                 joy_cons
//                     .left
//                     .peripheral
//                     .disconnect()
//                     .await
//                     .map_err(|err| err.to_string())?;
//                 joy_cons
//                     .right
//                     .peripheral
//                     .disconnect()
//                     .await
//                     .map_err(|err| err.to_string())?
//             }
//
//             NsController::ProController(controller) => controller
//                 .device
//                 .peripheral
//                 .disconnect()
//                 .await
//                 .map_err(|err| err.to_string())?,
//
//             NsController::NsoGcController(controller) => controller
//                 .device
//                 .peripheral
//                 .disconnect()
//                 .await
//                 .map_err(|err| err.to_string())?,
//         }
//     }
//
//     Ok(())
// }
