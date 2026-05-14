use crate::data::profile_kind::ProfileKind;
use sea_orm::entity::prelude::*;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum ProfileKindType {
    // ----------- Main Buttons -----------
    #[sea_orm(string_value = "Ps4")]
    Ps4,
    #[sea_orm(string_value = "Xbox360")]
    Xbox360,
}

impl From<ProfileKind> for ProfileKindType {
    fn from(kind: ProfileKind) -> Self {
        match kind {
            ProfileKind::Ps4 => ProfileKindType::Ps4,
            ProfileKind::Xbox360 => ProfileKindType::Xbox360,
        }
    }
}

impl From<ProfileKindType> for ProfileKind {
    fn from(value: ProfileKindType) -> Self {
        match value {
            ProfileKindType::Ps4 => ProfileKind::Ps4,
            ProfileKindType::Xbox360 => ProfileKind::Xbox360,
        }
    }
}
