use actix_web::{post, put, web, HttpResponse, Responder};
use chrono::Local;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, QueryFilter};

use log::{debug, error, warn};

use crate::error::Error;
use crate::extractor::AuthenticatedUser;
use crate::request::{LoginUserRequest, UpdateUserRequest};
use crate::response::Token;
use crate::state::AppState;
use crate::utils::password_encoder;
use crate::utils::token::generate_save_token;

/// 登录
#[cfg_attr(feature = "swagger", 
utoipa::path(
    tag="用户",
    context_path="/api/horo-admin",
    // request_body=LoginUserRequest,
    responses(
        (status = 200, description = "OK", body = Token),
    ), 
)
)]
#[post("/login")]
pub async fn login(
    app_state: web::Data<AppState>,
    r: actix_web_validator::Json<LoginUserRequest>,
) -> Result<impl Responder, Error> {

    let db = &app_state.db;
    let token_expire_seconds = app_state.token_expire_seconds;

    let user = entity::user::Entity::find()
        .filter(entity::user::Column::Name.eq(r.name.clone()))
        .one(db)
        .await?
        .ok_or(Error::NotFound(format!("用户：{}不存在", r.name)))?;

    let password_with_salt = format!("{}{}", r.password, user.salt);

    if !password_encoder::verify(&password_with_salt, &user.password).unwrap_or_else(|error| {
        debug!("校验密码函数执行错误：{}", error);
        false
    }) {
        return Err(Error::Forbidden("密码认证错误！".into()));
    }

    let token = generate_save_token(&user, &user.salt, token_expire_seconds)?;

    let mut updated_user: entity::user::ActiveModel = user.into();
    updated_user.last_login_date = ActiveValue::Set(Some(Local::now().naive_local()));
    updated_user.update(db).await?;
    let token: Token = token.into();

    Ok(HttpResponse::Ok().json(token))
}

/// 更新用户
/// 返回：code: 202，更新的user
#[cfg_attr(feature = "swagger", 
utoipa::path(
    tag="用户",
    context_path="/api/horo-admin",
    responses(
        (status = 200, description = "OK", body=()),
    ),
    security(
        ("token" = [])
    ),
)
)]
#[put("/user")]
pub async fn update_user(
    app_state: web::Data<AppState>,
    user: AuthenticatedUser,
    updated_user: actix_web_validator::Json<UpdateUserRequest>,
) -> Result<impl Responder, Error> {
    let db = &app_state.db;
    let user_id = user.id;
    let updated_user = updated_user.into_inner();

    let user =  entity::user::Entity::find_by_id(user_id).one(db).await?.ok_or_else(||
    
  {
        debug!("没有找到登录用户 user id: {}", user_id);
        Error::InternalServerError(format!(
            "用户数据错误 user id: {}，查看debug日志了解详情",
            user_id
        ))
    })?;

    // let salt  = user.salt.clone();

    // 验证旧密码
    // if let Some(old_password) = updated_user.old_password {
    let old_password = format!("{}{}", updated_user.old_password, user.salt);

    if !password_encoder::verify(&old_password, &user.password).unwrap_or_else(|error| {
        error!("校验密码函数执行错误：{}", error);
        false
    }) {
        warn!("旧密码不正确，不能修改密码");
        return Err(Error::BadRequest("旧密码不正确，不能修改密码".into()));
    }

    let salt: String = thread_rng()
        .sample_iter(Alphanumeric)
        .take(5)
        .map(char::from)
        .collect();

    let mut user: entity::user::ActiveModel = user.into();
    let new_passowrd = format!("{}{}", updated_user.password, salt);
    let new_password = password_encoder::encode(&new_passowrd)?;

    user.password = ActiveValue::Set(new_password);
    user.salt = ActiveValue::Set(salt);

    user.update(db).await?;

    Ok(HttpResponse::Ok().finish())
}
