use super::{date_time_format, option_date_time_format};
use chrono::NaiveDateTime;
use serde::Serialize;

#[cfg(feature = "swagger")]
use utoipa::ToSchema;

// #[cfg_attr(feature = "swagger", derive(ToSchema))]
// #[derive(Serialize)]
// pub struct Native {
//     pub id: u32,
//     pub name: Option<String>,
//     pub sex: bool,
//     pub year: i32,
//     pub month: u8,
//     pub day: u8,
//     pub hour: u8,
//     pub minute: u8,
//     pub second: u8,
//     pub tz: f64,
//     pub st: bool,
//     pub geo: Option<entity::geo::Model>,
//     pub is_native: bool,
//     pub app: entity::app::Model,
//     pub describe: Option<String>,
//     #[serde(with = "date_time_format")]
//     pub create_date: NaiveDateTime,
//     #[serde(with = "option_date_time_format")]
//     pub last_update_date: Option<NaiveDateTime>,
// }

// impl Native {
//     pub fn new(
//         native: entity::native::Model,
//         geo: Option<entity::geo::Model>,
//         app: entity::app::Model,
//     ) -> Self {
//         Self {
//             id: native.id,
//             name: native.name,
//             sex: native.sex,
//             year: native.year,
//             month: native.month,
//             day: native.day,
//             hour: native.hour,
//             minute: native.minute,
//             second: native.second,
//             tz: native.tz,
//             st: native.st,
//             geo,
//             is_native: native.is_native,
//             app,
//             describe: native.describe,
//             create_date: native.create_date,
//             last_update_date: native.last_update_date,
//         }
//     }
// }

// 参考连接
// https://github.com/SeaQL/sea-orm/discussions/781
#[cfg_attr(feature = "swagger", derive(ToSchema))]
#[derive(Serialize)]
pub struct Native {
    pub id: u32,
    pub name: Option<String>,
    pub sex: bool,
    pub year: i32,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub tz: f64,
    pub st: bool,
    // pub geo: Option<entity::geo::Model>,
    geo: Geo,
    // pub is_native: bool,
    // pub app: entity::app::Model,
    // app: App,
    pub describe: Option<String>,
    // pub user_id: u32,
    #[cfg_attr(feature = "swagger", schema(value_type = String))]
    #[serde(with = "date_time_format")]
    pub create_date: NaiveDateTime,
    #[cfg_attr(feature = "swagger", schema(value_type = String))]
    #[serde(with = "option_date_time_format")]
    pub last_update_date: Option<NaiveDateTime>,
}

// #[cfg_attr(feature = "swagger", derive(ToSchema))]
// #[derive(Serialize)]
// pub struct App {
//     id: u32,
//     name: String,
// }
// impl TryGetable for App {
//     fn try_get_by<I: sea_orm::ColIdx>(
//         res: &sea_orm::prelude::QueryResult,
//         _index: I,
//     ) -> Result<Self, sea_orm::TryGetError> {
//         let id: u32 = res.try_get_by("app_id")?;
//         let name: String = res.try_get_by("app_name")?;
//         Ok(App { id, name })
//     }
// }

#[cfg_attr(feature = "swagger", derive(ToSchema))]
#[derive(Serialize)]
pub struct Geo {
    id: u32,
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

impl From<entity::geo::Model> for Geo {
    fn from(geo: entity::geo::Model) -> Self {
        Self {
            id: geo.id,
            name: geo.name,
            east: geo.east,
            long_d: geo.long_d,
            long_m: geo.long_m,
            long_s: geo.long_s,
            north: geo.north,
            lat_d: geo.lat_d,
            lat_m: geo.lat_m,
            lat_s: geo.lat_s,
        }
    }
}

// impl TryGetable for Geo {
//     fn try_get_by<I: sea_orm::ColIdx>(
//         res: &sea_orm::prelude::QueryResult,
//         _index: I,
//     ) -> Result<Self, sea_orm::TryGetError> {
//         let id: Option<u32> = res.try_get_by("geo_id")?;
//         let name: Option<String> = res.try_get_by("geo_name")?;
//         let long: Option<f64> = res.try_get_by("long")?;
//         let lat: Option<f64> = res.try_get_by("lat")?;

//         if name.is_some() && long.is_some() && lat.is_some() {
//             let geo = Geo {
//                 id: id.unwrap(),
//                 name: name.unwrap(),
//                 long: long.unwrap(),
//                 lat: lat.unwrap(),
//             };
//             Ok(geo)
//         } else if name.is_none() && long.is_none() && lat.is_none() {
//             Err(sea_orm::TryGetError::Null("geo是null值".to_string()))
//         } else {
//             let msg = "geo 表数据异常，字段存在null值!".to_string();
//             let error = sea_orm::TryGetError::DbErr(sea_orm::DbErr::Custom(msg));
//             Err(error)
//         }
//     }
// }

impl Native {
    pub fn new(native: entity::native::Model, geo: entity::geo::Model) -> Self {
        Self {
            id: native.id,
            name: native.name,
            sex: native.sex,
            year: native.year,
            month: native.month,
            day: native.day,
            hour: native.hour,
            minute: native.minute,
            second: native.second,
            tz: native.tz,
            st: native.st,
            geo: geo.into(),
            describe: native.describe,
            // user_id: native.user_id,
            create_date: native.create_date,
            last_update_date: native.last_update_date,
        }
    }
}
