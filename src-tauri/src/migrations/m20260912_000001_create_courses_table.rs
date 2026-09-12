use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Courses::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Courses::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Courses::Title).string().not_null())
                    .col(ColumnDef::new(Courses::Version).string().not_null())
                    .col(ColumnDef::new(Courses::InstalledAt).timestamp().not_null())
                    .col(ColumnDef::new(Courses::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Courses::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum Courses {
    Table,
    Id,
    Title,
    Version,
    InstalledAt,
    UpdatedAt,
}
