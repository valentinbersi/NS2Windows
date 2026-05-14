use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Profiles::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Profiles::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Profiles::Kind).string().not_null())
                    .col(ColumnDef::new(Profiles::Name).string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Conditions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Conditions::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Conditions::Output).string().not_null())
                    .col(ColumnDef::new(Conditions::ProfileId).uuid().not_null())
                    .index(
                        IndexCreateStatement::new()
                            .name("unique_conditions_output_profileId")
                            .col(Conditions::Output)
                            .col(Conditions::ProfileId)
                            .unique(),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("foreign_conditions_profileId")
                            .from(Conditions::Table, Conditions::ProfileId)
                            .to(Profiles::Table, Profiles::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ValueConditions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ValueConditions::ConditionsId)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ValueConditions::Value).string().not_null())
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("foreign_valueConditions_conditionsId")
                            .from(ValueConditions::Table, ValueConditions::ConditionsId)
                            .to(Conditions::Table, Conditions::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ValueConditions::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Conditions::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Profiles::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Profiles {
    Table,
    Id,
    Kind,
    Name,
}

#[derive(DeriveIden)]
enum Conditions {
    Table,
    Id,
    Output,
    ProfileId,
}

#[derive(DeriveIden)]
enum ValueConditions {
    Table,
    ConditionsId,
    Value,
}
