use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum NsInput {
    // ----------- Main Buttons -----------
    B,
    A,
    Y,
    X,

    // ----------- Special buttons -----------
    Home,
    Capture,

    // ----------- Right buttons -----------
    R,
    Zr,
    Tr,
    Sr,
    Gr,

    // ----------- Left buttons -----------
    L,
    Zl,
    Tl,
    Sl,
    Gl,

    // ----------- Menu buttons -----------
    Plus,
    Minus,

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

impl Display for NsInput {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            NsInput::B => "B",
            NsInput::A => "A",
            NsInput::Y => "Y",
            NsInput::X => "X",
            NsInput::Home => "Home",
            NsInput::Capture => "Capture",
            NsInput::R => "R",
            NsInput::Tr => "TR",
            NsInput::Zr => "ZR",
            NsInput::Sr => "SR",
            NsInput::Gr => "GR",
            NsInput::L => "L",
            NsInput::Tl => "TL",
            NsInput::Zl => "ZL",
            NsInput::Sl => "SL",
            NsInput::Gl => "GL",
            NsInput::Plus => "Plus",
            NsInput::Minus => "Minus",
            NsInput::Down => "Down",
            NsInput::Left => "Left",
            NsInput::Right => "Right",
            NsInput::Up => "Up",
            NsInput::LeftXMinus => "Left X-",
            NsInput::LeftXPlus => "Left X+",
            NsInput::LeftYMinus => "Left Y-",
            NsInput::LeftYPlus => "Left Y+",
            NsInput::RightXMinus => "Right X-",
            NsInput::RightXPlus => "Right X+",
            NsInput::RightYMinus => "Right Y-",
            NsInput::RightYPlus => "Right Y+",
            NsInput::AccelUp => "Accel Up",
            NsInput::AccelDown => "Accel Down",
            NsInput::AccelLeft => "Accel Left",
            NsInput::AccelRight => "Accel Right",
            NsInput::AccelForward => "Accel Forward",
            NsInput::AccelBackward => "Accel Backward",
            NsInput::GyroPitchUp => "Gyro Pitch Up",
            NsInput::GyroPitchDown => "Gyro Pitch Down",
            NsInput::GyroRollLeft => "Gyro Roll Left",
            NsInput::GyroRollRight => "Gyro Roll Right",
            NsInput::GyroYawLeft => "Gyro Yaw Left",
            NsInput::GyroYawRight => "Gyro Yaw Right",
        })
    }
}
