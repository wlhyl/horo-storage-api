use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

use crate::{
    error::Error,
    extractor::AuthenticatedUser,
    request::{HoroscopeRequest, PageQueryparams, UpdateHoroscopeRequest},
    services,
    state::AppState,
};
#[cfg(feature = "swagger")]
use crate::response::Horoscope;

/// 分页列出所有horoscopes
#[cfg_attr(feature = "swagger", 
utoipa::path(
    tag="时间",
    context_path="/api/horo-admin",
    params(PageQueryparams),
    responses(
        (status = 200, description = "返回native", body = Horoscope),
    ),
    security(
        ("token" = [])
    ),
)
)]
#[get("/horoscopes")]
pub async fn get_horoscope_pages(
    app_state: web::Data<AppState>,
    r: web::Query<PageQueryparams>,
    user: AuthenticatedUser,
) -> Result<impl Responder, Error> {
    let pages = services::horoscope::get_pages(&app_state.db, user.id, &r).await?;
    Ok(HttpResponse::Ok().json(pages))
}

/// 根据id返回horoscope
#[cfg_attr(feature = "swagger",
utoipa::path(
    tag="时间",
    context_path="/api/horo-admin",
    responses(
        (status = 200, description = "返回native", body = Horoscope),
        (status = 400, description = "返回错误", body = String),
    ),
    security(
        ("token" = [])
    ),
)
)]
#[get("/horoscopes/{id}")]
pub async fn get_horoscope_by_id(
    app_state: web::Data<AppState>,
    id: web::Path<u32>,
    user: AuthenticatedUser,
) -> Result<impl Responder, Error> {
    let native = services::horoscope::get_by_id(&app_state.db, id.into_inner(), user.id).await?;
    Ok(HttpResponse::Ok().json(native))
}

/// 新增一条horoscope
#[cfg_attr(feature = "swagger",
utoipa::path(
    tag="时间",
    context_path="/api/horo-admin",
    request_body=HoroscopeRequest,
    responses(
        (status = 200, description = "返回native", body = Horoscope),
        (status = 400, description = "返回错误", body = String),
    ),
    security(
        ("token" = [])
    ),
)
)]
#[post("/horoscopes")]
pub async fn add_horoscope(
    app_state: web::Data<AppState>,
    r: actix_web_validator::Json<HoroscopeRequest>,
    user: AuthenticatedUser,
) -> Result<impl Responder, Error> {
    let horoscope = services::horoscope::insert(&app_state.db, r.into_inner(), user.id).await?;
    Ok(HttpResponse::Ok().json(horoscope))
}

/// 更新一条native
#[cfg_attr(feature = "swagger",
utoipa::path(
    tag="时间",
    context_path="/api/horo-admin",
    // request_body=NativeRequest,
    responses(
        (status = 200, description = "无返回body"),
        (status = 400, description = "返回错误", body = String),
    ),
    security(
        ("token" = [])
    ),
)
)]
#[put("/horoscopes/{id}")]
pub async fn update_native(
    app_state: web::Data<AppState>,
    id: web::Path<u32>,
    req: actix_web_validator::Json<UpdateHoroscopeRequest>,
    user: AuthenticatedUser,
) -> Result<impl Responder, Error> {
    services::horoscope::update(&app_state.db, id.into_inner(), user.id, &req).await?;
    Ok(HttpResponse::Ok().finish())
}

/// 删除一条mative
#[cfg_attr(feature = "swagger",
utoipa::path(
    tag="时间",
    context_path="/api/horo-admin",
    responses(
        (status = 200, description = "无返回body"),
        (status = 400, description = "返回错误", body = String),
    ),
    security(
        ("token" = [])
    ),
)
)]
#[delete("/horoscopes/{id}")]
pub async fn delete_native(
    app_state: web::Data<AppState>,
    id: web::Path<u32>,
    user: AuthenticatedUser,
) -> Result<impl Responder, Error> {
    services::horoscope::delete(&app_state.db, id.into_inner(), user.id).await?;
    Ok(HttpResponse::Ok().finish())
}
