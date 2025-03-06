use sea_orm::sea_query::extension::postgres::TypeAlterStatement;
use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
enum Password {
    Table,
    Email,
    Username,
    EncryptedEmail,
    EncryptedUsername,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Password::Table)
                    .rename_column(Password::Email, Password::EncryptedEmail)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Password::Table)
                    .rename_column(Password::Username, Password::EncryptedUsername)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Password::Table)
                    .rename_column(Password::EncryptedEmail, Password::Email)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Password::Table)
                    .rename_column(Password::EncryptedUsername, Password::Username)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}
