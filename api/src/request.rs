mod geo;
mod native;
mod user;

pub use geo::GeoRequest;
pub use native::NativeRequest;
pub use user::{LoginUserRequest, UpdateUserRequest};

use serde::Deserialize;

#[cfg(feature = "swagger")]
use utoipa::{IntoParams, ToSchema};

/// 分页查询参数
#[derive(Deserialize)]
#[cfg_attr(feature = "swagger", derive(ToSchema, IntoParams))]
pub struct PageQueryparams {
    /// 第几页，从0开始
    pub page: u64,
    /// 每页大小
    pub size: u64,
}
