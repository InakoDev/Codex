use sea_orm::entity::prelude::*;

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
