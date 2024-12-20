// migration/src/lib.rs

pub use sea_orm_migration::prelude::*;

// Add each migration file as a module
mod m20241220_000001_create_bakery_table;
mod m20241220_000002_create_chef_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            // Define the order of migrations.
            Box::new(m20241220_000001_create_bakery_table::Migration),
            Box::new(m20241220_000002_create_chef_table::Migration),
        ]
    }
}
