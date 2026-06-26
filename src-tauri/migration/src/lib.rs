pub use sea_orm_migration::prelude::*;
mod m20260513_215331_create_tables_v1;
mod m20260625_221540_profile_refactor_v2;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260513_215331_create_tables_v1::Migration),
            Box::new(m20260625_221540_profile_refactor_v2::Migration),
        ]
    }
}
