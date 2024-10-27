use serde::Deserialize;
use validator::Validate;

#[cfg(feature = "swagger")]
use utoipa::ToSchema;

use super::geo::GeoRequest;

#[cfg_attr(feature = "swagger", derive(ToSchema))]
#[derive(Deserialize, Validate)]
pub struct NativeRequest {
    /// 姓名
    #[validate(length(min = 1), non_control_character)]
    pub name: Option<String>,
    /// 性别
    pub sex: bool,
    /// 年，最小值1900
    #[validate(range(min = 1900, message = "年最小1900"))]
    pub year: i32,
    /// 月
    #[validate(range(min = 1, max = 12, message = "1<=月份<=12"))]
    pub month: u8,
    /// 日
    #[validate(range(min = 1, max = 31, message = "1<=日期<=31"))]
    pub day: u8,
    /// 时
    #[validate(range(min = 0, max = 23, message = "0<=时<=23"))]
    pub hour: u8,
    /// 分
    #[validate(range(min = 0, max = 59, message = "0<=分<=59"))]
    pub minute: u8,
    /// 秒
    #[validate(range(min = 0, max = 59, message = "0<=秒<=59"))]
    pub second: u8,
    /// 出生地时区，东区为正数，西区为负数
    #[validate(range(min = -12.0, max = 12.0, message = "-12<=时区<=12"))]
    pub tz: f64,
    /// 出生时的夏令时，有夏令时：true，无夏令时： false
    pub st: bool,
    pub geo: GeoRequest,

    /// 说明文字
    pub describe: Option<String>,
}
