use crate::data::controller_data::ControllerData;
use crate::data::input_data::InputData;
use crate::data::motion_data::MotionData;
use crate::data::output::Output;
use crate::data::output::Output::{
    AccelBackward, AccelDown, AccelForward, AccelLeft, AccelRight, AccelUp, CircleB, CrossA, Down,
    GyroPitchDown, GyroPitchUp, GyroRollLeft, GyroRollRight, GyroYawLeft, GyroYawRight, L1Lb, L2Lt,
    L3Ls, Left, LeftXMinus, LeftXPlus, LeftYMinus, LeftYPlus, OptionsStart, PsGuide, R1Rb, R2Rt,
    R3Rs, Right, RightXMinus, RightXPlus, RightYMinus, RightYPlus, Share, SquareX, TouchpadBack,
    TriangleY, Up,
};
use crate::data::profile_kind::ProfileKind;
use crate::data::ps4_controller_data::{Ps4Button, Ps4ControllerData};
use crate::data::stick_data::StickData;
use crate::data::trigger_data::TriggerData;
use crate::data::xbox360_controller_data::{Xbox360Button, Xbox360ControllerData};
use crate::profiles::condition::Condition;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Profile {
    pub profile_name: String,
    pub profile_kind: ProfileKind,
    pub outputs: HashMap<Output, Condition>,
}

impl Profile {
    pub fn new(
        profile_name: String,
        kind: ProfileKind,
        outputs: HashMap<Output, Condition>,
    ) -> Self {
        Self {
            profile_name,
            profile_kind: kind,
            outputs,
        }
    }

    fn is_button_pressed(&self, output: Output, input_data: &InputData) -> bool {
        self.outputs
            .get(&output)
            .map(|condition| condition.evaluate(&input_data))
            .unwrap_or(0_f32)
            >= 0.5
    }

    fn axis(&self, input_data: &InputData, minus: Output, plus: Output) -> f32 {
        self.outputs
            .get(&minus)
            .map(|condition| condition.evaluate(input_data))
            .unwrap_or(0_f32)
            + self
                .outputs
                .get(&plus)
                .map(|condition| condition.evaluate(input_data))
                .unwrap_or(0_f32)
    }

    fn stick(
        &self,
        input_data: &InputData,
        x_minus: Output,
        x_plus: Output,
        y_minus: Output,
        y_plus: Output,
    ) -> StickData {
        StickData {
            x: self.axis(input_data, x_minus, x_plus),
            y: self.axis(input_data, y_minus, y_plus),
        }
    }

    fn left_stick(&self, input_data: &InputData) -> StickData {
        self.stick(input_data, LeftXMinus, LeftXPlus, LeftYMinus, LeftYPlus)
    }

    fn right_stick(&self, input_data: &InputData) -> StickData {
        self.stick(input_data, RightXMinus, RightXPlus, RightYMinus, RightYPlus)
    }

    fn triggers(&self, input_data: &InputData) -> TriggerData {
        TriggerData {
            trigger_l: (self
                .outputs
                .get(&L2Lt)
                .map(|condition| condition.evaluate(input_data))
                .unwrap_or(0_f32)
                * 127_f32
                + 128_f32) as u8,

            trigger_r: (self
                .outputs
                .get(&R2Rt)
                .map(|condition| condition.evaluate(input_data))
                .unwrap_or(0_f32)
                * 127_f32
                + 128_f32) as u8,
        }
    }

