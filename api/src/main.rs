use std::net::SocketAddrV4;

use actix_web::{middleware, web, App, HttpServer};
use actix_web_validator::JsonConfig;
use clap::Parser;

use storage_api::{
    config::Config,
    database::setup_database,
    error::Error,
    middleware::Auth,
    routers::{api_routes, health_routes},
    state::AppState,
};

#[cfg(feature = "swagger")]
use storage_api::swagger::ApiDoc;

#[cfg(feature = "swagger")]
use utoipa::OpenApi;

#[cfg(feature = "swagger")]
use utoipa_swagger_ui::SwaggerUi;

#[cfg(feature = "cors")]
use actix_cors::Cors;

use storage_api::cli::ServerConfig;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server_config = ServerConfig::parse();
    let config = Config::from_env().expect("Failed to load configuration");

    // 初始化日志
    log4rs::init_file(&config.log4rs_config, Default::default())
        .expect("Failed to initialize logging");

    // 设置数据库
    let db = setup_database(&config.database_url)
        .await
        .expect("Failed to connect to database");

    let shared_data = web::Data::new(AppState {
        db,
        // jwt_secret,
        token_expire_seconds: config.token_expire_seconds,
    });

    #[cfg(feature = "swagger")]
    let openapi = ApiDoc::openapi();

    HttpServer::new(move || {
        #[cfg(feature = "cors")]
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        let auth = Auth;

        let json_config = JsonConfig::default()
            // .limit(4096)
            // .content_type(|mime| {  // <- accept text/plain content type
            //     mime.type_() == mime::TEXT && mime.subtype() == mime::PLAIN
            // })
            .error_handler(|err, _req| {
                // <- create custom error response
                // error::InternalError::from_response(err, HttpResponse::Conflict().finish()).into()
                Error::from(err).into()
            });

        let app = App::new()
            // .app_data(QsQueryConfig::default().error_handler(|err, _| {
            //     let json_error = match &err {
            //         Error::Validate(error) => ValidationErrorJsonPayload::from(error),
            //         _ => ValidationErrorJsonPayload { message: err.to_string(), fields: Vec::new() },
            //     };
            //     error::InternalError::from_response(err, HttpResponse::Conflict().json(json_error)).into()
            // }))
            .app_data(json_config)
            .app_data(shared_data.clone())
            .configure(health_routes)
            .service(
                web::scope("/api/horo-admin")
                    .configure(api_routes)
                    .wrap(auth),
            );

        #[cfg(feature = "swagger")]
        let app = app.service(
            SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
        );
        #[cfg(feature = "cors")]
        let app = app.wrap(cors);
        let app = app
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default());

        app
    })
    .workers(server_config.workers.into())
    .bind(SocketAddrV4::new(server_config.ip, server_config.port))?
    .run()
    .await
}
