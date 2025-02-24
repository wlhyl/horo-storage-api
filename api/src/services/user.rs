use chrono::Local;
use rand::{distr::Alphanumeric, Rng};
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, QueryFilter};

use crate::{
    error::Error,
    request::{LoginUserRequest, UpdateUserRequest},
    response::Token,
    utils::{password_encoder, token::generate_save_token},
};

pub async fn login(
    db: &sea_orm::DatabaseConnection,
    token_expire_seconds: u64,
    req: &LoginUserRequest,
) -> Result<Token, Error> {
    let user = entity::users::Entity::find()
        .filter(entity::users::Column::Username.eq(&req.name))
        .one(db)
        .await?
        .ok_or_else(|| Error::NotFound(format!("用户：{}不存在", req.name)))?;

    let password_with_salt = format!("{}{}", req.password, user.salt);

    if !password_encoder::verify(&password_with_salt, &user.password_hash)? {
        return Err(Error::Forbidden("密码错误".to_string()));
    }

    let token = generate_save_token(&user, &user.salt, token_expire_seconds)?;

    let mut updated_user: entity::users::ActiveModel = user.into();
    updated_user.last_login_at = ActiveValue::Set(Some(Local::now().naive_local()));
    updated_user.update(db).await?;

    Ok(Token::from(token))
}

pub async fn update_password(
    db: &sea_orm::DatabaseConnection,
    user_id: u32,
    req: &UpdateUserRequest,
) -> Result<(), Error> {
    let user = entity::users::Entity::find_by_id(user_id)
        .one(db)
        .await?
        .ok_or_else(|| Error::NotFound(format!("用户ID: {}不存在", user_id)))?;

    let old_password = format!("{}{}", req.old_password, user.salt);

    if !password_encoder::verify(&old_password, &user.password_hash)? {
        return Err(Error::BadRequest("旧密码不正确".to_string()));
    }

    let salt: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(5)
        .map(char::from)
        .collect();

    let mut active_user: entity::users::ActiveModel = user.into();
    let new_password = format!("{}{}", req.password, salt);
    let password_hash = password_encoder::encode(&new_password)?;

    active_user.password_hash = ActiveValue::Set(password_hash);
    active_user.salt = ActiveValue::Set(salt);
    active_user.last_login_at = ActiveValue::Set(Some(Local::now().naive_local()));

    active_user.update(db).await?;

    Ok(())
}
