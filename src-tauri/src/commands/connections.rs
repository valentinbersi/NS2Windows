use crate::communication::communicator::LedPatten;
use crate::connection::connected_controller::{
    ConnectedController, ConnectedDualJoyCon, ConnectedNsoProController, ConnectedSingleJoyCon,
};
use crate::connection::joycon_side::JoyConSide;
use crate::dtos::connection::Connection;
use crate::dtos::controller_kind::ControllerKind;
use crate::state::AppState;
use btleplug::api::Peripheral;
use sea_orm::sqlx::__rt::sleep;
use std::sync::Arc;
use std::time::Duration;
use tauri::{AppHandle, Emitter, State};
use uuid::Uuid;

async fn connect_dual_joy_con(
    state: &State<'_, AppState>,
    app: &AppHandle,
) -> Result<Arc<ConnectedController>, String> {
    app.emit("waiting_connection", ControllerKind::LeftJoyCon)
        .map_err(|err| err.to_string())?;

    let left = state
        .connector
        .wait_for_controller()
        .await
        .map_err(|err| err.to_string())?;

    app.emit("waiting_connection", ControllerKind::RightJoyCon)
        .map_err(|err| err.to_string())?;

    let right = state
        .connector
        .wait_for_controller()
        .await
        .map_err(|err| err.to_string())?;

    let connected_controller = Arc::new(ConnectedController::DualJoyCon(ConnectedDualJoyCon {
        left,
        right,
    }));

    Ok(connected_controller)
}

async fn connect_single_controller(
    state: &State<'_, AppState>,
    app: &AppHandle,
    controller_kind: ControllerKind,
) -> Result<Arc<ConnectedController>, String> {
    app.emit("waiting_connection", controller_kind)
        .map_err(|err| err.to_string())?;

    let device = state
        .connector
        .wait_for_controller()
        .await
        .map_err(|err| err.to_string())?;

    let connected_controller = match controller_kind {
        ControllerKind::LeftJoyCon => ConnectedController::SingleJoyCon(ConnectedSingleJoyCon {
            device,
            joy_con_side: JoyConSide::Left,
        }),
        ControllerKind::RightJoyCon => ConnectedController::SingleJoyCon(ConnectedSingleJoyCon {
            device,
            joy_con_side: JoyConSide::Right,
        }),
        ControllerKind::ProNsoGcController => {
            ConnectedController::NsoGcProController(ConnectedNsoProController { device })
        }
        _ => return Err("Invalid State".to_string()),
    };

    let connected_controller = Arc::new(connected_controller);

    Ok(connected_controller)
}

#[tauri::command]
pub async fn connect_controller(
    state: State<'_, AppState>,
    app: AppHandle,
    controller_kind: ControllerKind,
) -> Result<(), String> {
    let connected_controller = if let ControllerKind::DualJoyCons = controller_kind {
        connect_dual_joy_con(&state, &app).await
    } else {
        connect_single_controller(&state, &app, controller_kind).await
    }?;

    let id = Uuid::now_v7();

    state
        .connected_controllers
        .write()
        .await
        .insert(id, connected_controller.clone());

    app.emit("finishing_connection", controller_kind)
        .map_err(|err| err.to_string())?;

    match &*connected_controller {
        ConnectedController::SingleJoyCon(joy_con) => {
            joy_con
                .device
                .peripheral
                .subscribe(&joy_con.device.input_char)
                .await
                .map_err(|err| err.to_string())?;

            state
                .communicator
                .send_custom_command(&joy_con.device)
                .await
                .map_err(|err| err.to_string())?;

            sleep(Duration::from_millis(200)).await;

            state
                .communicator
                .set_device_led(&joy_con.device, LedPatten::Led1)
                .await
                .map_err(|err| err.to_string())?;

            state
                .communicator
                .emit_sound(&joy_con.device)
                .await
                .map_err(|err| err.to_string())?;
        }

        ConnectedController::DualJoyCon(joy_cons) => {
            joy_cons
                .left
                .peripheral
                .subscribe(&joy_cons.left.input_char)
                .await
                .map_err(|err| err.to_string())?;

            state
                .communicator
                .send_custom_command(&joy_cons.left)
                .await
                .map_err(|err| err.to_string())?;

            sleep(Duration::from_millis(200)).await;

            state
                .communicator
                .set_device_led(&joy_cons.left, LedPatten::Led1)
                .await
                .map_err(|err| err.to_string())?;

            state
                .communicator
                .emit_sound(&joy_cons.left)
                .await
                .map_err(|err| err.to_string())?;

            joy_cons
                .right
                .peripheral
                .subscribe(&joy_cons.right.input_char)
                .await
                .map_err(|err| err.to_string())?;

            state
                .communicator
                .send_custom_command(&joy_cons.right)
                .await
                .map_err(|err| err.to_string())?;

            sleep(Duration::from_millis(200)).await;

            state
                .communicator
                .set_device_led(&joy_cons.right, LedPatten::Led1)
                .await
                .map_err(|err| err.to_string())?;

            state
                .communicator
                .emit_sound(&joy_cons.right)
                .await
                .map_err(|err| err.to_string())?;
        }

        ConnectedController::NsoGcProController(controller) => {
            controller
                .device
                .peripheral
                .subscribe(&controller.device.input_char)
                .await
                .map_err(|err| err.to_string())?;

            state
                .communicator
                .send_custom_command(&controller.device)
                .await
                .map_err(|err| err.to_string())?;

            sleep(Duration::from_millis(200)).await;

            state
                .communicator
                .set_device_led(&controller.device, LedPatten::Led1)
                .await
                .map_err(|err| err.to_string())?;

            state
                .communicator
                .emit_sound(&controller.device)
                .await
                .map_err(|err| err.to_string())?;
        }
    }

    Ok(())
}

#[tauri::command]
pub async fn get_connections(state: State<'_, AppState>) -> Result<Vec<Connection>, ()> {
    Ok(state
        .connected_controllers
        .read()
        .await
        .iter()
        .map(|(id, connected_controller)| Connection {
            id: *id,
            controller_kind: match &**connected_controller {
                ConnectedController::SingleJoyCon(joy_con) => match joy_con.joy_con_side {
                    JoyConSide::Left => ControllerKind::LeftJoyCon,
                    JoyConSide::Right => ControllerKind::RightJoyCon,
                },
                ConnectedController::DualJoyCon(_) => ControllerKind::DualJoyCons,
                ConnectedController::NsoGcProController(_) => ControllerKind::ProNsoGcController,
            },
        })
        .collect())
}

#[tauri::command]
pub async fn remove_connection(state: State<'_, AppState>, id: Uuid) -> Result<(), String> {
    let controller = state.connected_controllers.write().await.remove(&id);

    if let Some(controller) = controller {
        match &*controller {
            ConnectedController::SingleJoyCon(joy_con) => joy_con
                .device
                .peripheral
                .disconnect()
                .await
                .map_err(|err| err.to_string())?,

            ConnectedController::DualJoyCon(joy_cons) => {
                joy_cons
                    .left
                    .peripheral
                    .disconnect()
                    .await
                    .map_err(|err| err.to_string())?;
                joy_cons
                    .right
                    .peripheral
                    .disconnect()
                    .await
                    .map_err(|err| err.to_string())?
            }

            ConnectedController::NsoGcProController(controller) => controller
                .device
                .peripheral
                .disconnect()
                .await
                .map_err(|err| err.to_string())?,
        }
    }

    Ok(())
}
