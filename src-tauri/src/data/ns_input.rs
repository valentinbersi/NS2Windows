use serde::{Deserialize, Serialize};
use std::fmt::Display;

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
