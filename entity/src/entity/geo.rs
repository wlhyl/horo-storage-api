//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.1

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "geo")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    pub name: String,
    pub east: bool,
    pub long_d: u16,
    pub long_m: u8,
    pub long_s: u8,
    pub north: bool,
    pub lat_d: u8,
    pub lat_m: u8,
    pub lat_s: u8,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::native::Entity")]
    Native,
}

impl Related<super::native::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Native.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
