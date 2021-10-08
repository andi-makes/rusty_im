//! SeaORM Entity. Generated by sea-orm-codegen 0.2.4

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "item")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "Text")]
    pub name: String,
    #[sea_orm(column_type = "Text")]
    pub description: String,
    pub id_batch: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::batch::Entity",
        from = "Column::IdBatch",
        to = "super::batch::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Batch,
    #[sea_orm(has_many = "super::property::Entity")]
    Property,
}

impl Related<super::batch::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Batch.def()
    }
}

impl Related<super::property::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Property.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
