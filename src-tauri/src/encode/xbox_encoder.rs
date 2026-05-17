use crate::data::output::Output;
use crate::data::output::Output::{L2Lt, R2Rt};
use crate::data::output_data::OutputData;
use crate::data::trigger_data::TriggerData;
use vigem_rust::{X360Button, X360Report};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct XboxEncoder;

struct StickData {
    lx: i16,
    ly: i16,
    rx: i16,
    ry: i16,
}

impl XboxEncoder {
    const STICK_CENTER: i16 = 0;

    fn axis(&self, data: &OutputData, minus: Output, plus: Output) -> f32 {
        let minus = data.get(minus).unwrap_or(0_f32);
        let plus = data.get(plus).unwrap_or(0_f32);

        minus + plus
    }

    fn stick(&self, data: &OutputData, minus: Output, plus: Output) -> i16 {
        let axis = self.axis(data, minus, plus);

        (axis * i16::MAX as f32) as i16
    }

    fn encode_sticks(&self, data: &OutputData) -> StickData {
        let lx = self.stick(data, Output::LeftXMinus, Output::LeftXPlus);
        let ly = self.stick(data, Output::LeftYMinus, Output::LeftYPlus);
        let rx = self.stick(data, Output::RightXMinus, Output::RightXPlus);
        let ry = self.stick(data, Output::RightYMinus, Output::RightYPlus);

        StickData { lx, ly, rx, ry }
    }

    const THRESHOLD: f32 = 0.5;

    fn is_button_pressed(&self, data: &OutputData, output: Output) -> bool {
        data.get(output)
            .is_some_and(|value| value >= Self::THRESHOLD)
    }

    fn encode_buttons(&self, data: &OutputData) -> X360Button {
        let mut buttons = X360Button::empty();

        if self.is_button_pressed(data, Output::R1Rb) {
            buttons |= X360Button::RIGHT_SHOULDER;
        }

        if self.is_button_pressed(data, Output::L1Lb) {
            buttons |= X360Button::LEFT_SHOULDER;
        }

        if self.is_button_pressed(data, Output::OptionsStart) {
            buttons |= X360Button::START;
        }

        if self.is_button_pressed(data, Output::TriangleY) {
            buttons |= X360Button::Y;
        }

        if self.is_button_pressed(data, Output::SquareX) {
            buttons |= X360Button::X;
        }

        if self.is_button_pressed(data, Output::CircleB) {
            buttons |= X360Button::B;
        }

        if self.is_button_pressed(data, Output::CrossA) {
            buttons |= X360Button::A;
        }

        if self.is_button_pressed(data, Output::L3Ls) {
            buttons |= X360Button::LEFT_THUMB;
        }

        if self.is_button_pressed(data, Output::R3Rs) {
            buttons |= X360Button::RIGHT_THUMB;
        }

        if self.is_button_pressed(data, Output::PsGuide) {
            buttons |= X360Button::GUIDE;
        }

        if self.is_button_pressed(data, Output::TouchpadBack) {
            buttons |= X360Button::BACK;
        }

        if self.is_button_pressed(data, Output::Up) {
            buttons |= X360Button::DPAD_UP;
        }

        if self.is_button_pressed(data, Output::Down) {
            buttons |= X360Button::DPAD_DOWN;
        }

        if self.is_button_pressed(data, Output::Left) {
            buttons |= X360Button::DPAD_LEFT;
        }

        if self.is_button_pressed(data, Output::Right) {
            buttons |= X360Button::DPAD_RIGHT;
        }

        buttons
    }

    fn encode_trigger(&self, data: &OutputData, output: Output) -> u8 {
        data.get(output)
            .map(|value| value * u8::MAX as f32)
            .map(|value| value as u8)
            .unwrap_or(0)
    }

    fn encode_triggers(&self, data: &OutputData) -> TriggerData {
        TriggerData {
            l: self.encode_trigger(data, R2Rt),
            r: self.encode_trigger(data, L2Lt),
        }
    }

    pub fn encode(&self, data: &OutputData) -> X360Report {
        let sticks = self.encode_sticks(data);
        let buttons = self.encode_buttons(data);
        let triggers = self.encode_triggers(data);

        X360Report {
            buttons,
            left_trigger: triggers.l,
            right_trigger: triggers.r,
            thumb_lx: sticks.lx,
            thumb_ly: sticks.ly,
            thumb_rx: sticks.rx,
            thumb_ry: sticks.ry,
        }
    }
}
