//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.1

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "block_rewards")]
pub struct Model {
    #[sea_orm(
        primary_key,
        auto_increment = false,
        column_type = "VarBinary(StringLen::None)"
    )]
    pub address_hash: Vec<u8>,
    #[sea_orm(primary_key, auto_increment = false)]
    pub address_type: String,
    #[sea_orm(
        primary_key,
        auto_increment = false,
        column_type = "VarBinary(StringLen::None)"
    )]
    pub block_hash: Vec<u8>,
    #[sea_orm(column_type = "Decimal(Some((100, 0)))", nullable)]
    pub reward: Option<BigDecimal>,
    pub inserted_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::blocks::Entity",
        from = "Column::BlockHash",
        to = "super::blocks::Column::Hash",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Blocks,
}

impl Related<super::blocks::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Blocks.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
