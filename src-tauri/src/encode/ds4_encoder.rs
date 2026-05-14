use crate::data::ps4_controller_data::{Ps4Button, Ps4ControllerData};
use bitflags::bitflags_match;
use vigem_rust::controller::ds4::{Ds4ReportEx, Ds4ReportExData, Ds4SpecialButton};
use vigem_rust::{Ds4Button, Ds4Dpad};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Ds4Encoder;

impl Ds4Encoder {
    fn encode_sticks(&self, ps4: &Ps4ControllerData, report: &mut Ds4ReportExData) {
        report.thumb_lx = (ps4.left_stick.x * 127_f32 + 128_f32) as u8;
        report.thumb_ly = (ps4.left_stick.y * 127_f32 + 128_f32) as u8;
        report.thumb_rx = (ps4.right_stick.x * 127_f32 + 128_f32) as u8;
        report.thumb_ry = (ps4.right_stick.y * 127_f32 + 128_f32) as u8;
    }

    fn encode_buttons(&self, ps4: &Ps4ControllerData, report: &mut Ds4ReportExData) {
        if ps4.buttons.contains(Ps4Button::R1) {
            report.buttons |= Ds4Button::SHOULDER_RIGHT.bits();
        }

        if ps4.buttons.contains(Ps4Button::L1) {
            report.buttons |= Ds4Button::SHOULDER_LEFT.bits();
        }

        if ps4.buttons.contains(Ps4Button::SHARE) {
            report.buttons |= Ds4Button::SHARE.bits();
        }

        if ps4.buttons.contains(Ps4Button::OPTIONS) {
            report.buttons |= Ds4Button::OPTIONS.bits();
        }

        if ps4.buttons.contains(Ps4Button::TRIANGLE) {
            report.buttons |= Ds4Button::TRIANGLE.bits();
        }

        if ps4.buttons.contains(Ps4Button::SQUARE) {
            report.buttons |= Ds4Button::SQUARE.bits();
        }

        if ps4.buttons.contains(Ps4Button::CIRCLE) {
            report.buttons |= Ds4Button::CIRCLE.bits();
        }

        if ps4.buttons.contains(Ps4Button::CROSS) {
            report.buttons |= Ds4Button::CROSS.bits();
        }

        if ps4.buttons.contains(Ps4Button::L3) {
            report.buttons |= Ds4Button::THUMB_LEFT.bits();
        }

        if ps4.buttons.contains(Ps4Button::R3) {
            report.buttons |= Ds4Button::THUMB_RIGHT.bits();
        }

        if ps4.buttons.contains(Ps4Button::PS) {
            report.special |= Ds4SpecialButton::PS.bits();
        }

        if ps4.buttons.contains(Ps4Button::TOUCHPAD) {
            report.special |= Ds4SpecialButton::TOUCHPAD.bits();
        }
    }

    fn encode_d_pad(&self, ps4: &Ps4ControllerData, report: &mut Ds4ReportExData) {
        let d_pad = bitflags_match!(ps4.buttons, {
            // Pressing all buttons results in neutral
            Ps4Button::UP | Ps4Button::DOWN | Ps4Button::LEFT | Ps4Button::RIGHT => Ds4Dpad::Neutral,

            // Pressing 3 buttons cancels the two ones pointing in opposite directions
            Ps4Button::UP | Ps4Button::LEFT | Ps4Button::RIGHT => Ds4Dpad::North,
            Ps4Button::DOWN | Ps4Button::LEFT | Ps4Button::RIGHT => Ds4Dpad::South,
            Ps4Button::UP | Ps4Button::DOWN | Ps4Button::LEFT => Ds4Dpad::West,
            Ps4Button::UP | Ps4Button::DOWN | Ps4Button::RIGHT => Ds4Dpad::East,

            // Pressing 2 buttons in opposite directions results in neutral
            Ps4Button::UP | Ps4Button::DOWN => Ds4Dpad::Neutral,
            Ps4Button::LEFT | Ps4Button::RIGHT => Ds4Dpad::Neutral,

            // Pressing 2 buttons "adds" the output
            Ps4Button::UP | Ps4Button::LEFT => Ds4Dpad::NorthWest,
            Ps4Button::UP | Ps4Button::RIGHT => Ds4Dpad::NorthEast,
            Ps4Button::DOWN | Ps4Button::LEFT => Ds4Dpad::SouthWest,
            Ps4Button::DOWN | Ps4Button::RIGHT => Ds4Dpad::SouthEast,

            // Pressing 1 button maps directly to the corresponding direction
            Ps4Button::UP => Ds4Dpad::North,
            Ps4Button::DOWN => Ds4Dpad::South,
            Ps4Button::LEFT => Ds4Dpad::West,
            Ps4Button::RIGHT => Ds4Dpad::East,

            // Pressing 0 buttons results in neutral
           _ => Ds4Dpad::Neutral
        });

        report.set_dpad(d_pad);
    }

    fn encode_triggers(&self, ps4: &Ps4ControllerData, report: &mut Ds4ReportExData) {
        report.trigger_l = ps4.trigger_data.trigger_l;
        report.trigger_r = ps4.trigger_data.trigger_r;
    }

    fn encode_motion(&self, ps4: &Ps4ControllerData, report: &mut Ds4ReportExData) {
        report.gyro_x = ps4.motion.gyro_x;
        report.gyro_y = ps4.motion.gyro_y;
        report.gyro_z = ps4.motion.gyro_z;
        report.accel_x = ps4.motion.accel_x;
        report.accel_y = ps4.motion.accel_y;
        report.accel_z = ps4.motion.accel_z;
    }

    pub fn encode(&self, ps4: Ps4ControllerData) -> Ds4ReportEx {
        let mut report = Ds4ReportExData::default();

        self.encode_sticks(&ps4, &mut report);
        self.encode_buttons(&ps4, &mut report);
        self.encode_d_pad(&ps4, &mut report);
        self.encode_triggers(&ps4, &mut report);
        self.encode_motion(&ps4, &mut report);

        Ds4ReportEx { report }
    }
}
