use sea_orm_migration::prelude::*;

use crate::m20250227_191111_create_table_user::User;

#[derive(DeriveIden)]
pub enum Session {
    Table,
    Id,
    UserId,
    Token,
    ExpiresAt,
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
                    .table(Session::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Session::Id).uuid().primary_key())
                    .col(ColumnDef::new(Session::UserId).uuid().not_null())
                    .col(ColumnDef::new(Session::Token).string().not_null())
                    .col(
                        ColumnDef::new(Session::ExpiresAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Session::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Session::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("session_user_id_fkey")
                    .from_tbl(Session::Table)
                    .from_col(Session::UserId)
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
                    .table(Session::Table)
                    .name("session_token_index")
                    .col(Session::Token)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("session_user_id_fkey")
                    .table(Session::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .table(Session::Table)
                    .name("session_token_index")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(Session::Table).to_owned())
            .await?;

        Ok(())
    }
}
