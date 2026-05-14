use uuid::Uuid;

use crate::entities::ns_input_type::NsInputType;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, DeriveEntityModel)]
#[sea_orm(table_name = "value_conditions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub condition_id: Uuid,

    #[sea_orm(unique)]
    pub input: NsInputType,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::condition::Entity",
        from = "Column::ConditionId",
        to = "super::condition::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Condition,
}

impl Related<super::condition::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Condition.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
