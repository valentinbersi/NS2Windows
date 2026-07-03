use crate::data::input_data::InputData;
use crate::data::ns_input::NsInput;
use crate::dtos::motion_source::MotionSource;
use bitflags::{Flags, bitflags};
use maplit::hashmap;
use std::collections::HashMap;
use std::ops::{BitAnd, Range};

bitflags! {
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    struct LeftJoyConButtons: u16 {
        const Capture = 0x00_01;
        const Sr = 0x00_40;
        const Sl = 0x00_80;
        const Down = 0x01_00;
        const Right = 0x02_00;
        const Left = 0x04_00;
        const Up = 0x08_00;
        const L = 0x10_00;
        const Zl = 0x20_00;
        const Minus = 0x40_00;
        const Stick = 0x80_00;
    }

    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    struct RightJoyConButtons: u16 {
        const Home = 0x00_01;
        const C = 0x00_10;
        const Sr = 0x00_40;
        const Sl = 0x00_80;
        const B = 0x01_00;
        const A = 0x02_00;
        const Y = 0x04_00;
        const X = 0x08_00;
        const R = 0x10_00;
        const Zr = 0x20_00;
        const Plus = 0x40_00;
        const Stick = 0x80_00;
    }

    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    struct ProControllerButtons: u32 {
        const Home = 0x00_00_01;
        const Capture = 0x00_00_02;
        const Gr = 0x00_00_04;
        const Gl = 0x00_00_08;
        const C = 0x00_00_10;
        const Down = 0x00_01_00;
        const Right = 0x00_02_00;
        const Left = 0x00_04_00;
        const Up = 0x00_08_00;
        const L = 0x00_10_00;
        const Zl = 0x00_20_00;
        const Minus = 0x00_40_00;
        const LeftStick = 0x00_80_00;
        const B = 0x01_00_00;
        const A = 0x02_00_00;
        const Y = 0x04_00_00;
        const X = 0x08_00_00;
        const R = 0x10_00_00;
        const Zr = 0x20_00_00;
        const Plus = 0x40_00_00;
        const RightStick = 0x80_00_00;
    }

    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    struct NsoGcControllerButtons: u32 {
        const Home = 0x00_00_01;
        const Capture = 0x00_00_02;
        const C = 0x00_00_10;
        const Down = 0x00_01_00;
        const Right = 0x00_02_00;
        const Left = 0x00_04_00;
        const Up = 0x00_08_00;
        const L = 0x00_10_00;
        const Zl = 0x00_20_00;
        const Minus = 0x00_40_00;
        const LeftStick = 0x00_80_00;
        const B = 0x01_00_00;
        const A = 0x02_00_00;
        const Y = 0x04_00_00;
        const X = 0x08_00_00;
        const R = 0x10_00_00;
        const Zr = 0x20_00_00;
        const Plus = 0x40_00_00;
        const RightStick = 0x80_00_00;
    }
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

    fn decode_joystick(&self, buffer: &[u8]) -> StickData {
        if buffer.len() < 3 {
            return StickData { x: 0_f32, y: 0_f32 };
        }

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

    fn decode_joy_con_stick(&self, unique_buffer: &[u8]) -> HashMap<NsInput, f32> {
        const JOY_CON_STICK_BYTES: Range<usize> = 0x5..0x5 + 0x3;

        let analog_stick = &unique_buffer[JOY_CON_STICK_BYTES];
        let stick = self.decode_joystick(analog_stick);
        hashmap! {
            NsInput::LeftXMinus => -stick.x.clamp(-1_f32, 0_f32),
            NsInput::LeftXPlus => stick.x.clamp(0_f32, 1_f32),
            NsInput::LeftYMinus => -stick.y.clamp(-1_f32, 0_f32),
            NsInput::LeftYPlus => stick.y.clamp(0_f32, 1_f32),
        }
    }

    fn decode_controller_stick(&self, unique_buffer: &[u8]) -> HashMap<NsInput, f32> {
        const CONTROLLER_LEFT_STICK_BYTES: Range<usize> = 0x5..0x5 + 0x3;
        const CONTROLLER_RIGHT_STICK_BYTES: Range<usize> = 0x8..0x8 + 0x3;

        let left_analog_stick = &unique_buffer[CONTROLLER_LEFT_STICK_BYTES];
        let left_stick = self.decode_joystick(left_analog_stick);

        let right_analog_stick = &unique_buffer[CONTROLLER_RIGHT_STICK_BYTES];
        let right_stick = self.decode_joystick(right_analog_stick);

        hashmap! {
            NsInput::LeftXMinus => -left_stick.x.clamp(-1_f32, 0_f32),
            NsInput::LeftXPlus => left_stick.x.clamp(0_f32, 1_f32),
            NsInput::LeftYMinus => -left_stick.y.clamp(-1_f32, 0_f32),
            NsInput::LeftYPlus => left_stick.y.clamp(0_f32, 1_f32),

            NsInput::RightXMinus => -right_stick.x.clamp(-1_f32, 0_f32),
            NsInput::RightXPlus => right_stick.x.clamp(0_f32, 1_f32),
            NsInput::RightYMinus => -right_stick.y.clamp(-1_f32, 0_f32),
            NsInput::RightYPlus => right_stick.y.clamp(0_f32, 1_f32),
        }
    }

    // ------------ buttons decoding ------------

    fn value_from_flags<F: Flags + BitAnd<Output = F> + PartialEq>(buttons: F, flag: F) -> f32 {
        if buttons & flag != F::empty() {
            1_f32
        } else {
            0_f32
        }
    }

    const JOY_CON_BUTTONS_BYTES: Range<usize> = 0x2..0x2 + 0x2;

    fn decode_left_joy_con_buttons(&self, unique_buffer: &[u8]) -> HashMap<NsInput, f32> {
        let buttons_buffer = &unique_buffer[Self::JOY_CON_BUTTONS_BYTES];
        let buttons_buffer = buttons_buffer.try_into().unwrap();

        let buttons_bit_field = u16::from_le_bytes(buttons_buffer);
        let buttons = LeftJoyConButtons::from_bits(buttons_bit_field).unwrap();

        hashmap! {
            NsInput::Capture => Self::value_from_flags(buttons, LeftJoyConButtons::Capture),

            NsInput::Sr => Self::value_from_flags(buttons, LeftJoyConButtons::Sr),
            NsInput::Sl => Self::value_from_flags(buttons, LeftJoyConButtons::Sl),

            NsInput::Down => Self::value_from_flags(buttons, LeftJoyConButtons::Down),
            NsInput::Right => Self::value_from_flags(buttons, LeftJoyConButtons::Right),
            NsInput::Left => Self::value_from_flags(buttons, LeftJoyConButtons::Left),
            NsInput::Up => Self::value_from_flags(buttons, LeftJoyConButtons::Up),

            NsInput::L => Self::value_from_flags(buttons, LeftJoyConButtons::L),
            NsInput::Zl => Self::value_from_flags(buttons, LeftJoyConButtons::Zl),

            NsInput::Minus => Self::value_from_flags(buttons, LeftJoyConButtons::Minus),

            NsInput::Tl => Self::value_from_flags(buttons, LeftJoyConButtons::Stick),
        }
    }

    fn decode_right_joy_con_buttons(&self, unique_buffer: &[u8]) -> HashMap<NsInput, f32> {
        let buttons_buffer = &unique_buffer[Self::JOY_CON_BUTTONS_BYTES];
        let buttons_buffer = buttons_buffer.try_into().unwrap();

        let buttons_bit_field = u16::from_le_bytes(buttons_buffer);
        let buttons = RightJoyConButtons::from_bits(buttons_bit_field).unwrap();

        hashmap! {
            NsInput::Home => Self::value_from_flags(buttons, RightJoyConButtons::Home),
            NsInput::Chat => Self::value_from_flags(buttons, RightJoyConButtons::C),

            NsInput::Sr => Self::value_from_flags(buttons, RightJoyConButtons::Sr),
            NsInput::Sl => Self::value_from_flags(buttons, RightJoyConButtons::Sl),

            NsInput::B => Self::value_from_flags(buttons, RightJoyConButtons::B),
            NsInput::A => Self::value_from_flags(buttons, RightJoyConButtons::A),
            NsInput::Y => Self::value_from_flags(buttons, RightJoyConButtons::Y),
            NsInput::X => Self::value_from_flags(buttons, RightJoyConButtons::X),

            NsInput::R => Self::value_from_flags(buttons, RightJoyConButtons::R),
            NsInput::Zr => Self::value_from_flags(buttons, RightJoyConButtons::Zr),

            NsInput::Plus => Self::value_from_flags(buttons, RightJoyConButtons::Plus),

            NsInput::Tl => Self::value_from_flags(buttons, RightJoyConButtons::Stick),
        }
    }

    const CONTROLLER_BUTTONS_BYTES: Range<usize> = 0x2..0x2 + 0x3;

    fn decode_pro_controller_buttons(&self, unique_buffer: &[u8]) -> HashMap<NsInput, f32> {
        let buttons_buffer = &unique_buffer[Self::CONTROLLER_BUTTONS_BYTES];
        let buttons_buffer = [0, buttons_buffer[0], buttons_buffer[1], buttons_buffer[2]];

        let buttons_bit_field = u32::from_le_bytes(buttons_buffer);
        let buttons = ProControllerButtons::from_bits(buttons_bit_field).unwrap();

        hashmap! {
            NsInput::Home => Self::value_from_flags(buttons, ProControllerButtons::Home),
            NsInput::Capture => Self::value_from_flags(buttons, ProControllerButtons::Capture),

            NsInput::Gr => Self::value_from_flags(buttons, ProControllerButtons::Gr),
            NsInput::Gl => Self::value_from_flags(buttons, ProControllerButtons::Gl),

            NsInput::Chat => Self::value_from_flags(buttons, ProControllerButtons::C),

            NsInput::Down => Self::value_from_flags(buttons, ProControllerButtons::Down),
            NsInput::Right => Self::value_from_flags(buttons, ProControllerButtons::Right),
            NsInput::Left => Self::value_from_flags(buttons, ProControllerButtons::Left),
            NsInput::Up => Self::value_from_flags(buttons, ProControllerButtons::Up),

            NsInput::L => Self::value_from_flags(buttons, ProControllerButtons::L),
            NsInput::Zl => Self::value_from_flags(buttons, ProControllerButtons::Zl),

            NsInput::Minus => Self::value_from_flags(buttons, ProControllerButtons::Minus),

            NsInput::Tl => Self::value_from_flags(buttons, ProControllerButtons::LeftStick),

            NsInput::B => Self::value_from_flags(buttons, ProControllerButtons::B),
            NsInput::A => Self::value_from_flags(buttons, ProControllerButtons::A),
            NsInput::Y => Self::value_from_flags(buttons, ProControllerButtons::Y),
            NsInput::X => Self::value_from_flags(buttons, ProControllerButtons::X),

            NsInput::R => Self::value_from_flags(buttons, ProControllerButtons::R),
            NsInput::Zr => Self::value_from_flags(buttons, ProControllerButtons::Zr),

            NsInput::Plus => Self::value_from_flags(buttons, ProControllerButtons::Plus),

            NsInput::Tr => Self::value_from_flags(buttons, ProControllerButtons::RightStick),
        }
    }

    fn decode_nso_gc_controller_buttons(&self, unique_buffer: &[u8]) -> HashMap<NsInput, f32> {
        let buttons_buffer = &unique_buffer[Self::CONTROLLER_BUTTONS_BYTES];
        let buttons_buffer = [0, buttons_buffer[0], buttons_buffer[1], buttons_buffer[2]];

        let buttons_bit_field = u32::from_le_bytes(buttons_buffer);
        let buttons = NsoGcControllerButtons::from_bits(buttons_bit_field).unwrap();

        hashmap! {
            NsInput::Home => Self::value_from_flags(buttons, NsoGcControllerButtons::Home),
            NsInput::Capture => Self::value_from_flags(buttons, NsoGcControllerButtons::Capture),

            NsInput::Chat => Self::value_from_flags(buttons, NsoGcControllerButtons::C),

            NsInput::Down => Self::value_from_flags(buttons, NsoGcControllerButtons::Down),
            NsInput::Right => Self::value_from_flags(buttons, NsoGcControllerButtons::Right),
            NsInput::Left => Self::value_from_flags(buttons, NsoGcControllerButtons::Left),
            NsInput::Up => Self::value_from_flags(buttons, NsoGcControllerButtons::Up),

            NsInput::L => Self::value_from_flags(buttons, NsoGcControllerButtons::L),
            NsInput::Zl => Self::value_from_flags(buttons, NsoGcControllerButtons::Zl),

            NsInput::Minus => Self::value_from_flags(buttons, NsoGcControllerButtons::Minus),

            NsInput::Tl => Self::value_from_flags(buttons, NsoGcControllerButtons::LeftStick),

            NsInput::B => Self::value_from_flags(buttons, NsoGcControllerButtons::B),
            NsInput::A => Self::value_from_flags(buttons, NsoGcControllerButtons::A),
            NsInput::Y => Self::value_from_flags(buttons, NsoGcControllerButtons::Y),
            NsInput::X => Self::value_from_flags(buttons, NsoGcControllerButtons::X),

            NsInput::R => Self::value_from_flags(buttons, NsoGcControllerButtons::R),
            NsInput::Zr => Self::value_from_flags(buttons, NsoGcControllerButtons::Zr),

            NsInput::Plus => Self::value_from_flags(buttons, NsoGcControllerButtons::Plus),

            NsInput::Tr => Self::value_from_flags(buttons, NsoGcControllerButtons::RightStick),
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
        const ACCEL_X_BYTES: Range<usize> = 0x0..0x0 + 0x2;
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

    fn decode_nso_gc_controller_triggers(&self, unique_buffer: &[u8]) -> HashMap<NsInput, f32> {
        const LEFT_ANALOG_TRIGGER_BYTE: usize = 0xC;
        const RIGHT_ANALOG_TRIGGER_BYTE: usize = 0xD;

        let l_trigger = self.decode_calibrated_trigger(unique_buffer[LEFT_ANALOG_TRIGGER_BYTE]);
        let r_trigger = self.decode_calibrated_trigger(unique_buffer[RIGHT_ANALOG_TRIGGER_BYTE]);

        hashmap! {
            NsInput::LTrigger => l_trigger,
            NsInput::RTrigger => r_trigger,
        }
    }

    // ------------ Controller decodings ----------------

    pub fn decode_left_joy_con(&self, unique_buffer: &[u8], common_buffer: &[u8]) -> InputData {
        let stick_inputs = self.decode_joy_con_stick(unique_buffer);
        let button_inputs = self.decode_left_joy_con_buttons(unique_buffer);
        let motion_inputs = self.decode_motion(common_buffer);

        let mut inputs = stick_inputs;
        inputs.extend(button_inputs);
        inputs.extend(motion_inputs);

        InputData::new(inputs)
    }

    pub fn decode_right_joy_con(&self, unique_buffer: &[u8], common_buffer: &[u8]) -> InputData {
        let stick_inputs = self.decode_joy_con_stick(unique_buffer);
        let button_inputs = self.decode_right_joy_con_buttons(unique_buffer);
        let motion_inputs = self.decode_motion(common_buffer);

        let mut inputs = stick_inputs;
        inputs.extend(button_inputs);
        inputs.extend(motion_inputs);

        InputData::new(inputs)
    }

    pub fn decode_dual_joy_cons(
        &self,
        left_unique_buffer: &[u8],
        left_common_buffer: &[u8],
        right_unique_buffer: &[u8],
        right_common_buffer: &[u8],
        motion_source: MotionSource,
    ) -> InputData {
        let left_stick_inputs = self.decode_joy_con_stick(left_unique_buffer);
        let left_button_inputs = self.decode_left_joy_con_buttons(left_unique_buffer);

        let right_stick_inputs = self.decode_joy_con_stick(right_unique_buffer);
        let right_button_inputs = self.decode_left_joy_con_buttons(right_unique_buffer);

        let motion_data = match motion_source {
            MotionSource::Left => self.decode_motion(left_common_buffer),
            MotionSource::Right => self.decode_motion(right_common_buffer),
        };

        let mut inputs = left_stick_inputs;
        inputs.extend(left_button_inputs);
        inputs.extend(right_stick_inputs);
        inputs.extend(right_button_inputs);
        inputs.extend(motion_data);

        InputData::new(inputs)
    }

    pub fn decode_pro_controller(&self, unique_buffer: &[u8], common_buffer: &[u8]) -> InputData {
        let stick_inputs = self.decode_controller_stick(unique_buffer);
        let button_inputs = self.decode_pro_controller_buttons(unique_buffer);
        let motion_inputs = self.decode_motion(common_buffer);

        let mut inputs = stick_inputs;
        inputs.extend(button_inputs);
        inputs.extend(motion_inputs);

        InputData::new(inputs)
    }

    pub fn decode_nso_gc_controller(
        &self,
        unique_buffer: &[u8],
        common_buffer: &[u8],
    ) -> InputData {
        let stick_inputs = self.decode_controller_stick(unique_buffer);
        let button_inputs = self.decode_nso_gc_controller_buttons(unique_buffer);
        let trigger_inputs = self.decode_nso_gc_controller_triggers(unique_buffer);
        let motion_inputs = self.decode_motion(common_buffer);

        let mut inputs = stick_inputs;
        inputs.extend(button_inputs);
        inputs.extend(trigger_inputs);
        inputs.extend(motion_inputs);

        InputData::new(inputs)
    }
}
