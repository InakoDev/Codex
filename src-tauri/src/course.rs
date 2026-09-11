use sea_orm::entity::prelude::*;

// Based this stuff on something on GitHub.
// I need to implement progress tracking, though i'm unsure of what way I want to implement that.
// So I'll just have it like this for now.

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "courses")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,

    pub title: String,
    pub version: String,

    #[sea_orm(column_type = "Timestamp")]
    pub installed_at: DateTimeUtc,

    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}