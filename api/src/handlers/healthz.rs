use actix_web::{get, web, HttpResponse, Responder};
use log::info;
use sea_orm::{ConnectionTrait, Statement};

use crate::{error::Error, state::AppState};

#[get("/readiness")]
pub async fn readiness_handler(app_state: web::Data<AppState>) -> Result<impl Responder, Error> {
    let db = &app_state.db;

    info!("Check db connect...");
    db.query_one(Statement::from_string(
        sea_orm::DatabaseBackend::MySql,
        "select 'readiness' ",
    ))
    .await
    .map_err(|error| Error::DBError(format!("连接 db 错误：{}", error)))?;

    info!("Db connect is ok!");

    Ok(HttpResponse::Ok().json("ok"))
}

#[get("/liveness")]
pub async fn liveness_handler(_: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json("ok")
}
