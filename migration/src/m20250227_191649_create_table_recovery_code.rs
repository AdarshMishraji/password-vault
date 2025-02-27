use sea_orm_migration::prelude::*;

use crate::m20250227_191111_create_table_user::User;

#[derive(DeriveIden)]
pub enum RecoveryCode {
    Table,
    Id,
    UserId,
    CodeHash,
    EncryptedDEK,
    Used,
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
                    .table(RecoveryCode::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(RecoveryCode::Id).uuid().primary_key())
                    .col(ColumnDef::new(RecoveryCode::UserId).uuid().not_null())
                    .col(ColumnDef::new(RecoveryCode::CodeHash).string().not_null())
                    .col(
                        ColumnDef::new(RecoveryCode::EncryptedDEK)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RecoveryCode::Used)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(RecoveryCode::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(RecoveryCode::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("recovery_code_user_id_fkey")
                    .from_tbl(RecoveryCode::Table)
                    .from_col(RecoveryCode::UserId)
                    .to_tbl(User::Table)
                    .to_col(User::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("recovery_code_user_id_fkey")
                    .table(RecoveryCode::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(RecoveryCode::Table).to_owned())
            .await?;

        Ok(())
    }
}
