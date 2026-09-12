pub mod m20260912_000001_create_courses_table;

use sea_orm_migration::prelude::*;

pub struct Migrator;
#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20260912_000001_create_courses_table::Migration)]
    }
}
