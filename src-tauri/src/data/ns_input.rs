use serde::{Deserialize, Serialize};

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
    Chat,

    // ----------- Right buttons -----------
    R,
    Zr,
    Z,
    Tr,
    Sr,
    Gr,
    RTrigger,

    // ----------- Left buttons -----------
    L,
    Zl,
    Tl,
    Sl,
    Gl,
    LTrigger,

    // ----------- Menu buttons -----------
    Plus,
    Minus,
    StartPause,

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
