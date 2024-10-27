use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use chrono::Local;

use log::debug;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, ModelTrait, PaginatorTrait,
    QueryFilter, TransactionTrait,
};

use crate::{
    error::Error,
    extractor::AuthenticatedUser,
    request::{NativeRequest, PageQueryparams},
    response::{Native, PageResponser},
    state::AppState,
};

/// 分布列出所有natives
#[cfg_attr(feature = "swagger", 
utoipa::path(
    tag="时间",
    context_path="/api/horo-admin",
    params(PageQueryparams),
    responses(
        (status = 200, description = "返回native", body = Native),
    ),
    security(
        ("token" = [])
    ),
)
)]
#[get("/natives")]
pub async fn get_native_pages(
    app_state: web::Data<AppState>,
    r: web::Query<PageQueryparams>,
    user: AuthenticatedUser,
) -> Result<impl Responder, Error> {
    let db = &app_state.db;
    let user_id = user.id;

    let native_geo_pages = entity::native::Entity::find()
        .find_also_related(entity::geo::Entity)
        .filter(entity::native::Column::UserId.eq(user_id))
        .paginate(db, r.size);

    let total_pages = native_geo_pages.num_pages().await?;
    let natives: Result<Vec<Native>, Error> = native_geo_pages
        .fetch_page(r.page)
        .await?
        .into_iter()
        .map(|(native, geo)| {
            geo.ok_or_else(|| {
                debug!(
                    "数据库中native表与geo表的一对一约束异常，native id={}的记录没有对应的geo",
                    native.id
                );
                Error::InternalServerError(
                    "数据异常：native记录没有对应geo记录，查看debug日志了解详情！".into(),
                )
            })
            .map(|geo| Native::new(native, geo))
        })
        .collect();

    let natives = natives?;

    let res = PageResponser {
        data: natives,
        total: total_pages,
    };

    Ok(HttpResponse::Ok().json(res))
}

