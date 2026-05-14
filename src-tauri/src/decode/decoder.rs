use crate::data::input_data::InputData;
use crate::data::ns_input::NsInput;
use crate::data::ns_input::NsInput::{
    AccelBackward, AccelDown, AccelForward, AccelLeft, AccelRight, AccelUp, Capture, Down, Gl, Gr,
    GyroPitchDown, GyroPitchUp, GyroRollLeft, GyroRollRight, GyroYawLeft, GyroYawRight, Home, Left,
    LeftXMinus, LeftXPlus, LeftYMinus, LeftYPlus, Minus, Plus, Right, RightXMinus, RightXPlus,
    RightYMinus, RightYPlus, Sl, Sr, Tl, Tr, Up, Zl, Zr, A, B, L, R, X, Y,
};
use bitflags::bitflags;
use maplit::hashmap;
use std::collections::HashMap;
use std::ops::RangeInclusive;

const LEFT_STICK_RANGE: RangeInclusive<usize> = 10..=12;
const RIGHT_STICK_RANGE: RangeInclusive<usize> = 13..=16;

bitflags! {
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    struct RightJoyConButtonMasks: u32 {
        const Plus = 0x0002;
        const Tr = 0x0004;
        const Y = 0x0100;
        const B = 0x0200;
        const X = 0x0400;
        const A = 0x0800;
        const Sr = 0x1000;
        const Sl = 0x2000;
        const R = 0x4000;
        const Zr = 0x8000;
    }

    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    struct LeftJoyConButtonMasks: u32 {
        const Down = 0x0001;
        const Up = 0x0002;
        const Right = 0x0004;
        const Left = 0x0008;
        const Sr = 0x0010;
        const Sl = 0x0020;
        const L = 0x0040;
        const Zl = 0x0080;
        const Minus = 0x0100;
        const Tl = 0x0800;
    }

    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    struct ProControllerButtonMasks: u64 {
        const Down = 0x000000010000;
        const Up = 0x000000020000;
        const Right = 0x000000040000;
        const Left = 0x000000080000;
        const Zl = 0x000000800000;
        const Minus = 0x000001000000;
        const Plus = 0x000002000000;
        const Tr = 0x000004000000;
        const Tl = 0x000008000000;
        // const Guide = 0x000010000000;
        const L = 0x000000400000;
        const Y = 0x000100000000;
        const X = 0x000200000000;
        const B = 0x000400000000;
        const A = 0x000800000000;
        const R = 0x004000000000;
        const Zr = 0x008000000000;
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

        let b0 = buffer[0] as i32;
        let b1 = buffer[1] as i32;
        let b2 = buffer[2] as i32;

        let x_raw = ((b1 & 0x0F) << 8) | b0;
        let y_raw = (b2 << 4) | ((b1 & 0xF0) >> 4);

        let mut x = (x_raw - 2048) as f32 / 2048_f32;
        let mut y = (y_raw - 2048) as f32 / 2048_f32;

        let dead_zone = 0.08;
        if x.abs() < dead_zone && y.abs() < dead_zone {
            return StickData { x: 0_f32, y: 0_f32 };
        }

        x = (x * 1.7).clamp(-1_f32, 1_f32);
        y = (y * 1.7).clamp(-1_f32, 1_f32);

        StickData { x, y }
    }

    fn decode_left_joystick(&self, buffer: &[u8]) -> HashMap<NsInput, f32> {
        let stick = self.decode_joystick(&buffer[LEFT_STICK_RANGE]);

        hashmap! {
            LeftXMinus => stick.x.clamp(-1_f32, 0_f32),
            LeftXPlus => stick.x.clamp(0_f32, 1_f32),
            LeftYMinus => stick.y.clamp(-1_f32, 0_f32),
            LeftYPlus => stick.y.clamp(0_f32, 1_f32),
        }
    }

    fn decode_right_joystick(&self, buffer: &[u8]) -> HashMap<NsInput, f32> {
        let stick = self.decode_joystick(&buffer[RIGHT_STICK_RANGE]);

        hashmap! {
            RightXMinus => stick.x.clamp(-1_f32, 0_f32),
            RightXPlus => stick.x.clamp(0_f32, 1_f32),
            RightYMinus => stick.y.clamp(-1_f32, 0_f32),
            RightYPlus => stick.y.clamp(0_f32, 1_f32),
        }
    }

    fn decode_dual_joysticks(&self, buffer: &[u8]) -> HashMap<NsInput, f32> {
        let mut stick_data = self.decode_left_joystick(buffer);
        stick_data.extend(self.decode_right_joystick(buffer));
        stick_data
    }

    // ------------ buttons decoding ------------

    fn decode_left_buttons(&self, buffer: &[u8]) -> HashMap<NsInput, f32> {
        let button_offset = 4;
        let b0 = buffer[button_offset];
        let b1 = buffer[button_offset + 1];
        let b2 = buffer[button_offset + 2];

        let state = u32::from_be_bytes([0, b0, b1, b2]);

        let from_flag = |flag: LeftJoyConButtonMasks| {
            if state & flag.bits() != 0 {
                1_f32
            } else {
                0_f32
            }
        };

        hashmap! {
            Capture => 0_f32,

            Sr => from_flag(LeftJoyConButtonMasks::Sr),

            L => from_flag(LeftJoyConButtonMasks::L),
            Tl => from_flag(LeftJoyConButtonMasks::Tl),
            Zl => from_flag(LeftJoyConButtonMasks::Zl),
            Sl => from_flag(LeftJoyConButtonMasks::Sl),

            Minus => from_flag(LeftJoyConButtonMasks::Minus),

            Down => from_flag(LeftJoyConButtonMasks::Down),
            Left => from_flag(LeftJoyConButtonMasks::Left),
            Right => from_flag(LeftJoyConButtonMasks::Right),
            Up => from_flag(LeftJoyConButtonMasks::Up),
        }
    }

    fn decode_right_buttons(&self, buffer: &[u8]) -> HashMap<NsInput, f32> {
        let button_offset = 3;
        let b0 = buffer[button_offset];
        let b1 = buffer[button_offset + 1];
        let b2 = buffer[button_offset + 2];

        let state = u32::from_le_bytes([0, b0, b1, b2]);

        let from_flag = |flag: RightJoyConButtonMasks| {
            if state & flag.bits() != 0 {
                1_f32
            } else {
                0_f32
            }
        };

        hashmap! {
            B => from_flag(RightJoyConButtonMasks::B),
            A => from_flag(RightJoyConButtonMasks::A),
            Y => from_flag(RightJoyConButtonMasks::Y),
            X => from_flag(RightJoyConButtonMasks::X),

            Home => 0_f32,

            R => from_flag(RightJoyConButtonMasks::R),
            Tr => from_flag(RightJoyConButtonMasks::Tr),
            Zr => from_flag(RightJoyConButtonMasks::Zr),
            Sr => from_flag(RightJoyConButtonMasks::Sr),

            Sl => from_flag(RightJoyConButtonMasks::Sl),

            Plus => from_flag(RightJoyConButtonMasks::Plus),
        }
    }

    fn decode_pro_buttons(&self, buffer: &[u8]) -> HashMap<NsInput, f32> {
        let mut state = 0;
        for i in 3..=8 {
            state = (state << 8) | (buffer[i] as u64)
        }

        let from_flag = |flag: ProControllerButtonMasks| {
            if state & flag.bits() != 0 {
                1_f32
            } else {
                0_f32
            }
        };

        hashmap! {
            B => from_flag(ProControllerButtonMasks::B),
            A => from_flag(ProControllerButtonMasks::A),
            Y => from_flag(ProControllerButtonMasks::Y),
            X => from_flag(ProControllerButtonMasks::X),

            Home => 0_f32,
            Capture => 0_f32,

            R => from_flag(ProControllerButtonMasks::R),
            Tr => from_flag(ProControllerButtonMasks::Tr),
            Zr => from_flag(ProControllerButtonMasks::Zr),
            Gr => 0_f32,

            L => from_flag(ProControllerButtonMasks::L),
            Tl => from_flag(ProControllerButtonMasks::Tl),
            Zl => from_flag(ProControllerButtonMasks::Zl),
            Gl => 0_f32,

            Plus => from_flag(ProControllerButtonMasks::Plus),
            Minus => from_flag(ProControllerButtonMasks::Minus),

            Down => from_flag(ProControllerButtonMasks::Down),
            Left => from_flag(ProControllerButtonMasks::Left),
            Right => from_flag(ProControllerButtonMasks::Right),
            Up => from_flag(ProControllerButtonMasks::Up),
        }
    }

    fn decode_gc_buttons(&self, buffer: &[u8]) -> HashMap<NsInput, f32> {
        let mut state = 0;
        for i in 3..=8 {
            state = (state << 8) | (buffer[i] as u64)
        }

        let from_flag = |flag: ProControllerButtonMasks| {
            if state & flag.bits() != 0 {
                1_f32
            } else {
                0_f32
            }
        };

        hashmap! {
            B => from_flag(ProControllerButtonMasks::B),
            A => from_flag(ProControllerButtonMasks::A),
            Y => from_flag(ProControllerButtonMasks::Y),
            X => from_flag(ProControllerButtonMasks::X),

            Home => 0_f32,
            Capture => 0_f32,

            R => from_flag(ProControllerButtonMasks::R),
            Tr => from_flag(ProControllerButtonMasks::Tr),
            Gr => 0_f32,

            L => from_flag(ProControllerButtonMasks::L),
            Tl => from_flag(ProControllerButtonMasks::Tl),
            Gl => 0_f32,

            Plus => from_flag(ProControllerButtonMasks::Plus),
            Minus => from_flag(ProControllerButtonMasks::Minus),

            Down => from_flag(ProControllerButtonMasks::Down),
            Left => from_flag(ProControllerButtonMasks::Left),
            Right => from_flag(ProControllerButtonMasks::Right),
            Up => from_flag(ProControllerButtonMasks::Up),
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

    fn decode_motion(&self, buffer: &[u8]) -> HashMap<NsInput, f32> {
        let accel_x = i16::from_le_bytes([buffer[0x30], buffer[0x31]]);
        let accel_y = i16::from_le_bytes([buffer[0x32], buffer[0x33]]);
        let accel_z = i16::from_le_bytes([buffer[0x34], buffer[0x35]]);

        let gyro_x = i16::from_le_bytes([buffer[0x36], buffer[0x37]]);
        let gyro_y = i16::from_le_bytes([buffer[0x38], buffer[0x39]]);
        let gyro_z = i16::from_le_bytes([buffer[0x3A], buffer[0x3B]]);

        hashmap! {
            AccelUp => accel_y.clamp(0, i16::MAX) as f32 / 16.384,
            AccelDown => accel_y.clamp(i16::MIN, 0) as f32 / 16.384,
            AccelRight => accel_x.clamp(0, i16::MAX) as f32 / 16.384,
            AccelLeft => accel_x.clamp(i16::MIN, 0) as f32 / 16.384,
            AccelForward => accel_z.clamp(0, i16::MAX) as f32 / 16.384,
            AccelBackward => accel_z.clamp(i16::MIN, 0) as f32 / 16.384,

            GyroPitchUp => gyro_x.clamp(0, i16::MAX) as f32 / 16.384,
            GyroPitchDown => gyro_x.clamp(i16::MIN, 0) as f32 / 16.384,
            GyroRollRight => gyro_y.clamp(0, i16::MAX) as f32 / 16.384,
            GyroRollLeft => gyro_y.clamp(i16::MIN, 0) as f32 / 16.384,
            GyroYawRight => gyro_z.clamp(0, i16::MAX) as f32 / 16.384,
            GyroYawLeft => gyro_z.clamp(i16::MIN, 0) as f32 / 16.384,
        }
    }

    fn decode_gc_triggers(&self, buffer: &[u8]) -> HashMap<NsInput, f32> {
        hashmap! {
            Zl => buffer[0x3c] as f32 / 255_f32,
            Zr => buffer[0x3d] as f32 / 255_f32,
        }
    }

    // ------------ Controller decodings ----------------

    pub fn decode_left_joycon(&self, buffer: &[u8]) -> InputData {
        let mut inputs = self.decode_left_joystick(buffer);
        inputs.extend(self.decode_left_buttons(buffer));
        // inputs.extend(self.decode_mouse_coords(buffer));
        inputs.extend(self.decode_motion(buffer));

        InputData::new(inputs)
    }

    pub fn decode_right_joycon(&self, buffer: &[u8]) -> InputData {
        let mut inputs = self.decode_right_joystick(buffer);
        inputs.extend(self.decode_right_buttons(buffer));
        // inputs.extend(self.decode_mouse_coords(buffer));
        inputs.extend(self.decode_motion(buffer));

        InputData::new(inputs)
    }

    pub fn decode_pro_controller(&self, buffer: &[u8]) -> InputData {
        let mut inputs = self.decode_dual_joysticks(buffer);
        inputs.extend(self.decode_pro_buttons(buffer));
        // inputs.extend(self.decode_mouse_coords(buffer));
        inputs.extend(self.decode_motion(buffer));

        InputData::new(inputs)
    }

    pub fn decode_gc_controller(&self, buffer: &[u8]) -> InputData {
        let mut inputs = self.decode_dual_joysticks(buffer);
        inputs.extend(self.decode_pro_buttons(buffer));
        // inputs.extend(self.decode_mouse_coords(buffer));
        inputs.extend(self.decode_motion(buffer));
        inputs.extend(self.decode_gc_triggers(buffer));

        InputData::new(inputs)
    }
}
