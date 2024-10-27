use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use log::debug;
use serde::{Deserialize, Serialize};

use crate::error::{self, Error};

pub fn generate_save_token(
    user: &entity::user::Model,
    secret: &str,
    token_expire_seconds: u64,
) -> Result<String, Error> {
    let exp = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::seconds(
            token_expire_seconds.try_into().map_err(|e| {
                debug!("错误：{e}, 配置的token过期时间：{token_expire_seconds}");
                error::Error::InternalServerError("应用配置错误，请修改token过期时间的配置".into())
            })?,
        ))
        .ok_or_else(|| {
            debug!("计算token过期时间错误！");
            Error::InternalServerError("生成token错误".to_string())
        })?
        .timestamp();

    let claims = Claims {
        id: user.id,
        name: user.name.clone(),
        exp,
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )?;

    Ok(token)
}

pub fn verify(token: &str, secret: &str) -> Result<Claims, Error> {
    let token = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )?;
    Ok(token.claims)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: u32,
    name: String,
    exp: i64,
}
