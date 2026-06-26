use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared("DELETE FROM value_conditions;")
            .await?;
        db.execute_unprepared("DELETE FROM conditions;").await?;
        db.execute_unprepared("DELETE FROM profiles;").await?;

        manager
            .drop_table(Table::drop().table(ValueConditions::Table).to_owned())
            .await?;

        // 4. Alter Conditions table
        manager
            .alter_table(
                Table::alter()
                    .table(Conditions::Table)
                    .add_column(
                        ColumnDef::new(Conditions::Input)
                            .blob()
                            .not_null()
                            .default(Value::Bytes(Some(vec![]))),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Conditions::Table)
                    .drop_column(Conditions::Input)
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ValueConditions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ValueConditions::ConditionId)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ValueConditions::Input).string().not_null())
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("foreign_valueConditions_conditionsId")
                            .from(ValueConditions::Table, ValueConditions::ConditionId)
                            .to(Conditions::Table, Conditions::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Conditions {
    Table,
    Id,
    Input,
}

#[derive(DeriveIden)]
enum ValueConditions {
    Table,
    ConditionId,
    Input,
}
