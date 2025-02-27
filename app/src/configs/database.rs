use sea_orm::{Database, DatabaseConnection, DbErr};

use super::env::Env;

pub async fn get_connection(env_variables: &Env) -> Result<DatabaseConnection, DbErr> {
    Database::connect(&env_variables.database_url).await
}
