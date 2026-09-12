use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Just one row per completed lesson. That's it.
        // The statistics for courses started, lessons completed, days learning,
        // and a calendar view are just different ways of counting.grouping this table.

        manager
            .create_table(
                Table::create()
                    .table(LessonProgress::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(LessonProgress::CourseId).string().not_null())
                    .col(ColumnDef::new(LessonProgress::LessonId).string().not_null())
                    .col(
                        ColumnDef::new(LessonProgress::CompletedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .col(LessonProgress::CourseId)
                            .col(LessonProgress::LessonId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(LessonProgress::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum LessonProgress {
    Table,
    CourseId,
    LessonId,
    CompletedAt,
}
