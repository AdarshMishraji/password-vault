use std::sync::Arc;

use sea_orm::{Database, DatabaseConnection, DbErr};

use super::env::Env;

pub async fn get_connection(env_variables: &Env) -> Result<Arc<DatabaseConnection>, DbErr> {
    Ok(Arc::new(
        Database::connect(&env_variables.database_url).await?,
    ))
}
