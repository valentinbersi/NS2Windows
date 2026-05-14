use crate::data::stick_data::StickData;
use crate::data::trigger_data::TriggerData;
use bitflags::bitflags;

bitflags! {
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct Xbox360Button: u16 {
        const RB = 0x1;
        const LB = 0x2;
        const START = 0x4;
        const BACK = 0x8;
        const UP = 0x10;
        const LEFT = 0x20;
        const DOWN = 0x40;
        const RIGHT = 0x80;
        const Y = 0x100;
        const X = 0x200;
        const B = 0x400;
        const A = 0x800;
        const LS = 0x1000;
        const RS = 0x2000;
        const GUIDE = 0x4000;
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Xbox360ControllerData {
    pub left_stick: StickData,
    pub right_stick: StickData,
    pub buttons: Xbox360Button,
    pub trigger_data: TriggerData,
}