    fn ps4_buttons(&self, input_data: &InputData) -> Ps4Button {
        let mut buttons = Ps4Button::default();

        if self.is_button_pressed(CircleB, input_data) {
            buttons |= Ps4Button::CIRCLE;
        }

        if self.is_button_pressed(CrossA, input_data) {
            buttons |= Ps4Button::CROSS;
        }

        if self.is_button_pressed(SquareX, input_data) {
            buttons |= Ps4Button::SQUARE;
        }

        if self.is_button_pressed(TriangleY, input_data) {
            buttons |= Ps4Button::TRIANGLE;
        }

        if self.is_button_pressed(PsGuide, input_data) {
            buttons |= Ps4Button::PS;
        }

        if self.is_button_pressed(Share, input_data) {
            buttons |= Ps4Button::SHARE;
        }

        if self.is_button_pressed(R1Rb, input_data) {
            buttons |= Ps4Button::R1;
        }

        if self.is_button_pressed(R3Rs, input_data) {
            buttons |= Ps4Button::R3;
        }

        if self.is_button_pressed(L1Lb, input_data) {
            buttons |= Ps4Button::L1;
        }

        if self.is_button_pressed(L3Ls, input_data) {
            buttons |= Ps4Button::L3;
        }

        if self.is_button_pressed(OptionsStart, input_data) {
            buttons |= Ps4Button::OPTIONS;
        }

        if self.is_button_pressed(TouchpadBack, input_data) {
            buttons |= Ps4Button::TOUCHPAD;
        }

        if self.is_button_pressed(Down, input_data) {
            buttons |= Ps4Button::DOWN;
        }

        if self.is_button_pressed(Left, input_data) {
            buttons |= Ps4Button::LEFT;
        }

        if self.is_button_pressed(Right, input_data) {
            buttons |= Ps4Button::RIGHT;
        }

        if self.is_button_pressed(Up, input_data) {
            buttons |= Ps4Button::UP;
        }

        buttons
    }

    fn motion(&self, input_data: &InputData) -> MotionData {
        MotionData {
            gyro_x: (self.axis(input_data, GyroPitchDown, GyroPitchUp) * 16.384) as i16,
            gyro_y: (self.axis(input_data, GyroYawLeft, GyroYawRight) * 16.384) as i16,
            gyro_z: (self.axis(input_data, GyroRollLeft, GyroRollRight) * 16.384) as i16,
            accel_x: (self.axis(input_data, AccelLeft, AccelRight) * 16.384) as i16,
            accel_y: (self.axis(input_data, AccelDown, AccelUp) * 16.384) as i16,
            accel_z: (self.axis(input_data, AccelBackward, AccelForward) * 16.384) as i16,
        }
    }

    fn xbox_buttons(&self, input_data: &InputData) -> Xbox360Button {
        let mut buttons = Xbox360Button::default();

        if self.is_button_pressed(CircleB, input_data) {
            buttons |= Xbox360Button::B;
        }

        if self.is_button_pressed(CrossA, input_data) {
            buttons |= Xbox360Button::A;
        }

        if self.is_button_pressed(SquareX, input_data) {
            buttons |= Xbox360Button::X;
        }

        if self.is_button_pressed(TriangleY, input_data) {
            buttons |= Xbox360Button::Y;
        }

        if self.is_button_pressed(PsGuide, input_data) {
            buttons |= Xbox360Button::GUIDE;
        }

        if self.is_button_pressed(R1Rb, input_data) {
            buttons |= Xbox360Button::RB;
        }

        if self.is_button_pressed(R3Rs, input_data) {
            buttons |= Xbox360Button::RS;
        }

        if self.is_button_pressed(L1Lb, input_data) {
            buttons |= Xbox360Button::LB;
        }

        if self.is_button_pressed(L3Ls, input_data) {
            buttons |= Xbox360Button::LS;
        }

        if self.is_button_pressed(OptionsStart, input_data) {
            buttons |= Xbox360Button::START;
        }

        if self.is_button_pressed(TouchpadBack, input_data) {
            buttons |= Xbox360Button::BACK;
        }

        if self.is_button_pressed(Down, input_data) {
            buttons |= Xbox360Button::DOWN;
        }

        if self.is_button_pressed(Left, input_data) {
            buttons |= Xbox360Button::LEFT;
        }

        if self.is_button_pressed(Right, input_data) {
            buttons |= Xbox360Button::RIGHT;
        }

        if self.is_button_pressed(Up, input_data) {
            buttons |= Xbox360Button::UP;
        }

        buttons
    }

    pub fn evaluate(&self, input_data: &InputData) -> ControllerData {
        let left_stick = self.left_stick(input_data);
        let right_stick = self.right_stick(input_data);
        let trigger_data = self.triggers(input_data);

        match self.profile_kind {
            ProfileKind::Ps4 => {
                let buttons = self.ps4_buttons(input_data);
                let motion = self.motion(input_data);

                ControllerData::Ps4(Ps4ControllerData {
                    left_stick,
                    right_stick,
                    buttons,
                    trigger_data,
                    motion,
                })
            }

            ProfileKind::Xbox360 => {
                let buttons = self.xbox_buttons(input_data);

                ControllerData::Xbox360(Xbox360ControllerData {
                    left_stick,
                    right_stick,
                    buttons,
                    trigger_data,
                })
            }
        }
    }
}
