use crate::data::output::Output;
use sea_orm::entity::prelude::*;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum OutputType {
    // ----------- Main Buttons -----------
    #[sea_orm(string_value = "CrossA")]
    CrossA,
    #[sea_orm(string_value = "CircleB")]
    CircleB,
    #[sea_orm(string_value = "SquareX")]
    SquareX,
    #[sea_orm(string_value = "TriangleY")]
    TriangleY,

    // ----------- Special buttons -----------
    #[sea_orm(string_value = "PsGuide")]
    PsGuide,
    #[sea_orm(string_value = "Share")]
    Share,

    // ----------- Right buttons -----------
    #[sea_orm(string_value = "R1Lb")]
    R1Rb,
    #[sea_orm(string_value = "R2Rt")]
    R2Rt,
    #[sea_orm(string_value = "R3Rs")]
    R3Rs,

    // ----------- Left buttons -----------
    #[sea_orm(string_value = "L1Lb")]
    L1Lb,
    #[sea_orm(string_value = "L2Lt")]
    L2Lt,
    #[sea_orm(string_value = "L3Ls")]
    L3Ls,

    // ----------- Menu buttons -----------
    #[sea_orm(string_value = "OptionsStart")]
    OptionsStart,
    #[sea_orm(string_value = "TouchpadBack")]
    TouchpadBack,

    // ----------- D pad buttons -----------
    #[sea_orm(string_value = "Down")]
    Down,
    #[sea_orm(string_value = "Left")]
    Left,
    #[sea_orm(string_value = "Right")]
    Right,
    #[sea_orm(string_value = "Up")]
    Up,

    // ----------- Joystick input -----------
    #[sea_orm(string_value = "LeftXMinus")]
    LeftXMinus,
    #[sea_orm(string_value = "LeftXPlus")]
    LeftXPlus,
    #[sea_orm(string_value = "LeftYMinus")]
    LeftYMinus,
    #[sea_orm(string_value = "LeftYPlus")]
    LeftYPlus,
    #[sea_orm(string_value = "RightXMinus")]
    RightXMinus,
    #[sea_orm(string_value = "RightXPlus")]
    RightXPlus,
    #[sea_orm(string_value = "RightYMinus")]
    RightYMinus,
    #[sea_orm(string_value = "RightYPlus")]
    RightYPlus,

    // ----------- Motion input -----------
    #[sea_orm(string_value = "AccelUp")]
    AccelUp,
    #[sea_orm(string_value = "AccelDown")]
    AccelDown,
    #[sea_orm(string_value = "AccelLeft")]
    AccelLeft,
    #[sea_orm(string_value = "AccelRight")]
    AccelRight,
    #[sea_orm(string_value = "AccelForward")]
    AccelForward,
    #[sea_orm(string_value = "AccelBackward")]
    AccelBackward,
    #[sea_orm(string_value = "GyroPitchUp")]
    GyroPitchUp,
    #[sea_orm(string_value = "GyroPitchDown")]
    GyroPitchDown,
    #[sea_orm(string_value = "GyroRollLeft")]
    GyroRollLeft,
    #[sea_orm(string_value = "GyroRollRight")]
    GyroRollRight,
    #[sea_orm(string_value = "GyroYawLeft")]
    GyroYawLeft,
    #[sea_orm(string_value = "GyroYawRight")]
    GyroYawRight,
}

