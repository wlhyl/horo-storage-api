use dotenvy::dotenv;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub database_url: String,
    pub log4rs_config: String,
    pub token_expire_seconds: u64,
}

impl Config {
    pub fn from_env() -> Result<Self, dotenvy::Error> {
        dotenv().ok();
        Ok(Config {
            database_url: std::env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set"),
            log4rs_config: std::env::var("LOG4RS_CONFIG")
                .expect("LOG4RS_CONFIG must be set"),
            token_expire_seconds: std::env::var("TOKEN_EXPIRE_SECONDS")
                .expect("TOKEN_EXPIRE_SECONDS must be set")
                .parse()
                .expect("TOKEN_EXPIRE_SECONDS must be a positive integer"),
        })
    }
}