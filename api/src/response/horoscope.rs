use super::{date_time_format, option_date_time_format};
use chrono::NaiveDateTime;
use serde::Serialize;

#[cfg(feature = "swagger")]
use utoipa::ToSchema;

// 参考连接
// https://github.com/SeaQL/sea-orm/discussions/781
#[cfg_attr(feature = "swagger", derive(ToSchema))]
#[derive(Serialize)]
pub struct Horoscope {
    id: u32,
    name: String,
    gender: bool,
    birth_year: i32,
    birth_month: u8,
    birth_day: u8,
    birth_hour: u8,
    birth_minute: u8,
    birth_second: u8,
    time_zone_offset: f64,
    is_dst: bool,
    location: Location,
    description: String,
    #[cfg_attr(feature = "swagger", schema(value_type = String))]
    #[serde(with = "date_time_format")]
    created_at: NaiveDateTime,
    #[cfg_attr(feature = "swagger", schema(value_type = String))]
    #[serde(with = "option_date_time_format")]
    updated_at: Option<NaiveDateTime>,
}

#[cfg_attr(feature = "swagger", derive(ToSchema))]
#[derive(Serialize)]
pub struct Location {
    id: u32,
    name: String,
    is_east: bool,
    longitude_degree: u16,
    longitude_minute: u8,
    longitude_second: u8,
    is_north: bool,
    latitude_degree: u8,
    latitude_minute: u8,
    latitude_second: u8,
}

impl From<entity::locations::Model> for Location {
    fn from(location: entity::locations::Model) -> Self {
        Self {
            id: location.id,
            name: location.name,
            is_east: location.is_east,
            longitude_degree: location.longitude_degree,
            longitude_minute: location.longitude_minute,
            longitude_second: location.longitude_second,
            is_north: location.is_north,
            latitude_degree: location.latitude_degree,
            latitude_minute: location.latitude_minute,
            latitude_second: location.latitude_second,
        }
    }
}

impl Horoscope {
    pub fn new(native: entity::horoscopes::Model, geo: entity::locations::Model) -> Self {
        Self {
            id: native.id,
            name: native.name,
            gender: native.gender,
            birth_year: native.birth_year,
            birth_month: native.birth_month,
            birth_day: native.birth_day,
            birth_hour: native.birth_hour,
            birth_minute: native.birth_minute,
            birth_second: native.birth_second,
            time_zone_offset: native.time_zone_offset,
            is_dst: native.is_dst,
            location: geo.into(),
            description: native.description,
            // user_id: native.user_id,
            created_at: native.created_at,
            updated_at: native.updated_at,
        }
    }
}