impl From<Output> for OutputType {
    fn from(value: Output) -> Self {
        match value {
            Output::CrossA => OutputType::CrossA,
            Output::CircleB => OutputType::CircleB,
            Output::SquareX => OutputType::SquareX,
            Output::TriangleY => OutputType::TriangleY,
            Output::PsGuide => OutputType::PsGuide,
            Output::Share => OutputType::Share,
            Output::R1Rb => OutputType::R1Rb,
            Output::R2Rt => OutputType::R2Rt,
            Output::R3Rs => OutputType::R3Rs,
            Output::L1Lb => OutputType::L1Lb,
            Output::L2Lt => OutputType::L2Lt,
            Output::L3Ls => OutputType::L3Ls,
            Output::OptionsStart => OutputType::OptionsStart,
            Output::TouchpadBack => OutputType::TouchpadBack,
            Output::Down => OutputType::Down,
            Output::Left => OutputType::Left,
            Output::Right => OutputType::Right,
            Output::Up => OutputType::Up,
            Output::LeftXMinus => OutputType::LeftXMinus,
            Output::LeftXPlus => OutputType::LeftXPlus,
            Output::LeftYMinus => OutputType::LeftYMinus,
            Output::LeftYPlus => OutputType::LeftYPlus,
            Output::RightXMinus => OutputType::RightXMinus,
            Output::RightXPlus => OutputType::RightXPlus,
            Output::RightYMinus => OutputType::RightYMinus,
            Output::RightYPlus => OutputType::RightYPlus,
            Output::AccelUp => OutputType::AccelUp,
            Output::AccelDown => OutputType::AccelDown,
            Output::AccelLeft => OutputType::AccelLeft,
            Output::AccelRight => OutputType::AccelRight,
            Output::AccelForward => OutputType::AccelForward,
            Output::AccelBackward => OutputType::AccelBackward,
            Output::GyroPitchUp => OutputType::GyroPitchUp,
            Output::GyroPitchDown => OutputType::GyroPitchDown,
            Output::GyroRollLeft => OutputType::GyroRollLeft,
            Output::GyroRollRight => OutputType::GyroRollRight,
            Output::GyroYawLeft => OutputType::GyroYawLeft,
            Output::GyroYawRight => OutputType::GyroYawRight,
        }
    }
}

impl From<OutputType> for Output {
    fn from(value: OutputType) -> Self {
        match value {
            OutputType::CrossA => Output::CrossA,
            OutputType::CircleB => Output::CircleB,
            OutputType::SquareX => Output::SquareX,
            OutputType::TriangleY => Output::TriangleY,
            OutputType::PsGuide => Output::PsGuide,
            OutputType::Share => Output::Share,
            OutputType::R1Rb => Output::R1Rb,
            OutputType::R2Rt => Output::R2Rt,
            OutputType::R3Rs => Output::R3Rs,
            OutputType::L1Lb => Output::L1Lb,
            OutputType::L2Lt => Output::L2Lt,
            OutputType::L3Ls => Output::L3Ls,
            OutputType::OptionsStart => Output::OptionsStart,
            OutputType::TouchpadBack => Output::TouchpadBack,
            OutputType::Down => Output::Down,
            OutputType::Left => Output::Left,
            OutputType::Right => Output::Right,
            OutputType::Up => Output::Up,
            OutputType::LeftXMinus => Output::LeftXMinus,
            OutputType::LeftXPlus => Output::LeftXPlus,
            OutputType::LeftYMinus => Output::LeftYMinus,
            OutputType::LeftYPlus => Output::LeftYPlus,
            OutputType::RightXMinus => Output::RightXMinus,
            OutputType::RightXPlus => Output::RightXPlus,
            OutputType::RightYMinus => Output::RightYMinus,
            OutputType::RightYPlus => Output::RightYPlus,
            OutputType::AccelUp => Output::AccelUp,
            OutputType::AccelDown => Output::AccelDown,
            OutputType::AccelLeft => Output::AccelLeft,
            OutputType::AccelRight => Output::AccelRight,
            OutputType::AccelForward => Output::AccelForward,
            OutputType::AccelBackward => Output::AccelBackward,
            OutputType::GyroPitchUp => Output::GyroPitchUp,
            OutputType::GyroPitchDown => Output::GyroPitchDown,
            OutputType::GyroRollLeft => Output::GyroRollLeft,
            OutputType::GyroRollRight => Output::GyroRollRight,
            OutputType::GyroYawLeft => Output::GyroYawLeft,
            OutputType::GyroYawRight => Output::GyroYawRight,
        }
    }
}
