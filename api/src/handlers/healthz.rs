use actix_web::{get, web, HttpResponse, Responder};
use log::info;
use sea_orm::{ConnectionTrait, Statement};

use crate::{error::Error, state::AppState};

// #[cfg_attr(feature = "swagger",
// utoipa::path(
//     tag = "健康检查",
//     context_path = "/api/horo-admin",
//     responses(
//         (status = 200, description = "服务正常"),
//         (status = 500, description = "服务异常"),
//     )
// ))]
#[get("/readiness")]
pub async fn readiness_handler(app_state: web::Data<AppState>) -> Result<impl Responder, Error> {
    let db = &app_state.db;

    info!("检查数据库连接...");
    db.query_one(Statement::from_string(
        sea_orm::DatabaseBackend::MySql, // MariaDB 使用 MySQL 协议
        "SELECT 1",
    ))
    .await
    .map_err(|error| Error::Database(format!("数据库连接失败：{}", error)))?;

    info!("数据库连接正常");

    Ok(HttpResponse::Ok().json("ok"))
}

// #[cfg_attr(feature = "swagger",
// utoipa::path(
//     tag = "健康检查",
//     context_path = "/api/horo-admin",
//     responses(
//         (status = 200, description = "服务正常"),
//     )
// ))]
#[get("/liveness")]
pub async fn liveness_handler() -> impl Responder {
    HttpResponse::Ok().json("ok")
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};
    use sea_orm::MockDatabase;

    #[actix_web::test]
    async fn test_liveness() {
        let app = test::init_service(App::new().service(liveness_handler)).await;

        let req = test::TestRequest::get().uri("/liveness").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_readiness() {
        let db = MockDatabase::new()
            .append_query_results([[1i32]]) // 指定返回类型为 i32
            .into_connection();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState { db }))
                .service(readiness_handler),
        )
        .await;

        let req = test::TestRequest::get().uri("/readiness").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());
    }
}
