//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.6

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "horoscopes")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    pub name: String,
    pub gender: bool,
    pub birth_year: i32,
    pub birth_month: u8,
    pub birth_day: u8,
    pub birth_hour: u8,
    pub birth_minute: u8,
    pub birth_second: u8,
    #[sea_orm(column_type = "Double")]
    pub time_zone_offset: f64,
    pub is_dst: bool,
    #[sea_orm(unique)]
    pub location_id: u32,
    #[sea_orm(column_type = "Text", nullable)]
    pub description: String,
    pub user_id: u32,
    pub created_at: DateTime,
    pub updated_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::locations::Entity",
        from = "Column::LocationId",
        to = "super::locations::Column::Id",
        on_update = "Restrict",
        on_delete = "Restrict"
    )]
    Locations,
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::UserId",
        to = "super::users::Column::Id",
        on_update = "Restrict",
        on_delete = "Restrict"
    )]
    Users,
}

impl Related<super::locations::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Locations.def()
    }
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
