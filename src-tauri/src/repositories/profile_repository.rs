use crate::data::output::Output;
use crate::entities::{condition, profile, value_condition};
use crate::profiles::input::Input;
use crate::profiles::profile::Profile;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DatabaseTransaction, DbErr, EntityTrait,
    ModelTrait, QueryFilter, Set, TransactionTrait,
};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Clone, Debug, Default)]
pub struct ProfileRepository {
    db: DatabaseConnection,
}

impl ProfileRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    async fn save_condition(
        &self,
        txn: &DatabaseTransaction,
        output: Output,
        profile_id: Uuid,
        condition: &Input,
    ) -> Result<(), DbErr> {
        let id = Uuid::now_v7();

        condition::ActiveModel {
            id: Set(id),
            output: Set(output.into()),
            profile_id: Set(profile_id),
        }
        .insert(txn)
        .await?;

        match condition {
            Input::Value(input) => {
                value_condition::ActiveModel {
                    condition_id: Set(id),
                    input: Set(input.clone().into()),
                }
                .insert(txn)
                .await?;
            }
        }

        Ok(())
    }

    async fn inner_delete_profile(
        &self,
        txn: &DatabaseTransaction,
        name: &str,
    ) -> Result<(), DbErr> {
        let profile = profile::Entity::find()
            .filter(profile::Column::Name.eq(name))
            .one(txn)
            .await?;

        if let Some(profile) = profile {
            profile.delete(txn).await?;
        }

        Ok(())
    }

    pub async fn save_profile(&self, profile: Profile) -> Result<(), DbErr> {
        let txn = self.db.begin().await?;

        self.inner_delete_profile(&txn, &profile.name).await?;

        let id = Uuid::now_v7();

        profile::ActiveModel {
            id: Set(id),
            kind: Set(profile.kind.into()),
            name: Set(profile.name),
        }
        .insert(&txn)
        .await?;

        for (output, condition) in profile.outputs.iter() {
            self.save_condition(&txn, output.clone(), id, condition)
                .await?;
        }

        txn.commit().await
    }

    pub async fn delete_profile(&self, name: &str) -> Result<(), DbErr> {
        let txn = self.db.begin().await?;
        self.inner_delete_profile(&txn, name).await?;
        txn.commit().await
    }

    pub async fn find_profile_by_name(&self, name: &str) -> Result<Option<Profile>, DbErr> {
        let txn = self.db.begin().await?;

        let profile = profile::Entity::find()
            .filter(profile::Column::Name.eq(name))
            .one(&txn)
            .await?;

        match profile {
            None => Ok(None),
            Some(profile) => {
                let conditions = condition::Entity::find()
                    .filter(condition::Column::ProfileId.eq(profile.id))
                    .find_also_related(value_condition::Entity)
                    .all(&txn)
                    .await?;

                txn.commit().await?;

                let outputs = conditions
                    .into_iter()
                    .map(|(condition, value_condition)| {
                        (
                            condition.output.into(),
                            Input::Value(value_condition.unwrap().input.into()),
                        )
                    })
                    .collect::<HashMap<Output, Input>>();

                Ok(Some(Profile::new(
                    profile.name,
                    profile.kind.into(),
                    outputs,
                )))
            }
        }
    }

    pub async fn profile_names(&self) -> Result<Vec<String>, DbErr> {
        profile::Entity::find()
            .all(&self.db)
            .await
            .map(|profiles| profiles.into_iter().map(|profile| profile.name).collect())
    }
}
