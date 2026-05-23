use crate::connection::joy_con_side::JoyConSide;
use crate::connection::motion_source::MotionSource;
use btleplug::api::{Characteristic, Peripheral as PeripheralApi};
use btleplug::platform::Peripheral;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct ConnectedDevice {
    pub peripheral: Peripheral,
    pub input_char: Arc<Characteristic>,
    pub write_char: Arc<Characteristic>,
}

impl ConnectedDevice {
    pub async fn disconnect(&self) -> btleplug::Result<()> {
        self.peripheral.disconnect().await
    }
}

#[derive(Clone, Debug)]
pub struct ConnectedSingleJoyCon {
    pub device: ConnectedDevice,
    pub joy_con_side: JoyConSide,
}

impl ConnectedSingleJoyCon {
    pub async fn disconnect(&self) -> btleplug::Result<()> {
        self.device.disconnect().await
    }
}

#[derive(Clone, Debug)]
pub struct ConnectedDualJoyCon {
    pub left: ConnectedDevice,
    pub right: ConnectedDevice,
    pub motion_source: MotionSource,
}

impl ConnectedDualJoyCon {
    pub async fn disconnect(&self) -> btleplug::Result<()> {
        self.left.disconnect().await?;
        self.right.disconnect().await
    }
}

#[derive(Clone, Debug)]
pub struct ConnectedProController {
    pub device: ConnectedDevice,
}

impl ConnectedProController {
    pub async fn disconnect(&self) -> btleplug::Result<()> {
        self.device.disconnect().await
    }
}

#[derive(Clone, Debug)]
pub struct ConnectedNsoGcController {
    pub device: ConnectedDevice,
}

impl ConnectedNsoGcController {
    pub async fn disconnect(&self) -> btleplug::Result<()> {
        self.device.disconnect().await
    }
}

#[derive(Clone, Debug)]
pub enum ConnectedController {
    SingleJoyCon(ConnectedSingleJoyCon),
    DualJoyCon(ConnectedDualJoyCon),
    ProController(ConnectedProController),
    NsoGcController(ConnectedNsoGcController),
}

impl ConnectedController {
    pub async fn disconnect(&self) -> btleplug::Result<()> {
        match self {
            ConnectedController::SingleJoyCon(joy_con) => joy_con.disconnect().await,
            ConnectedController::DualJoyCon(joy_cons) => joy_cons.disconnect().await,
            ConnectedController::ProController(controller) => controller.disconnect().await,
            ConnectedController::NsoGcController(controller) => controller.disconnect().await,
        }
    }
}
