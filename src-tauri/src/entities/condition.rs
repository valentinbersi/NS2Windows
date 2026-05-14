use crate::entities::out_input_type::OutputType;
use sea_orm::prelude::*;
use sea_orm_migration::prelude::*;
use uuid::Uuid;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, DeriveEntityModel)]
#[sea_orm(table_name = "conditions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    pub output: OutputType,

    pub profile_id: Uuid,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::profile::Entity",
        from = "Column::ProfileId",
        to = "super::profile::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Profile,

    #[sea_orm(has_one = "super::value_condition::Entity")]
    ValueCondition,
}

impl Related<super::profile::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Profile.def()
    }
}

impl Related<super::value_condition::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ValueCondition.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
