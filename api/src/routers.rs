use actix_web::web;

use crate::handlers::{
    healthz::{liveness_handler, readiness_handler},
    horoscope::{delete_native, get_horoscope_by_id, get_horoscope_pages, add_horoscope, update_native},
    user::{login, update_user},
};

pub fn health_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(readiness_handler).service(liveness_handler);
}
pub fn api_routes(cfg: &mut web::ServiceConfig) {
    cfg
        // native
        .service(get_horoscope_pages)
        .service(get_horoscope_by_id)
        .service(add_horoscope)
        .service(update_native)
        .service(delete_native)
        // login
        .service(login)
        .service(update_user);
}
