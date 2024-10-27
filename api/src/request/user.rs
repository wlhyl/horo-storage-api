use serde::Deserialize;

#[cfg(feature = "swagger")]
use utoipa::ToSchema;
use validator::Validate;

/// 登录user
#[cfg_attr(feature = "swagger", derive(ToSchema))]
#[derive(Deserialize, Validate)]
pub struct LoginUserRequest {
    /// 用户名
    #[validate(length(min = 1), non_control_character)]
    pub name: String,
    /// 密码
    #[validate(length(min = 1), non_control_character)]
    pub password: String,
}

/// insert/update user
#[cfg_attr(feature = "swagger", derive(ToSchema))]
#[derive(Deserialize, Validate)]
pub struct UpdateUserRequest {
    /// 密码
    #[validate(length(min = 1), non_control_character)]
    pub password: String,
    /// 旧密码
    #[validate(length(min = 1), non_control_character)]
    pub old_password: String,
}
