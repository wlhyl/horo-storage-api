use std::{env, net::SocketAddrV4, time::Duration};

use actix_web::{middleware, web, App, HttpServer};
use actix_web_validator::JsonConfig;
use clap::Parser;

use sea_orm::{ConnectOptions, Database};
use storage_api::{
    args,
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let log4rs_config = env::var("LOG4RS_CONFIG")
        .expect("没设置 LOG4RS_CONFIG 环境变量，可在.env文件中设置或export LOG4RS_CONFIG=...");

    log4rs::init_file(&log4rs_config, Default::default())
        .map_err(|error| format!("配置日志系统失败，日志配置文件：{log4rs_config}, {error}"))
        .unwrap();

    let database_url = env::var("DATABASE_URL")
        .expect("没设置 DATABASE_URL 环境变量，可在.env文件中设置或export DATABASE_URL=...");

    let mut opt = ConnectOptions::new(database_url);
    opt.max_connections(100)
        // .min_connections(5)
        .min_connections(1)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .sqlx_logging(false);

    let db = Database::connect(opt).await.expect("连接到数据库失败");

    // let jwt_secret = env::var("JWT_SECRET")
    //     .expect("没设置 JWT_SECRET 环境变量，可在.env文件中设置或export JWT_SECRET=...");

    let token_expire_seconds = env::var("TOKEN_EXPIRE_SECONDS").expect(
        "没设置 TOKEN_EXPIRE_SECONDS 环境变量，可在.env文件中设置或export TOKEN_EXPIRE_SECONDS=...",
    );

    let token_expire_seconds: u64 = token_expire_seconds
        .parse()
        .expect("TOKEN_EXPIRE_SECONDS的值必需是正整数");

    let shared_data = web::Data::new(AppState {
        db,
        // jwt_secret,
        token_expire_seconds,
    });

    let args = args::Args::parse();

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
    .workers(args.n)
    .bind(SocketAddrV4::new(args.ip, args.port))?
    .run()
    .await
}
