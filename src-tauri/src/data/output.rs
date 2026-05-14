use crate::data::profile_kind::ProfileKind;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Output {
    // ----------- Main Buttons -----------
    CrossA,
    CircleB,
    SquareX,
    TriangleY,

    // ----------- Special buttons -----------
    PsGuide,
    Share,

    // ----------- Right buttons -----------
    R1Rb,
    R2Rt,
    R3Rs,

    // ----------- Left buttons -----------
    L1Lb,
    L2Lt,
    L3Ls,

    // ----------- Menu buttons -----------
    OptionsStart,
    TouchpadBack,

    // ----------- D pad buttons -----------
    Down,
    Left,
    Right,
    Up,

    // ----------- Joystick input -----------
    LeftXMinus,
    LeftXPlus,
    LeftYMinus,
    LeftYPlus,
    RightXMinus,
    RightXPlus,
    RightYMinus,
    RightYPlus,

    // ----------- Motion input -----------
    AccelUp,
    AccelDown,
    AccelLeft,
    AccelRight,
    AccelForward,
    AccelBackward,
    GyroPitchUp,
    GyroPitchDown,
    GyroRollLeft,
    GyroRollRight,
    GyroYawLeft,
    GyroYawRight,
}

impl Output {
    pub fn to_string(&self, profile_kind: ProfileKind) -> Option<&'static str> {
        Some(match self {
            Output::Down => "Down",
            Output::Left => "Left",
            Output::Right => "Right",
            Output::Up => "Up",
            Output::LeftXMinus => "Left X-",
            Output::LeftXPlus => "Left X+",
            Output::LeftYMinus => "Left Y-",
            Output::LeftYPlus => "Left Y+",
            Output::RightXMinus => "Right X-",
            Output::RightXPlus => "Right X+",
            Output::RightYMinus => "Right Y-",
            Output::RightYPlus => "Right Y+",
            _ => match profile_kind {
                ProfileKind::Ps4 => match self {
                    Output::CrossA => "Cross",
                    Output::CircleB => "Circle",
                    Output::SquareX => "Square",
                    Output::TriangleY => "Triangle",
                    Output::PsGuide => "Ps",
                    Output::Share => "Share",
                    Output::R1Rb => "R1",
                    Output::R2Rt => "R2",
                    Output::R3Rs => "R3",
                    Output::L1Lb => "L1",
                    Output::L2Lt => "L2",
                    Output::L3Ls => "L3",
                    Output::OptionsStart => "Options",
                    Output::TouchpadBack => "Touchpad",
                    Output::AccelUp => "Accel Up",
                    Output::AccelDown => "Accel Down",
                    Output::AccelLeft => "Accel Left",
                    Output::AccelRight => "Accel Right",
                    Output::AccelForward => "Accel Forward",
                    Output::AccelBackward => "Accel Backward",
                    Output::GyroPitchUp => "Gyro Pitch Up",
                    Output::GyroPitchDown => "Gyro Pitch Down",
                    Output::GyroRollLeft => "Gyro Roll Left",
                    Output::GyroRollRight => "Gyro Roll Right",
                    Output::GyroYawLeft => "Gyro Yaw Left",
                    Output::GyroYawRight => "Gyro Yaw Right",
                    _ => panic!(),
                },
                ProfileKind::Xbox360 => match self {
                    Output::CrossA => "A",
                    Output::CircleB => "B",
                    Output::SquareX => "X",
                    Output::TriangleY => "Y",
                    Output::PsGuide => "Guide",
                    Output::R1Rb => "Lb",
                    Output::R2Rt => "Rt",
                    Output::R3Rs => "Rs",
                    Output::L1Lb => "Lb",
                    Output::L2Lt => "Lt",
                    Output::L3Ls => "Ls",
                    Output::OptionsStart => "Start",
                    Output::TouchpadBack => "Back",
                    _ => panic!(),
                },
            },
        })
    }
}
