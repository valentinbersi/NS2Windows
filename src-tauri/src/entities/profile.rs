use crate::entities::profile_kind_type::ProfileKindType;
use sea_orm::entity::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, DeriveEntityModel)]
#[sea_orm(table_name = "profiles")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    pub kind: ProfileKindType,

    #[sea_orm(unique)]
    pub name: String,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::condition::Entity")]
    Conditions,
}

impl Related<super::condition::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Conditions.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
