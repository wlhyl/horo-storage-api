use serde::Serialize;

#[cfg(feature = "swagger")]
use utoipa::ToSchema;

/// 认证成功的token
#[derive(Serialize)]
#[cfg_attr(feature = "swagger", derive(ToSchema))]
pub struct Token {
    token: String,
}

impl From<String> for Token {
    fn from(token: String) -> Self {
        Self { token }
    }
}
