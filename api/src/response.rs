mod date_time_format;
mod option_date_time_format;

mod native;
mod user;

pub use native::{Geo, Native};
pub use user::Token;

use serde::Serialize;

#[cfg(feature = "swagger")]
use utoipa::ToSchema;

// 返回分页结果
#[cfg_attr(
    feature = "swagger", derive(ToSchema),
    // aliases(
        // PageResponserOfGeo = PageResponser<Vec<entity::geo::Model>>,
        // PageResponserOfArchive = PageResponser<Vec<Archive>>,
        // PageResponserOfStringList = PageResponser<Vec<String>>,
    // )
)]
#[derive(Serialize)]
pub struct PageResponser<T: Serialize> {
    /// 分页显示的数据
    pub data: T,
    /// 总页数
    pub total: u64,
}
