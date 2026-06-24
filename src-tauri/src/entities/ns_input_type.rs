use crate::data::ns_input::NsInput;
use sea_orm::entity::prelude::*;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum NsInputType {
    // ----------- Main Buttons -----------
    #[sea_orm(string_value = "B")]
    B,
    #[sea_orm(string_value = "A")]
    A,
    #[sea_orm(string_value = "Y")]
    Y,
    #[sea_orm(string_value = "X")]
    X,

    // ----------- Special buttons -----------
    #[sea_orm(string_value = "Home")]
    Home,
    #[sea_orm(string_value = "Capture")]
    Capture,
    #[sea_orm(string_value = "Chat")]
    Chat,

    // ----------- Right buttons -----------
    #[sea_orm(string_value = "R")]
    R,
    #[sea_orm(string_value = "Zr")]
    Zr,
    #[sea_orm(string_value = "Tr")]
    Tr,
    #[sea_orm(string_value = "Sr")]
    Sr,
    #[sea_orm(string_value = "Gr")]
    Gr,
    #[sea_orm(string_value = "RTrigger")]
    RTrigger,

    // ----------- Left buttons -----------
    #[sea_orm(string_value = "L")]
    L,
    #[sea_orm(string_value = "Zl")]
    Zl,
    #[sea_orm(string_value = "Tl")]
    Tl,
    #[sea_orm(string_value = "Sl")]
    Sl,
    #[sea_orm(string_value = "Gl")]
    Gl,
    #[sea_orm(string_value = "LTrigger")]
    LTrigger,

    // ----------- Menu buttons -----------
    #[sea_orm(string_value = "Plus")]
    Plus,
    #[sea_orm(string_value = "Minus")]
    Minus,

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

impl From<NsInput> for NsInputType {
    fn from(input: NsInput) -> Self {
        match input {
            NsInput::B => NsInputType::B,
            NsInput::A => NsInputType::A,
            NsInput::Y => NsInputType::Y,
            NsInput::X => NsInputType::X,

            NsInput::Home => NsInputType::Home,
            NsInput::Capture => NsInputType::Capture,
            NsInput::Chat => NsInputType::Chat,

            NsInput::R => NsInputType::R,
            NsInput::Zr => NsInputType::Zr,
            NsInput::Tr => NsInputType::Tr,
            NsInput::Sr => NsInputType::Sr,
            NsInput::Gr => NsInputType::Gr,

            NsInput::L => NsInputType::L,
            NsInput::Zl => NsInputType::Zl,
            NsInput::Tl => NsInputType::Tl,
            NsInput::Sl => NsInputType::Sl,
            NsInput::Gl => NsInputType::Gl,

            NsInput::Plus => NsInputType::Plus,
            NsInput::Minus => NsInputType::Minus,

            NsInput::Down => NsInputType::Down,
            NsInput::Left => NsInputType::Left,
            NsInput::Right => NsInputType::Right,
            NsInput::Up => NsInputType::Up,

            NsInput::LeftXMinus => NsInputType::LeftXMinus,
            NsInput::LeftXPlus => NsInputType::LeftXPlus,
            NsInput::LeftYMinus => NsInputType::LeftYMinus,
            NsInput::LeftYPlus => NsInputType::LeftYPlus,

            NsInput::RightXMinus => NsInputType::RightXMinus,
            NsInput::RightXPlus => NsInputType::RightXPlus,
            NsInput::RightYMinus => NsInputType::RightYMinus,
            NsInput::RightYPlus => NsInputType::RightYPlus,

            NsInput::AccelUp => NsInputType::AccelUp,
            NsInput::AccelDown => NsInputType::AccelDown,
            NsInput::AccelLeft => NsInputType::AccelLeft,
            NsInput::AccelRight => NsInputType::AccelRight,
            NsInput::AccelForward => NsInputType::AccelForward,
            NsInput::AccelBackward => NsInputType::AccelBackward,

            NsInput::GyroPitchUp => NsInputType::GyroPitchUp,
            NsInput::GyroPitchDown => NsInputType::GyroPitchDown,
            NsInput::GyroRollLeft => NsInputType::GyroRollLeft,
            NsInput::GyroRollRight => NsInputType::GyroRollRight,
            NsInput::GyroYawLeft => NsInputType::GyroYawLeft,
            NsInput::GyroYawRight => NsInputType::GyroYawRight,

            NsInput::RTrigger => NsInputType::RTrigger,
            NsInput::LTrigger => NsInputType::LTrigger,
        }
    }
}

impl From<NsInputType> for NsInput {
    fn from(value: NsInputType) -> Self {
        match value {
            NsInputType::B => NsInput::B,
            NsInputType::A => NsInput::A,
            NsInputType::Y => NsInput::Y,
            NsInputType::X => NsInput::X,

            NsInputType::Home => NsInput::Home,
            NsInputType::Capture => NsInput::Capture,
            NsInputType::Chat => NsInput::Chat,

            NsInputType::R => NsInput::R,
            NsInputType::Zr => NsInput::Zr,
            NsInputType::Tr => NsInput::Tr,
            NsInputType::Sr => NsInput::Sr,
            NsInputType::Gr => NsInput::Gr,

            NsInputType::L => NsInput::L,
            NsInputType::Zl => NsInput::Zl,
            NsInputType::Tl => NsInput::Tl,
            NsInputType::Sl => NsInput::Sl,
            NsInputType::Gl => NsInput::Gl,

            NsInputType::Plus => NsInput::Plus,
            NsInputType::Minus => NsInput::Minus,

            NsInputType::Down => NsInput::Down,
            NsInputType::Left => NsInput::Left,
            NsInputType::Right => NsInput::Right,
            NsInputType::Up => NsInput::Up,

            NsInputType::LeftXMinus => NsInput::LeftXMinus,
            NsInputType::LeftXPlus => NsInput::LeftXPlus,
            NsInputType::LeftYMinus => NsInput::LeftYMinus,
            NsInputType::LeftYPlus => NsInput::LeftYPlus,

            NsInputType::RightXMinus => NsInput::RightXMinus,
            NsInputType::RightXPlus => NsInput::RightXPlus,
            NsInputType::RightYMinus => NsInput::RightYMinus,
            NsInputType::RightYPlus => NsInput::RightYPlus,

            NsInputType::AccelUp => NsInput::AccelUp,
            NsInputType::AccelDown => NsInput::AccelDown,
            NsInputType::AccelLeft => NsInput::AccelLeft,
            NsInputType::AccelRight => NsInput::AccelRight,
            NsInputType::AccelForward => NsInput::AccelForward,
            NsInputType::AccelBackward => NsInput::AccelBackward,

            NsInputType::GyroPitchUp => NsInput::GyroPitchUp,
            NsInputType::GyroPitchDown => NsInput::GyroPitchDown,
            NsInputType::GyroRollLeft => NsInput::GyroRollLeft,
            NsInputType::GyroRollRight => NsInput::GyroRollRight,
            NsInputType::GyroYawLeft => NsInput::GyroYawLeft,
            NsInputType::GyroYawRight => NsInput::GyroYawRight,

            NsInputType::RTrigger => NsInput::RTrigger,
            NsInputType::LTrigger => NsInput::LTrigger,
        }
    }
}
