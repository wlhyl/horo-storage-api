use actix_web::web;

use crate::handlers::{
    healthz::{liveness_handler, readiness_handler},
    native::{delete_native, get_native_pages, insert_native, update_native},
    user::{login, update_user},
};

pub fn health_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(readiness_handler).service(liveness_handler);
}
pub fn api_routes(cfg: &mut web::ServiceConfig) {
    cfg
        // native
        .service(get_native_pages)
        .service(insert_native)
        .service(update_native)
        .service(delete_native)
        // login
        .service(login)
        .service(update_user);
}
