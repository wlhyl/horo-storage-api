use serde::Deserialize;
use validator::{Validate, ValidationError};

#[cfg(feature = "swagger")]
use utoipa::ToSchema;

use super::location::LocationRequest;

#[cfg_attr(feature = "swagger", derive(ToSchema))]
#[derive(Deserialize, Validate)]
pub struct HoroscopeRequest {
    /// 姓名
    #[validate(length(min = 1, max = 30), non_control_character)]
    pub name: String,
    /// 性别
    pub gender: bool,
    /// 年，最小值1900
    #[validate(range(min = 1900, message = "年最小1900"))]
    pub birth_year: i32,
    /// 月
    #[validate(range(min = 1, max = 12, message = "1<=月份<=12"))]
    pub birth_month: u8,
    /// 日
    #[validate(range(min = 1, max = 31, message = "1<=日期<=31"))]
    pub birth_day: u8,
    /// 时
    #[validate(range(min = 0, max = 23, message = "0<=时<=23"))]
    pub birth_hour: u8,
    /// 分
    #[validate(range(min = 0, max = 59, message = "0<=分<=59"))]
    pub birth_minute: u8,
    /// 秒
    #[validate(range(min = 0, max = 59, message = "0<=秒<=59"))]
    pub birth_second: u8,
    /// 出生地时区，东区为正数，西区为负数
    #[validate(range(min = -12.0, max = 12.0, message = "-12<=时区<=12"))]
    pub time_zone_offset: f64,
    /// 出生时的夏令时，有夏令时：true，无夏令时： false
    pub is_dst: bool,
    pub location: LocationRequest,

    /// 说明文字
    pub description: String,
}

#[cfg_attr(feature = "swagger", derive(ToSchema))]
#[derive(Deserialize, Validate)]
#[validate(schema(function = "validate_at_least_one_field"))]
pub struct UpdateHoroscopeRequest {
    /// 姓名
    #[validate(length(min = 1, max = 30), non_control_character)]
    pub name: Option<String>,
    /// 性别
    pub gender: Option<bool>,
    /// 年，最小值1900
    #[validate(range(min = 1900, message = "年最小1900"))]
    pub birth_year: Option<i32>,
    /// 月
    #[validate(range(min = 1, max = 12, message = "1<=月份<=12"))]
    pub birth_month: Option<u8>,
    /// 日
    #[validate(range(min = 1, max = 31, message = "1<=日期<=31"))]
    pub birth_day: Option<u8>,
    /// 时
    #[validate(range(min = 0, max = 23, message = "0<=时<=23"))]
    pub birth_hour: Option<u8>,
    /// 分
    #[validate(range(min = 0, max = 59, message = "0<=分<=59"))]
    pub birth_minute: Option<u8>,
    /// 秒
    #[validate(range(min = 0, max = 59, message = "0<=秒<=59"))]
    pub birth_second: Option<u8>,
    /// 出生地时区，东区为正数，西区为负数
    #[validate(range(min = -12.0, max = 12.0, message = "-12<=时区<=12"))]
    pub time_zone_offset: Option<f64>,
    /// 出生时的夏令时，有夏令时：true，无夏令时： false
    pub is_dst: Option<bool>,
    /// 地理位置信息
    pub location: Option<LocationRequest>,
    /// 说明文字
    pub description: Option<String>,
}

fn validate_at_least_one_field(request: &UpdateHoroscopeRequest) -> Result<(), ValidationError> {
    let has_value = request.name.is_some() 
        || request.gender.is_some()
        || request.birth_year.is_some()
        || request.birth_month.is_some()
        || request.birth_day.is_some()
        || request.birth_hour.is_some()
        || request.birth_minute.is_some()
        || request.birth_second.is_some()
        || request.time_zone_offset.is_some()
        || request.is_dst.is_some()
        || request.location.is_some()
        || request.description.is_some();

    if !has_value {
        return Err(ValidationError::new("至少需要设置一个字段"));
    }
    Ok(())
}
