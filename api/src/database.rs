use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::time::Duration;

pub async fn setup_database(database_url: &str) -> Result<DatabaseConnection, sea_orm::DbErr> {
    let mut opt = ConnectOptions::new(database_url.to_owned());
    opt.max_connections(100)
        // .min_connections(5)
        .min_connections(1)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .sqlx_logging(false);

    Database::connect(opt).await
}
