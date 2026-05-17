use crate::connection::joy_con_side::JoyConSide;
use crate::connection::motion_source::MotionSource;
use btleplug::api::Characteristic;
use btleplug::platform::Peripheral;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct ConnectedDevice {
    pub peripheral: Peripheral,
    pub input_char: Arc<Characteristic>,
    pub write_char: Arc<Characteristic>,
}

#[derive(Clone, Debug)]
pub struct ConnectedSingleJoyCon {
    pub device: ConnectedDevice,
    pub joy_con_side: JoyConSide,
}

#[derive(Clone, Debug)]
pub struct ConnectedDualJoyCon {
    pub left: ConnectedDevice,
    pub right: ConnectedDevice,
    pub motion_source: MotionSource,
}

#[derive(Clone, Debug)]
pub struct ConnectedProController {
    pub device: ConnectedDevice,
}

#[derive(Clone, Debug)]
pub struct ConnectedNsoGcController {
    pub device: ConnectedDevice,
}

#[derive(Clone, Debug)]
pub enum ConnectedController {
    SingleJoyCon(ConnectedSingleJoyCon),
    DualJoyCon(ConnectedDualJoyCon),
    ProController(ConnectedProController),
    NsoGcController(ConnectedNsoGcController),
}
