use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
enum Password {
    Table,
    IsDeleted,
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
                    .add_column(ColumnDef::new(Password::IsDeleted).boolean().default(false))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Password::Table)
                    .drop_column(Password::IsDeleted)
                    .to_owned(),
            )
            .await
    }
}
