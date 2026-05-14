use crate::data::ps4_controller_data::Ps4ControllerData;
use crate::data::xbox360_controller_data::Xbox360ControllerData;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum ControllerData {
    Ps4(Ps4ControllerData),
    Xbox360(Xbox360ControllerData),
}
