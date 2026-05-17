use crate::data::motion_data::MotionData;
use crate::data::output::Output;
use crate::data::output::Output::{L2Lt, R2Rt};
use crate::data::output_data::OutputData;
use crate::data::trigger_data::TriggerData;
use vigem_rust::controller::ds4::{Ds4ReportEx, Ds4ReportExData, Ds4SpecialButton};
use vigem_rust::{Ds4Button, Ds4Dpad};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Ds4Encoder;

struct StickData {
    lx: u8,
    ly: u8,
    rx: u8,
    ry: u8,
}

impl Ds4Encoder {
    const STICK_CENTER: f32 = 128_f32;

    fn axis(&self, data: &OutputData, minus: Output, plus: Output) -> f32 {
        let minus = data.get(minus).unwrap_or(0_f32);
        let plus = data.get(plus).unwrap_or(0_f32);

        minus + plus
    }

    fn stick(&self, data: &OutputData, minus: Output, plus: Output) -> u8 {
        let axis = self.axis(data, minus, plus);

        (axis * i8::MAX as f32 + Self::STICK_CENTER) as u8
    }

    fn encode_sticks(&self, data: &OutputData) -> StickData {
        let lx = self.stick(data, Output::LeftXMinus, Output::LeftXPlus);
        let ly = 255 - self.stick(data, Output::LeftYMinus, Output::LeftYPlus);
        let rx = self.stick(data, Output::RightXMinus, Output::RightXPlus);
        let ry = 255 - self.stick(data, Output::RightYMinus, Output::RightYPlus);

        StickData { lx, ly, rx, ry }
    }

    const THRESHOLD: f32 = 0.5;

    fn is_button_pressed(&self, data: &OutputData, output: Output) -> bool {
        data.get(output)
            .is_some_and(|value| value >= Self::THRESHOLD)
    }

    fn encode_buttons(&self, data: &OutputData) -> (Ds4Button, Ds4SpecialButton) {
        let mut buttons = Ds4Button::empty();
        let mut special = Ds4SpecialButton::empty();

        if self.is_button_pressed(data, Output::R1Rb) {
            buttons |= Ds4Button::SHOULDER_RIGHT;
        }

        if self.is_button_pressed(data, Output::L1Lb) {
            buttons |= Ds4Button::SHOULDER_LEFT;
        }

        if self.is_button_pressed(data, Output::Share) {
            buttons |= Ds4Button::SHARE;
        }

        if self.is_button_pressed(data, Output::OptionsStart) {
            buttons |= Ds4Button::OPTIONS;
        }

        if self.is_button_pressed(data, Output::TriangleY) {
            buttons |= Ds4Button::TRIANGLE;
        }

        if self.is_button_pressed(data, Output::SquareX) {
            buttons |= Ds4Button::SQUARE;
        }

        if self.is_button_pressed(data, Output::CircleB) {
            buttons |= Ds4Button::CIRCLE;
        }

        if self.is_button_pressed(data, Output::CrossA) {
            buttons |= Ds4Button::CROSS;
        }

        if self.is_button_pressed(data, Output::L3Ls) {
            buttons |= Ds4Button::THUMB_LEFT;
        }

        if self.is_button_pressed(data, Output::R3Rs) {
            buttons |= Ds4Button::THUMB_RIGHT;
        }

        if self.is_button_pressed(data, Output::PsGuide) {
            special |= Ds4SpecialButton::PS;
        }

        if self.is_button_pressed(data, Output::TouchpadBack) {
            special |= Ds4SpecialButton::TOUCHPAD;
        }

        (buttons, special)
    }

    fn encode_d_pad(&self, data: &OutputData) -> Ds4Dpad {
        let up = self.is_button_pressed(data, Output::Up);
        let down = self.is_button_pressed(data, Output::Down);
        let left = self.is_button_pressed(data, Output::Left);
        let right = self.is_button_pressed(data, Output::Right);

        match (up, down, left, right) {
            (true, true, true, true) => Ds4Dpad::Neutral,

            (true, _, true, true) => Ds4Dpad::North,
            (_, true, true, true) => Ds4Dpad::South,
            (true, true, true, _) => Ds4Dpad::West,
            (true, true, _, true) => Ds4Dpad::East,

            (true, true, _, _) => Ds4Dpad::Neutral,
            (_, _, true, true) => Ds4Dpad::Neutral,

            (true, _, true, _) => Ds4Dpad::NorthWest,
            (true, _, _, true) => Ds4Dpad::NorthEast,
            (_, true, true, _) => Ds4Dpad::SouthWest,
            (_, true, _, true) => Ds4Dpad::SouthEast,

            (true, _, _, _) => Ds4Dpad::North,
            (_, true, _, _) => Ds4Dpad::South,
            (_, _, true, _) => Ds4Dpad::West,
            (_, _, _, true) => Ds4Dpad::East,

            _ => Ds4Dpad::Neutral,
        }
    }

    fn encode_trigger(&self, data: &OutputData, output: Output) -> u8 {
        data.get(output)
            .map(|value| value * u8::MAX as f32)
            .map(|value| value as u8)
            .unwrap_or(0)
    }

    fn encode_triggers(&self, data: &OutputData) -> TriggerData {
        TriggerData {
            l: self.encode_trigger(data, L2Lt),
            r: self.encode_trigger(data, R2Rt),
        }
    }

    fn motion(&self, data: &OutputData, minus: Output, plus: Output) -> i16 {
        let axis = self.axis(data, minus, plus);

        (axis * 16.384) as i16
    }

    fn encode_motion(&self, data: &OutputData) -> MotionData {
        MotionData {
            gyro_x: self.motion(data, Output::GyroPitchDown, Output::GyroPitchUp),
            gyro_y: self.motion(data, Output::GyroYawLeft, Output::GyroYawRight),
            gyro_z: self.motion(data, Output::GyroRollLeft, Output::GyroRollRight),
            accel_x: self.motion(data, Output::AccelLeft, Output::AccelRight),
            accel_y: self.motion(data, Output::AccelDown, Output::AccelUp),
            accel_z: self.motion(data, Output::AccelBackward, Output::AccelForward),
        }
    }

    pub fn encode(&self, data: &OutputData) -> Ds4ReportEx {
        let sticks = self.encode_sticks(data);
        let (buttons, special) = self.encode_buttons(data);
        let d_pad = self.encode_d_pad(data);
        let trigger = self.encode_triggers(data);
        let motion = self.encode_motion(data);

        let mut report = Ds4ReportExData {
            thumb_lx: sticks.lx,
            thumb_ly: sticks.ly,
            thumb_rx: sticks.rx,
            thumb_ry: sticks.ry,
            buttons: buttons.bits(),
            special: special.bits(),
            trigger_l: trigger.l,
            trigger_r: trigger.r,
            gyro_x: motion.gyro_x,
            gyro_y: motion.gyro_y,
            gyro_z: motion.gyro_z,
            accel_x: motion.accel_x,
            accel_y: motion.accel_y,
            accel_z: motion.accel_z,
            ..Default::default()
        };

        report.set_dpad(d_pad);

        Ds4ReportEx { report }
    }
}
