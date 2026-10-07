use crate::data::ns_input::NsInput;
use crate::data::output::Output;
use crate::entities::profile_kind_type::ProfileKindType;
use crate::entities::{condition, profile};
use crate::profiles::input::input::Input;
use crate::profiles::input::value_input::ValueInput;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::profile_kind::ProfileKind;
    use migration::{Migrator, MigratorTrait};
    use sea_orm::{ConnectionTrait, Database};
    use std::collections::HashMap;

    #[tokio::test]
    async fn xbox_motion_upgrade_preserves_profiles_and_runs_once() {
        let path = std::env::temp_dir().join(format!("ns2windows-motion-{}.db", Uuid::new_v4()));
        std::fs::File::create(&path).unwrap();
        let db = Database::connect(format!("sqlite://{}", path.to_string_lossy()))
            .await
            .unwrap();
        Migrator::up(&db, None).await.unwrap();
        let repository = ProfileRepository::new(db.clone());
        let custom = Input::Value(ValueInput::new(NsInput::GyroYawLeft));
        repository
            .save_profile(Profile::new(
                "Xbox custom".into(),
                ProfileKind::Xbox360,
                [
                    (Output::GyroPitchUp, custom.clone()),
                    (Output::CrossA, Input::Value(ValueInput::new(NsInput::B))),
                ]
                .into_iter()
                .collect(),
            ))
            .await
            .unwrap();
        repository
            .save_profile(Profile::new(
                "PS4 unchanged".into(),
                ProfileKind::Ps4,
                HashMap::new(),
            ))
            .await
            .unwrap();
        let original = profile::Entity::find()
            .filter(profile::Column::Name.eq("Xbox custom"))
            .one(&db)
            .await
            .unwrap()
            .unwrap();
        let conditions_before = condition::Entity::find()
            .filter(condition::Column::ProfileId.eq(original.id))
            .all(&db)
            .await
            .unwrap();
        assert!(
            repository
                .backfill_xbox_motion_defaults(false)
                .await
                .unwrap()
        );
        let upgraded = repository
            .find_profile_by_name("Xbox custom")
            .await
            .unwrap()
            .unwrap();
        assert_eq!(upgraded.outputs.len(), 13);
        assert_eq!(upgraded.outputs[&Output::GyroPitchUp], custom);
        assert_eq!(
            upgraded.outputs[&Output::AccelRight],
            Input::Value(ValueInput::new(NsInput::AccelRight))
        );
        let after = profile::Entity::find()
            .filter(profile::Column::Name.eq("Xbox custom"))
            .one(&db)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(after, original);
        for before in conditions_before {
            assert_eq!(
                condition::Entity::find_by_id(before.id)
                    .one(&db)
                    .await
                    .unwrap()
                    .unwrap(),
                before
            );
        }
        assert!(
            repository
                .find_profile_by_name("PS4 unchanged")
                .await
                .unwrap()
                .unwrap()
                .outputs
                .is_empty()
        );
        // Retry after an interrupted marker save adds no duplicate mappings.
        repository
            .backfill_xbox_motion_defaults(false)
            .await
            .unwrap();
        assert_eq!(
            condition::Entity::find()
                .filter(condition::Column::ProfileId.eq(original.id))
                .all(&db)
                .await
                .unwrap()
                .len(),
            13
        );
        condition::Entity::delete_many()
            .filter(condition::Column::ProfileId.eq(original.id))
            .filter(
                condition::Column::Output
                    .eq(crate::entities::out_input_type::OutputType::AccelRight),
            )
            .exec(&db)
            .await
            .unwrap();
        assert!(
            !repository
                .backfill_xbox_motion_defaults(true)
                .await
                .unwrap()
        );
        assert!(
            !repository
                .find_profile_by_name("Xbox custom")
                .await
                .unwrap()
                .unwrap()
                .outputs
                .contains_key(&Output::AccelRight)
        );
        drop(repository);
        db.close().await.unwrap();
        std::fs::remove_file(path).unwrap();
    }

    #[tokio::test]
    async fn backfill_is_transactional_on_failure() {
        let db = Database::connect("sqlite::memory:").await.unwrap();
        Migrator::up(&db, None).await.unwrap();
        let repository = ProfileRepository::new(db.clone());
        repository
            .save_profile(Profile::new(
                "Xbox".into(),
                ProfileKind::Xbox360,
                HashMap::new(),
            ))
            .await
            .unwrap();
        // Fail after at least one output has been inserted in the transaction.
        db.execute_unprepared("CREATE TRIGGER fail_motion BEFORE INSERT ON conditions WHEN NEW.output = 'GyroPitchUp' BEGIN SELECT RAISE(ABORT, 'test failure'); END;").await.unwrap();
        assert!(
            repository
                .backfill_xbox_motion_defaults(false)
                .await
                .is_err()
        );
        assert!(condition::Entity::find().all(&db).await.unwrap().is_empty());
    }
}

impl ProfileRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// One-time data upgrade, gated by the persisted settings marker. The
    /// transaction is idempotent if saving that marker fails after commit.
    pub async fn backfill_xbox_motion_defaults(
        &self,
        completed: bool,
    ) -> Result<bool, RepositoryError> {
        if completed {
            return Ok(false);
        }
        let txn = self.db.begin().await?;
        let profiles = profile::Entity::find()
            .filter(profile::Column::Kind.eq(ProfileKindType::Xbox360))
            .all(&txn)
            .await?;
        for profile in profiles {
            let existing = condition::Entity::find()
                .filter(condition::Column::ProfileId.eq(profile.id))
                .all(&txn)
                .await?;
            for (output, input) in [
                (Output::AccelUp, NsInput::AccelUp),
                (Output::AccelDown, NsInput::AccelDown),
                (Output::AccelLeft, NsInput::AccelLeft),
                (Output::AccelRight, NsInput::AccelRight),
                (Output::AccelForward, NsInput::AccelForward),
                (Output::AccelBackward, NsInput::AccelBackward),
                (Output::GyroPitchUp, NsInput::GyroPitchUp),
                (Output::GyroPitchDown, NsInput::GyroPitchDown),
                (Output::GyroRollLeft, NsInput::GyroRollLeft),
                (Output::GyroRollRight, NsInput::GyroRollRight),
                (Output::GyroYawLeft, NsInput::GyroYawLeft),
                (Output::GyroYawRight, NsInput::GyroYawRight),
            ] {
                if !existing
                    .iter()
                    .any(|condition| condition.output == output.into())
                {
                    self.save_condition(
                        &txn,
                        output,
                        Input::Value(ValueInput::new(input)),
                        profile.id,
                    )
                    .await?;
                }
            }
        }
        txn.commit().await?;
        Ok(true)
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
