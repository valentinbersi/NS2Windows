use crate::data::output::Output;
use crate::entities::{condition, profile};
use crate::profiles::input::input::Input;
use crate::profiles::profile::Profile;
use crate::repositories::repository_error::RepositoryError;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DatabaseTransaction, EntityTrait,
    ModelTrait, QueryFilter, Set, TransactionTrait,
};
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
        input: Input,
        profile_id: Uuid,
    ) -> Result<(), RepositoryError> {
        let id = Uuid::now_v7();

        let serialized_input = postcard::to_allocvec(&input)?;

        condition::ActiveModel {
            id: Set(id),
            input: Set(serialized_input),
            output: Set(output.into()),
            profile_id: Set(profile_id),
        }
        .insert(txn)
        .await?;

        Ok(())
    }

    async fn inner_delete_profile(
        &self,
        txn: &DatabaseTransaction,
        name: &str,
    ) -> Result<(), RepositoryError> {
        let profile = profile::Entity::find()
            .filter(profile::Column::Name.eq(name))
            .one(txn)
            .await?;

        if let Some(profile) = profile {
            profile.delete(txn).await?;
        }

        Ok(())
    }

    pub async fn save_profile(&self, profile: Profile) -> Result<(), RepositoryError> {
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

        for (output, input) in profile.outputs.into_iter() {
            self.save_condition(&txn, output, input, id).await?;
        }

        txn.commit().await.map_err(Into::into)
    }

    pub async fn delete_profile(&self, name: &str) -> Result<(), RepositoryError> {
        let txn = self.db.begin().await?;
        self.inner_delete_profile(&txn, name).await?;
        txn.commit().await.map_err(Into::into)
    }

    pub async fn find_profile_by_name(
        &self,
        name: &str,
    ) -> Result<Option<Profile>, RepositoryError> {
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
                    .all(&txn)
                    .await?;

                txn.commit().await?;

                let outputs = conditions
                    .into_iter()
                    .map(|condition| {
                        let output = condition.output.into();
                        let input = postcard::from_bytes(&condition.input)?;

                        Ok((output, input))
                    })
                    .collect::<Result<_, postcard::Error>>()?;

                Ok(Some(Profile::new(
                    profile.name,
                    profile.kind.into(),
                    outputs,
                )))
            }
        }
    }

    pub async fn profile_names(&self) -> Result<Vec<String>, RepositoryError> {
        profile::Entity::find()
            .all(&self.db)
            .await
            .map(|profiles| profiles.into_iter().map(|profile| profile.name).collect())
            .map_err(Into::into)
    }
}
