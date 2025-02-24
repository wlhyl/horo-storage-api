use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use log::debug;
use serde::{Deserialize, Serialize};

use crate::error::Error;

pub fn generate_save_token(
    user: &entity::users::Model,
    secret: &str,
    token_expire_seconds: u64,
) -> Result<String, Error> {
    let exp = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::seconds(
            token_expire_seconds.try_into().map_err(|e| {
                debug!("token过期时间转换错误: {}", e);
                Error::Internal("token配置错误".to_string())
            })?,
        ))
        .ok_or_else(|| {
            debug!("token过期时间计算错误");
            Error::Internal("token生成失败".to_string())
        })?
        .timestamp();

    let claims = Claims {
        id: user.id,
        name: user.username.clone(),
        exp,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|e| {
        debug!("token编码错误: {}", e);
        Error::Auth("token生成失败".to_string())
    })
}

pub fn verify(token: &str, secret: &str) -> Result<Claims, Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map(|token_data| token_data.claims)
    .map_err(|e| {
        debug!("token验证错误: {}", e);
        Error::Auth("无效的token".to_string())
    })
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: u32,
    pub name: String,
    exp: i64,
}
