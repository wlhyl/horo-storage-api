use sea_orm_migration::prelude::*;

// #[async_std::main]
#[actix_web::main]
async fn main() {
    cli::run_cli(migration::Migrator).await;
}
