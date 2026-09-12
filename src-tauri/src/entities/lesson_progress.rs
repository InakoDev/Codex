use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "lesson_progress")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub course_id: String,

    #[sea_orm(primary_key, auto_increment = false)]
    pub lesson_id: String,

    #[sea_orm(column_type = "Timestamp")]
    pub completed_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
