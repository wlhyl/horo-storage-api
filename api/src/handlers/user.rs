use actix_web::{post, put, web, HttpResponse, Responder};

use crate::error::Error;
use crate::extractor::AuthenticatedUser;
use crate::request::{LoginUserRequest, UpdateUserRequest};
#[cfg(feature = "swagger")]
use crate::response::Token;
use crate::services;
use crate::state::AppState;

#[cfg_attr(feature = "swagger", 
utoipa::path(
    tag = "用户",
    context_path = "/api/horo-admin",
    responses(
        (status = 200, description = "登录成功", body = Token),
        (status = 404, description = "用户不存在"),
        (status = 403, description = "密码错误"),
    ),
))]
#[post("/login")]
pub async fn login(
    app_state: web::Data<AppState>,
    req: actix_web_validator::Json<LoginUserRequest>,
) -> Result<impl Responder, Error> {
    let token = services::user::login(&app_state.db, app_state.token_expire_seconds, &req).await?;
    Ok(HttpResponse::Ok().json(token))
}

#[cfg_attr(feature = "swagger", 
utoipa::path(
    tag = "用户",
    context_path = "/api/horo-admin",
    responses(
        (status = 200, description = "更新成功"),
        (status = 400, description = "旧密码错误"),
        (status = 404, description = "用户不存在"),
    ),
    security(
        ("token" = [])
    ),
))]
#[put("/user")]
pub async fn update_user(
    app_state: web::Data<AppState>,
    user: AuthenticatedUser,
    req: actix_web_validator::Json<UpdateUserRequest>,
) -> Result<impl Responder, Error> {
    services::user::update_password(&app_state.db, user.id, &req).await?;
    Ok(HttpResponse::Ok().finish())
}
