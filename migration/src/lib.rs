pub use sea_orm_migration::prelude::*;

mod m20250227_191111_create_table_password;
mod m20250227_191111_create_table_user;
mod m20250227_191649_create_table_recovery_code;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250227_191111_create_table_user::Migration),
            Box::new(m20250227_191111_create_table_password::Migration),
            Box::new(m20250227_191649_create_table_recovery_code::Migration),
        ]
    }
}
