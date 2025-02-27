use super::m20250227_191111_create_table_user::User;
use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
pub enum Password {
    Table,
    Id,
    UserId,
    WebsiteURL,
    AppName,
    Username,
    Email,
    EncryptedPassword,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Password::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Password::Id).uuid().primary_key())
                    .col(ColumnDef::new(Password::UserId).uuid().not_null())
                    .col(ColumnDef::new(Password::WebsiteURL).text())
                    .col(ColumnDef::new(Password::AppName).text())
                    .col(ColumnDef::new(Password::Username).text())
                    .col(ColumnDef::new(Password::Email).text())
                    .col(
                        ColumnDef::new(Password::EncryptedPassword)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Password::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Password::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("password_user_id_fkey")
                    .from_tbl(Password::Table)
                    .from_col(Password::UserId)
                    .to_tbl(User::Table)
                    .to_col(User::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(Password::Table)
                    .name("password_website_url_index")
                    .col(Password::WebsiteURL)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(Password::Table)
                    .name("password_app_name_index")
                    .col(Password::AppName)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("password_user_id_fkey")
                    .table(Password::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("password_website_url_index")
                    .table(Password::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .name("password_app_name_index")
                    .table(Password::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table(Password::Table).to_owned())
            .await?;

        Ok(())
    }
}
