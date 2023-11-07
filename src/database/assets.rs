//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.3

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "assets")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
    pub address: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub chain_id: i32,
    #[sea_orm(column_type = "Text")]
    pub name: String,
    #[sea_orm(column_type = "Text")]
    pub symbol: String,
    pub decimals: i32,
    #[sea_orm(column_type = "Text")]
    pub logo_url: String,
    #[sea_orm(column_type = "Double")]
    pub price: f64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::aprs::Entity")]
    Aprs,
    #[sea_orm(has_many = "super::bribes::Entity")]
    Bribes,
}

impl Related<super::aprs::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Aprs.def()
    }
}

impl Related<super::bribes::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Bribes.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
