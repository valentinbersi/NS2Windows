use crate::data::motion_data::MotionData;
use crate::data::stick_data::StickData;
use crate::data::trigger_data::TriggerData;
use bitflags::bitflags;

bitflags! {
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct Ps4Button: u16 {
        const R1 = 0x1;
        const L1 = 0x2;
        const SHARE = 0x4;
        const OPTIONS = 0x8;
        const UP = 0x10;
        const LEFT = 0x20;
        const DOWN = 0x40;
        const RIGHT = 0x80;
        const TRIANGLE = 0x100;
        const SQUARE = 0x200;
        const CIRCLE = 0x400;
        const CROSS = 0x800;
        const L3 = 0x1000;
        const R3 = 0x2000;
        const PS = 0x4000;
        const TOUCHPAD = 0x8000;
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Ps4ControllerData {
    pub left_stick: StickData,
    pub right_stick: StickData,
    pub buttons: Ps4Button,
    pub trigger_data: TriggerData,
    pub motion: MotionData,
}
