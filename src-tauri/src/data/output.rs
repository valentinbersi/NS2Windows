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

    // ----------- Left joystick output -----------
    LeftXMinus,
    LeftXPlus,
    LeftYMinus,
    LeftYPlus,

    // ----------- Right joystick output -----------
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
