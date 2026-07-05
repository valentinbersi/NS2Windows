use crate::data::input_data::InputData;
use crate::data::ns_input::NsInput;
use crate::dtos::motion_source::MotionSource;
use bitflags::{bitflags, Flags};
use maplit::hashmap;
use std::collections::HashMap;
use std::ops::{BitAnd, Range};

bitflags! {
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    struct Switch2ControllerButtons: u32 {
        const Gr = 0x00_00_00_01;
        const Gl = 0x00_00_00_02;

        const Unused1 = 0x00_00_00_04;
        const Unused2 = 0x00_00_00_08;

        const Headset = 0x00_00_00_10;

        const Unused3 = 0x00_00_00_20;
        const Unused4 = 0x00_00_00_40;
        const Unused5 = 0x00_00_00_80;

        const Down = 0x00_00_01_00;
        const Up = 0x00_00_02_00;
        const Right = 0x00_00_04_00;
        const Left = 0x00_00_08_00;

        const SrLeft = 0x00_00_10_00;
        const SlLeft = 0x00_00_20_00;
        const L = 0x00_00_40_00;
        const Zl = 0x00_00_80_00;

        const Minus = 0x00_01_00_00;
        const Plus = 0x00_02_00_00;

        const RightStick = 0x00_04_00_00;
        const LeftStick = 0x00_08_00_00;

        const Home = 0x00_10_00_00;
        const Capture = 0x00_20_00_00;
        const C = 0x00_40_00_00;

        const Unused6 = 0x00_80_00_00;

        const Y = 0x01_00_00_00;
        const X = 0x02_00_00_00;
        const B = 0x04_00_00_00;
        const A = 0x08_00_00_00;

        const SrRight = 0x10_00_00_00;
        const SlRight = 0x20_00_00_00;
        const R = 0x40_00_00_00;
        const Zr = 0x80_00_00_00;
    }

    // #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    // struct LeftJoyConButtons: u16 {
    //     const Capture = 0x00_01;
    //     const Sr = 0x00_40;
    //     const Sl = 0x00_80;
    //     const Down = 0x01_00;
    //     const Right = 0x02_00;
    //     const Left = 0x04_00;
    //     const Up = 0x08_00;
    //     const L = 0x10_00;
    //     const Zl = 0x20_00;
    //     const Minus = 0x40_00;
    //     const Stick = 0x80_00;
    // }
    //
    // #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    // struct RightJoyConButtons: u16 {
    //     const Home = 0x00_01;
    //     const C = 0x00_10;
    //     const Sr = 0x00_40;
    //     const Sl = 0x00_80;
    //     const B = 0x01_00;
    //     const A = 0x02_00;
    //     const Y = 0x04_00;
    //     const X = 0x08_00;
    //     const R = 0x10_00;
    //     const Zr = 0x20_00;
    //     const Plus = 0x40_00;
    //     const Stick = 0x80_00;
    // }
    //
    // #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    // struct ProControllerButtons: u32 {
    //     const Home = 0x00_00_01;
    //     const Capture = 0x00_00_02;
    //     const Gr = 0x00_00_04;
    //     const Gl = 0x00_00_08;
    //     const C = 0x00_00_10;
    //     const Down = 0x00_01_00;
    //     const Right = 0x00_02_00;
    //     const Left = 0x00_04_00;
    //     const Up = 0x00_08_00;
    //     const L = 0x00_10_00;
    //     const Zl = 0x00_20_00;
    //     const Minus = 0x00_40_00;
    //     const LeftStick = 0x00_80_00;
    //     const B = 0x01_00_00;
    //     const A = 0x02_00_00;
    //     const Y = 0x04_00_00;
    //     const X = 0x08_00_00;
    //     const R = 0x10_00_00;
    //     const Zr = 0x20_00_00;
    //     const Plus = 0x40_00_00;
    //     const RightStick = 0x80_00_00;
    // }
    //
    // #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    // struct NsoGcControllerButtons: u32 {
    //     const Home = 0x00_00_01;
    //     const Capture = 0x00_00_02;
    //     const C = 0x00_00_10;
    //     const Down = 0x00_01_00;
    //     const Right = 0x00_02_00;
    //     const Left = 0x00_04_00;
    //     const Up = 0x00_08_00;
    //     const L = 0x00_10_00;
    //     const Zl = 0x00_20_00;
    //     const Minus = 0x00_40_00;
    //     const LeftStick = 0x00_80_00;
    //     const B = 0x01_00_00;
    //     const A = 0x02_00_00;
    //     const Y = 0x04_00_00;
    //     const X = 0x08_00_00;
    //     const R = 0x10_00_00;
    //     const Zr = 0x20_00_00;
    //     const Plus = 0x40_00_00;
    //     const RightStick = 0x80_00_00;
    // }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Decoder;

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
struct StickData {
    pub x: f32,
    pub y: f32,
}

impl Decoder {
    // ------------ joystick decoding ------------

    fn decode_stick(&self, buffer: &[u8]) -> StickData {
        let x = (u16::from_le_bytes([buffer[0], buffer[1]]) & 0x0FFF) as i32;
        let y = (u16::from_le_bytes([buffer[1], buffer[2]]) >> 4) as i32;

        let mut x = (x - 2048) as f32 / 2048_f32;
        let mut y = (y - 2048) as f32 / 2048_f32;

        let dead_zone = 0.08;
        if x.abs() < dead_zone && y.abs() < dead_zone {
            return StickData { x: 0_f32, y: 0_f32 };
        }

        x = (x * 1.7).clamp(-1_f32, 1_f32);
        y = (y * 1.7).clamp(-1_f32, 1_f32);

        StickData { x, y }
    }

    fn decode_left_stick(&self, buffer: &[u8]) -> HashMap<NsInput, f32> {
        const LEFT_STICK_BYTES: Range<usize> = 0xA..0xA + 0x3;

        let analog_stick = &buffer[LEFT_STICK_BYTES];
        let stick = self.decode_stick(analog_stick);

        let x_minus = -stick.x.clamp(-1_f32, 0_f32);
        let x_plus = stick.x.clamp(0_f32, 1_f32);
        let y_minus = -stick.y.clamp(-1_f32, 0_f32);
        let y_plus = stick.y.clamp(0_f32, 1_f32);

        hashmap! {
            NsInput::LeftXMinus => x_minus,
            NsInput::LeftXPlus => x_plus,
            NsInput::LeftYMinus => y_minus,
            NsInput::LeftYPlus => y_plus,
        }
    }

    fn decode_right_stick(&self, buffer: &[u8]) -> HashMap<NsInput, f32> {
        const RIGHT_STICK_BYTES: Range<usize> = 0xD..0xD + 0x3;

        let analog_stick = &buffer[RIGHT_STICK_BYTES];
        let stick = self.decode_stick(analog_stick);

        let x_minus = -stick.x.clamp(-1_f32, 0_f32);
        let x_plus = stick.x.clamp(0_f32, 1_f32);
        let y_minus = -stick.y.clamp(-1_f32, 0_f32);
        let y_plus = stick.y.clamp(0_f32, 1_f32);

        hashmap! {
            NsInput::RightXMinus => x_minus,
            NsInput::RightXPlus => x_plus,
            NsInput::RightYMinus => y_minus,
            NsInput::RightYPlus => y_plus,
        }
    }

    // fn decode_joy_con_stick(&self, unique_buffer: &[u8]) -> HashMap<NsInput, f32> {
    //     const JOY_CON_STICK_BYTES: Range<usize> = 0x5..0x5 + 0x3;
    //
    //     let analog_stick = &unique_buffer[JOY_CON_STICK_BYTES];
    //     let stick = self.decode_stick(analog_stick);
    //     hashmap! {
    //         NsInput::LeftXMinus => -stick.x.clamp(-1_f32, 0_f32),
    //         NsInput::LeftXPlus => stick.x.clamp(0_f32, 1_f32),
    //         NsInput::LeftYMinus => -stick.y.clamp(-1_f32, 0_f32),
    //         NsInput::LeftYPlus => stick.y.clamp(0_f32, 1_f32),
    //     }
    // }
    //
    // fn decode_controller_stick(&self, unique_buffer: &[u8]) -> HashMap<NsInput, f32> {
    //     const CONTROLLER_LEFT_STICK_BYTES: Range<usize> = 0x5..0x5 + 0x3;
    //     const CONTROLLER_RIGHT_STICK_BYTES: Range<usize> = 0x8..0x8 + 0x3;
    //
    //     let left_analog_stick = &unique_buffer[CONTROLLER_LEFT_STICK_BYTES];
    //     let left_stick = self.decode_stick(left_analog_stick);
    //
    //     let right_analog_stick = &unique_buffer[CONTROLLER_RIGHT_STICK_BYTES];
    //     let right_stick = self.decode_stick(right_analog_stick);
    //
    //     hashmap! {
    //         NsInput::LeftXMinus => -left_stick.x.clamp(-1_f32, 0_f32),
    //         NsInput::LeftXPlus => left_stick.x.clamp(0_f32, 1_f32),
    //         NsInput::LeftYMinus => -left_stick.y.clamp(-1_f32, 0_f32),
    //         NsInput::LeftYPlus => left_stick.y.clamp(0_f32, 1_f32),
    //
    //         NsInput::RightXMinus => -right_stick.x.clamp(-1_f32, 0_f32),
    //         NsInput::RightXPlus => right_stick.x.clamp(0_f32, 1_f32),
    //         NsInput::RightYMinus => -right_stick.y.clamp(-1_f32, 0_f32),
    //         NsInput::RightYPlus => right_stick.y.clamp(0_f32, 1_f32),
    //     }
    // }

    // ------------ buttons decoding ------------

    fn value_from_flags<F: Flags + BitAnd<Output = F> + PartialEq>(buttons: F, flag: F) -> f32 {
        if buttons & flag != F::empty() {
            1_f32
        } else {
            0_f32
        }
    }

    const BUTTONS_BYTES: Range<usize> = 0x4..0x4 + 0x4;

    fn decode_left_joy_con_buttons(&self, buffer: &[u8]) -> HashMap<NsInput, f32> {
        let buttons_buffer = &buffer[Self::BUTTONS_BYTES];
        let buttons_buffer = buttons_buffer.try_into().unwrap();

        let buttons_bit_field = u32::from_be_bytes(buttons_buffer);
        let buttons = Switch2ControllerButtons::from_bits(buttons_bit_field).unwrap();

        hashmap! {
            NsInput::Capture => Self::value_from_flags(buttons, Switch2ControllerButtons::Capture),

            NsInput::Sr => Self::value_from_flags(buttons, Switch2ControllerButtons::SrLeft),
            NsInput::Sl => Self::value_from_flags(buttons, Switch2ControllerButtons::SlLeft),

            NsInput::Down => Self::value_from_flags(buttons, Switch2ControllerButtons::Down),
            NsInput::Right => Self::value_from_flags(buttons, Switch2ControllerButtons::Right),
            NsInput::Left => Self::value_from_flags(buttons, Switch2ControllerButtons::Left),
            NsInput::Up => Self::value_from_flags(buttons, Switch2ControllerButtons::Up),

            NsInput::L => Self::value_from_flags(buttons, Switch2ControllerButtons::L),
            NsInput::Zl => Self::value_from_flags(buttons, Switch2ControllerButtons::Zl),

            NsInput::Minus => Self::value_from_flags(buttons, Switch2ControllerButtons::Minus),

            NsInput::Tl => Self::value_from_flags(buttons, Switch2ControllerButtons::LeftStick),
        }
    }

    fn decode_right_joy_con_buttons(&self, buffer: &[u8]) -> HashMap<NsInput, f32> {
        let buttons_buffer = &buffer[Self::BUTTONS_BYTES];
        let buttons_buffer = buttons_buffer.try_into().unwrap();

        let buttons_bit_field = u32::from_be_bytes(buttons_buffer);
        let buttons = Switch2ControllerButtons::from_bits(buttons_bit_field).unwrap();

        hashmap! {
            NsInput::Home => Self::value_from_flags(buttons, Switch2ControllerButtons::Home),
            NsInput::Chat => Self::value_from_flags(buttons, Switch2ControllerButtons::C),

            NsInput::Sr => Self::value_from_flags(buttons, Switch2ControllerButtons::SrRight),
            NsInput::Sl => Self::value_from_flags(buttons, Switch2ControllerButtons::SlRight),

            NsInput::B => Self::value_from_flags(buttons, Switch2ControllerButtons::B),
            NsInput::A => Self::value_from_flags(buttons, Switch2ControllerButtons::A),
            NsInput::Y => Self::value_from_flags(buttons, Switch2ControllerButtons::Y),
            NsInput::X => Self::value_from_flags(buttons, Switch2ControllerButtons::X),

            NsInput::R => Self::value_from_flags(buttons, Switch2ControllerButtons::R),
            NsInput::Zr => Self::value_from_flags(buttons, Switch2ControllerButtons::Zr),

            NsInput::Plus => Self::value_from_flags(buttons, Switch2ControllerButtons::Plus),

            NsInput::Tr => Self::value_from_flags(buttons, Switch2ControllerButtons::RightStick),
        }
    }

    fn decode_pro_controller_buttons(&self, buffer: &[u8]) -> HashMap<NsInput, f32> {
        let buttons_buffer = &buffer[Self::BUTTONS_BYTES];
        let buttons_buffer = buttons_buffer.try_into().unwrap();

        let buttons_bit_field = u32::from_be_bytes(buttons_buffer);
        let buttons = Switch2ControllerButtons::from_bits(buttons_bit_field).unwrap();

        hashmap! {
            NsInput::Home => Self::value_from_flags(buttons, Switch2ControllerButtons::Home),
            NsInput::Capture => Self::value_from_flags(buttons, Switch2ControllerButtons::Capture),

            NsInput::Gr => Self::value_from_flags(buttons, Switch2ControllerButtons::Gr),
            NsInput::Gl => Self::value_from_flags(buttons, Switch2ControllerButtons::Gl),

            NsInput::Chat => Self::value_from_flags(buttons, Switch2ControllerButtons::C),

            NsInput::Down => Self::value_from_flags(buttons, Switch2ControllerButtons::Down),
            NsInput::Right => Self::value_from_flags(buttons, Switch2ControllerButtons::Right),
            NsInput::Left => Self::value_from_flags(buttons, Switch2ControllerButtons::Left),
            NsInput::Up => Self::value_from_flags(buttons, Switch2ControllerButtons::Up),

            NsInput::L => Self::value_from_flags(buttons, Switch2ControllerButtons::L),
            NsInput::Zl => Self::value_from_flags(buttons, Switch2ControllerButtons::Zl),

            NsInput::Minus => Self::value_from_flags(buttons, Switch2ControllerButtons::Minus),

            NsInput::Tl => Self::value_from_flags(buttons, Switch2ControllerButtons::LeftStick),

            NsInput::B => Self::value_from_flags(buttons, Switch2ControllerButtons::B),
            NsInput::A => Self::value_from_flags(buttons, Switch2ControllerButtons::A),
            NsInput::Y => Self::value_from_flags(buttons, Switch2ControllerButtons::Y),
            NsInput::X => Self::value_from_flags(buttons, Switch2ControllerButtons::X),

            NsInput::R => Self::value_from_flags(buttons, Switch2ControllerButtons::R),
            NsInput::Zr => Self::value_from_flags(buttons, Switch2ControllerButtons::Zr),

            NsInput::Plus => Self::value_from_flags(buttons, Switch2ControllerButtons::Plus),

            NsInput::Tr => Self::value_from_flags(buttons, Switch2ControllerButtons::RightStick),
        }
    }

    fn decode_nso_gc_controller_buttons(&self, buffer: &[u8]) -> HashMap<NsInput, f32> {
        let buttons_buffer = &buffer[Self::BUTTONS_BYTES];
        let buttons_buffer = buttons_buffer.try_into().unwrap();

        let buttons_bit_field = u32::from_be_bytes(buttons_buffer);
        let buttons = Switch2ControllerButtons::from_bits(buttons_bit_field).unwrap();

        hashmap! {
            NsInput::Home => Self::value_from_flags(buttons, Switch2ControllerButtons::Home),
            NsInput::Capture => Self::value_from_flags(buttons, Switch2ControllerButtons::Capture),

            NsInput::Chat => Self::value_from_flags(buttons, Switch2ControllerButtons::C),

            NsInput::Down => Self::value_from_flags(buttons, Switch2ControllerButtons::Down),
            NsInput::Right => Self::value_from_flags(buttons, Switch2ControllerButtons::Right),
            NsInput::Left => Self::value_from_flags(buttons, Switch2ControllerButtons::Left),
            NsInput::Up => Self::value_from_flags(buttons, Switch2ControllerButtons::Up),

            NsInput::L => Self::value_from_flags(buttons, Switch2ControllerButtons::L),
            NsInput::Zl => Self::value_from_flags(buttons, Switch2ControllerButtons::Zl),

            NsInput::B => Self::value_from_flags(buttons, Switch2ControllerButtons::B),
            NsInput::A => Self::value_from_flags(buttons, Switch2ControllerButtons::A),
            NsInput::Y => Self::value_from_flags(buttons, Switch2ControllerButtons::Y),
            NsInput::X => Self::value_from_flags(buttons, Switch2ControllerButtons::X),

            NsInput::R => Self::value_from_flags(buttons, Switch2ControllerButtons::R),
            NsInput::Zr => Self::value_from_flags(buttons, Switch2ControllerButtons::Zr),

            NsInput::Plus => Self::value_from_flags(buttons, Switch2ControllerButtons::Plus),
        }
    }

    // ------------ Special data decoding ----------------

    // fn decode_mouse_coords(&self, buffer: &[u8]) -> MouseCoords {
    //     if buffer.len() < 0x18 {
    //         return MouseCoords { x: 960, y: 471 };
    //     }
    //
    //     let raw_x = i16::from_le_bytes([buffer[0x10], buffer[0x11]]);
    //     let raw_y = i16::from_le_bytes([buffer[0x12], buffer[0x13]]);
    //
    //     let norm_x = (raw_x as f32 / 32767_f32).clamp(-1_f32, 1_f32);
    //     let norm_y = (raw_y as f32 / 32767_f32).clamp(-1_f32, 1_f32);
    //
    //     let x = ((norm_x + 1_f32) * 0.5 * 1920_f32) as u16;
    //     let y = ((1_f32 - (norm_y + 1_f32) * 0.5_f32) * 943_f32) as u16;
    //
    //     MouseCoords { x, y }
    // }

    fn decode_motion_axis(&self, axis_buffer: &[u8]) -> (f32, f32) {
        let axis_buffer = axis_buffer.try_into().unwrap();
        let axis = i16::from_le_bytes(axis_buffer);
        let positive = axis.clamp(0, i16::MAX) as f32 / 16.384;
        let negative = -axis.clamp(i16::MIN, 0) as f32 / 16.384;

        (positive, negative)
    }

    fn decode_motion(&self, common_buffer: &[u8]) -> HashMap<NsInput, f32> {
        const MOTION_BYTES: Range<usize> = 0x30..0x30 + 0xC;
        const ACCEL_X_BYTES: Range<usize> = 0x0..0x2;
        const ACCEL_Y_BYTES: Range<usize> = 0x2..0x2 + 0x2;
        const ACCEL_Z_BYTES: Range<usize> = 0x4..0x4 + 0x2;
        const GYRO_X_BYTES: Range<usize> = 0x6..0x6 + 0x2;
        const GYRO_Y_BYTES: Range<usize> = 0x8..0x8 + 0x2;
        const GYRO_Z_BYTES: Range<usize> = 0xA..0xA + 0x2;

        let motion_buffer = &common_buffer[MOTION_BYTES];

        let (accel_right, accel_left) = self.decode_motion_axis(&motion_buffer[ACCEL_X_BYTES]);
        let (accel_up, accel_down) = self.decode_motion_axis(&motion_buffer[ACCEL_Y_BYTES]);
        let (accel_forward, accel_backward) =
            self.decode_motion_axis(&motion_buffer[ACCEL_Z_BYTES]);

        let (pitch_up, pitch_down) = self.decode_motion_axis(&motion_buffer[GYRO_X_BYTES]);
        let (roll_right, roll_left) = self.decode_motion_axis(&motion_buffer[GYRO_Z_BYTES]);
        let (yaw_right, yaw_left) = self.decode_motion_axis(&motion_buffer[GYRO_Y_BYTES]);

        hashmap! {
            NsInput::AccelUp => accel_up,
            NsInput::AccelDown => accel_down,
            NsInput::AccelRight => accel_right,
            NsInput::AccelLeft => accel_left,
            NsInput::AccelForward => accel_forward,
            NsInput::AccelBackward => accel_backward,

            NsInput::GyroPitchUp => pitch_up,
            NsInput::GyroPitchDown => pitch_down,
            NsInput::GyroRollRight => roll_right,
            NsInput::GyroRollLeft =>  roll_left,
            NsInput::GyroYawRight => yaw_right,
            NsInput::GyroYawLeft => yaw_left,
        }
    }

    fn decode_calibrated_trigger(&self, raw: u8) -> f32 {
        const LOWER_BOUND: u8 = 0x28;
        const UPPER_BOUND: u8 = 0xDD;

        if raw <= LOWER_BOUND {
            0_f32
        } else if raw >= UPPER_BOUND {
            1_f32
        } else {
            let adjusted = raw - LOWER_BOUND;
            let range = UPPER_BOUND - LOWER_BOUND;

            adjusted as f32 / range as f32
        }
    }

    fn decode_nso_gc_controller_triggers(&self, buffer: &[u8]) -> HashMap<NsInput, f32> {
        const LEFT_ANALOG_TRIGGER_BYTE: usize = 0x3C;
        const RIGHT_ANALOG_TRIGGER_BYTE: usize = 0x3D;

        let l_trigger = self.decode_calibrated_trigger(buffer[LEFT_ANALOG_TRIGGER_BYTE]);
        let r_trigger = self.decode_calibrated_trigger(buffer[RIGHT_ANALOG_TRIGGER_BYTE]);

        hashmap! {
            NsInput::LTrigger => l_trigger,
            NsInput::RTrigger => r_trigger,
        }
    }

    // ------------ Controller decodings ----------------

    pub fn decode_left_joy_con(&self, buffer: &[u8]) -> InputData {
        let stick_inputs = self.decode_left_stick(buffer);
        let button_inputs = self.decode_left_joy_con_buttons(buffer);
        let motion_inputs = self.decode_motion(buffer);

        let mut inputs = stick_inputs;
        inputs.extend(button_inputs);
        inputs.extend(motion_inputs);

        InputData::new(inputs)
    }

    pub fn decode_right_joy_con(&self, buffer: &[u8]) -> InputData {
        let stick_inputs = self.decode_right_stick(buffer);
        let button_inputs = self.decode_right_joy_con_buttons(buffer);
        let motion_inputs = self.decode_motion(buffer);

        let mut inputs = stick_inputs;
        inputs.extend(button_inputs);
        inputs.extend(motion_inputs);

        InputData::new(inputs)
    }

    pub fn decode_dual_joy_cons(
        &self,
        left_buffer: &[u8],
        right_buffer: &[u8],
        motion_source: MotionSource,
    ) -> InputData {
        let left_stick_inputs = self.decode_left_stick(left_buffer);
        let left_button_inputs = self.decode_left_joy_con_buttons(left_buffer);

        let right_stick_inputs = self.decode_right_stick(right_buffer);
        let right_button_inputs = self.decode_left_joy_con_buttons(right_buffer);

        let motion_data = match motion_source {
            MotionSource::Left => self.decode_motion(left_buffer),
            MotionSource::Right => self.decode_motion(right_buffer),
        };

        let mut inputs = left_stick_inputs;
        inputs.extend(left_button_inputs);
        inputs.extend(right_stick_inputs);
        inputs.extend(right_button_inputs);
        inputs.extend(motion_data);

        InputData::new(inputs)
    }

    pub fn decode_pro_controller(&self, buffer: &[u8]) -> InputData {
        let left_stick_inputs = self.decode_left_stick(buffer);
        let right_stick_inputs = self.decode_right_stick(buffer);
        let button_inputs = self.decode_pro_controller_buttons(buffer);
        let motion_inputs = self.decode_motion(buffer);

        let mut inputs = left_stick_inputs;
        inputs.extend(right_stick_inputs);
        inputs.extend(button_inputs);
        inputs.extend(motion_inputs);

        InputData::new(inputs)
    }

    pub fn decode_nso_gc_controller(&self, buffer: &[u8]) -> InputData {
        let left_stick_inputs = self.decode_left_stick(buffer);
        let right_stick_inputs = self.decode_right_stick(buffer);
        let button_inputs = self.decode_nso_gc_controller_buttons(buffer);
        let trigger_inputs = self.decode_nso_gc_controller_triggers(buffer);
        let motion_inputs = self.decode_motion(buffer);

        let mut inputs = left_stick_inputs;
        inputs.extend(right_stick_inputs);
        inputs.extend(button_inputs);
        inputs.extend(trigger_inputs);
        inputs.extend(motion_inputs);

        InputData::new(inputs)
    }
}
