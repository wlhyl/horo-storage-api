use sea_orm::DatabaseConnection;

pub struct AppState {
    // pub health_check_response: String,
    pub db: DatabaseConnection,
    // pub jwt_secret: String,
    pub token_expire_seconds: u64,
}
