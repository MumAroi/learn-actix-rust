pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_user_table;
mod m20240407_094039_create_real_property_table;
mod m20240407_103025_create_organize_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_user_table::Migration),
            Box::new(m20240407_094039_create_real_property_table::Migration),
            Box::new(m20240407_103025_create_organize_table::Migration),
        ]
    }
}
