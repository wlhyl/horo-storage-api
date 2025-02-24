use std::env;

use rand::{ distr::Alphanumeric, rng, Rng};
use sea_orm_migration::prelude::*;
use storage_api::utils::password_encoder;

use crate::m20220101_000001_create_table::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let name = env::var("USERNAME").map_err(|_error| {
            DbErr::Migration(
                "没设置 USERNAME 环境变量，可在.env文件中设置或export USERNAME=...".into(),
            )
        })?;

        let password = env::var("PASSWORD").map_err(|_error| {
            DbErr::Migration(
                "没设置 PASSWORD 环境变量，可在.env文件中设置或export PASSWORD=...".into(),
            )
        })?;

        let salt: String = rng()
            .sample_iter(Alphanumeric)
            .take(5)
            .map(char::from)
            .collect();

        let password = format!("{}{}", password, salt);
        let password = password_encoder::encode(&password)
            .map_err(|error| DbErr::Migration(error.to_string()))?;

        let now = chrono::Local::now().naive_local();
        let insert = Query::insert()
            .into_table(Users::Table)
            .columns([Users::Username, Users::PasswordHash, Users::Salt, Users::CreatedAt])
            .values_panic([name.into(), password.into(), salt.into(), now.into()])
            .to_owned();

        manager.exec_stmt(insert).await
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        Ok(())
    }
}
