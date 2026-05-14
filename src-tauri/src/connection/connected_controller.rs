use crate::connection::joycon_side::JoyConSide;
use btleplug::api::Characteristic;
use btleplug::platform::Peripheral;

#[derive(Clone, Debug)]
pub struct ConnectedDevice {
    pub peripheral: Peripheral,
    pub input_char: Characteristic,
    pub write_char: Characteristic,
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
}

#[derive(Clone, Debug)]
pub struct ConnectedNsoProController {
    pub device: ConnectedDevice,
}

#[derive(Clone, Debug)]
pub enum ConnectedController {
    SingleJoyCon(ConnectedSingleJoyCon),
    DualJoyCon(ConnectedDualJoyCon),
    NsoGcProController(ConnectedNsoProController),
}