/// 新增一条native
#[cfg_attr(feature = "swagger",
utoipa::path(
    tag="时间",
    context_path="/api/horo-admin",
    request_body=NativeRequest,
    responses(
        (status = 200, description = "返回native", body = Native),
        (status = 400, description = "返回错误", body = String),
    ),
    security(
        ("token" = [])
    ),
)
)]
#[post("/natives")]
pub async fn insert_native(
    app_state: web::Data<AppState>,
    r: actix_web_validator::Json<NativeRequest>,
    user: AuthenticatedUser,
) -> Result<impl Responder, Error> {
    let db = app_state.db.begin().await?;
    let r = r.into_inner();
    let user_id = user.id;

    let geo_insert = entity::geo::ActiveModel {
        name: ActiveValue::Set(r.geo.name),
        east: ActiveValue::Set(r.geo.east),
        long_d: ActiveValue::Set(r.geo.long_d),
        long_m: ActiveValue::Set(r.geo.long_m),
        long_s: ActiveValue::Set(r.geo.long_s),
        north: ActiveValue::Set(r.geo.north),
        lat_d: ActiveValue::Set(r.geo.lat_d),
        lat_m: ActiveValue::Set(r.geo.lat_m),
        lat_s: ActiveValue::Set(r.geo.lat_s),
        ..Default::default()
    };

    let geo_inserted = geo_insert.insert(&db).await?;

    let native_insert = entity::native::ActiveModel {
        name: ActiveValue::Set(r.name),
        sex: ActiveValue::Set(r.sex),
        year: ActiveValue::Set(r.year),
        month: ActiveValue::Set(r.month),
        day: ActiveValue::Set(r.day),
        hour: ActiveValue::Set(r.hour),
        minute: ActiveValue::Set(r.minute),
        second: ActiveValue::Set(r.second),

        tz: ActiveValue::Set(r.tz),
        st: ActiveValue::Set(r.st),

        geo_id: ActiveValue::Set(geo_inserted.id),

        describe: ActiveValue::Set(r.describe),
        user_id: ActiveValue::Set(user_id),
        create_date: ActiveValue::Set(Local::now().naive_local()),
        ..Default::default()
    };

    let native = native_insert.insert(&db).await?;

    db.commit().await?;

    let native = Native::new(native, geo_inserted);

    Ok(HttpResponse::Ok().json(native))
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
#[put("/natives/{id}")]
pub async fn update_native(
    app_state: web::Data<AppState>,
    id: web::Path<u32>,
    r: actix_web_validator::Json<NativeRequest>,
    user: AuthenticatedUser,
) -> Result<impl Responder, Error> {
    let db = app_state.db.begin().await?;
    let r = r.into_inner();
    let id = id.into_inner();
    let user_id = user.id;

    let (native, geo) = entity::native::Entity::find_by_id(id)
        .find_also_related(entity::geo::Entity)
        .filter(entity::native::Column::UserId.eq(user_id))
        .one(&db)
        .await?
        .ok_or_else(|| {
            debug!(
                "user id: `{}`没有对应的native记录，native id: `{}`",
                user_id, id
            );
            Error::BadRequest("native不存在".into())
        })?;

    let geo = geo.ok_or_else(|| {
        debug!(
            "数据库中native表与geo表的一对一约束异常，native id={}的记录没有对应的geo",
            native.id
        );
        Error::InternalServerError(
            "数据异常：native记录没有对应geo记录，查看debug日志了解详情！".into(),
        )
    })?;

    let mut geo: entity::geo::ActiveModel = geo.into();
    geo.name = ActiveValue::Set(r.geo.name);
    geo.east = ActiveValue::Set(r.geo.east);
    geo.long_d = ActiveValue::Set(r.geo.long_d);
    geo.long_m = ActiveValue::Set(r.geo.long_m);
    geo.long_s = ActiveValue::Set(r.geo.long_s);
    geo.north = ActiveValue::Set(r.geo.north);
    geo.lat_d = ActiveValue::Set(r.geo.lat_d);
    geo.lat_m = ActiveValue::Set(r.geo.lat_m);
    geo.lat_s = ActiveValue::Set(r.geo.lat_s);

    geo.update(&db).await?;

    let mut native: entity::native::ActiveModel = native.into();

    native.name = ActiveValue::Set(r.name);
    native.sex = ActiveValue::Set(r.sex);
    native.year = ActiveValue::Set(r.year);
    native.month = ActiveValue::Set(r.month);
    native.day = ActiveValue::Set(r.day);
    native.hour = ActiveValue::Set(r.hour);
    native.minute = ActiveValue::Set(r.minute);
    native.second = ActiveValue::Set(r.second);

    native.tz = ActiveValue::Set(r.tz);
    native.st = ActiveValue::Set(r.st);

    native.describe = ActiveValue::Set(r.describe);
    native.last_update_date = ActiveValue::Set(Some(Local::now().naive_local()));

    native.update(&db).await?;

    db.commit().await?;
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
#[delete("/natives/{id}")]
pub async fn delete_native(
    app_state: web::Data<AppState>,
    id: web::Path<u32>,
    user: AuthenticatedUser,
) -> Result<impl Responder, Error> {
    let db = app_state.db.begin().await?;
    let id = id.into_inner();
    let user_id = user.id;

    let native = entity::native::Entity::find_by_id(id)
        .filter(entity::native::Column::UserId.eq(user_id))
        .one(&db)
        .await?
        .ok_or_else(|| {
            debug!(
                "删除 native 错误，user id: `{}`没有对应的native记录，native id: `{}`",
                user_id, id
            );
            Error::BadRequest("native不存在".into())
        })?;

    let geo_id = native.geo_id;
    native.delete(&db).await?;

    entity::prelude::Geo::delete_by_id(geo_id).exec(&db).await?;

    db.commit().await?;

    Ok(HttpResponse::Ok().finish())
}
