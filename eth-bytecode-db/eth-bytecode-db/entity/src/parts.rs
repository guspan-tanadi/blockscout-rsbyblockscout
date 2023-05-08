//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.2

use super::sea_orm_active_enums::PartType;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "parts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub part_type: PartType,
    #[sea_orm(column_type = "Binary(BlobSize::Blob(None))")]
    pub data: Vec<u8>,
    #[sea_orm(column_type = "Text", nullable)]
    pub data_text: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::bytecode_parts::Entity")]
    BytecodeParts,
}

impl Related<super::bytecode_parts::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BytecodeParts.def()
    }
}

impl Related<super::bytecodes::Entity> for Entity {
    fn to() -> RelationDef {
        super::bytecode_parts::Relation::Bytecodes.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::bytecode_parts::Relation::Parts.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
