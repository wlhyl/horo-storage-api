use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(User::Name)
                            .string_len(64)
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(User::Password).string_len(64).not_null())
                    .col(ColumnDef::new(User::Salt).string_len(5).not_null())
                    .col(ColumnDef::new(User::CreateDate).date_time().not_null())
                    .col(ColumnDef::new(User::LastLoginDate).date_time())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Geo::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Geo::Id)
                            .unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Geo::Name).string_len(24).not_null())
                    .col(ColumnDef::new(Geo::East).boolean().not_null())
                    .col(ColumnDef::new(Geo::LongD).small_unsigned().not_null())
                    .col(ColumnDef::new(Geo::LongM).tiny_unsigned().not_null())
                    .col(ColumnDef::new(Geo::LongS).tiny_unsigned().not_null())
                    .col(ColumnDef::new(Geo::North).boolean().not_null())
                    .col(ColumnDef::new(Geo::LatD).tiny_unsigned().not_null())
                    .col(ColumnDef::new(Geo::LatM).tiny_unsigned().not_null())
                    .col(ColumnDef::new(Geo::LatS).tiny_unsigned().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Native::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Native::Id)
                            .unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Native::Name).string_len(4))
                    .col(ColumnDef::new(Native::Sex).boolean().not_null())
                    .col(ColumnDef::new(Native::Year).integer().not_null())
                    .col(ColumnDef::new(Native::Month).tiny_unsigned().not_null())
                    .col(ColumnDef::new(Native::Day).tiny_unsigned().not_null())
                    .col(ColumnDef::new(Native::Hour).tiny_unsigned().not_null())
                    .col(ColumnDef::new(Native::Minute).tiny_unsigned().not_null())
                    .col(ColumnDef::new(Native::Second).tiny_unsigned().not_null())
                    .col(ColumnDef::new(Native::TZ).double().not_null())
                    .col(ColumnDef::new(Native::ST).boolean().not_null())
                    .col(
                        ColumnDef::new(Native::GeoId)
                            .unsigned()
                            .unique_key()
                            .not_null(),
                    )
                    // .col(ColumnDef::new(Native::HouseId).unsigned())
                    // .col(ColumnDef::new(Native::IsNative).boolean().not_null())
                    // .col(ColumnDef::new(Native::AppId).unsigned().not_null())
                    .col(ColumnDef::new(Native::Describe).text())
                    .col(ColumnDef::new(Native::UserId).unsigned().not_null())
                    .col(ColumnDef::new(Native::CreateDate).date_time().not_null())
                    .col(ColumnDef::new(Native::LastUpdateDate).date_time())
                    .foreign_key(
                        ForeignKey::create()
                            .name("native_geo")
                            .from(Native::Table, Native::GeoId)
                            .to(Geo::Table, Geo::Id)
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("native_user")
                            .from(Native::Table, Native::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Native::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Geo::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Native {
    Table,
    Id,
    Name,
    Sex, //性别
    Year,
    Month,
    Day,
    Hour,
    Minute,
    Second,
    TZ,    //时区
    ST,    //夏令时
    GeoId, // 出生地
    // HouseId,  //宫位，null表示非占星数据，不当有house，由使用者提供
    Describe, // 说明文字
    UserId,
    CreateDate,
    LastUpdateDate,
}

#[derive(DeriveIden)]
enum Geo {
    Id,
    Table,
    Name,
    East,
    LongD, // 度
    LongM, // 分
    LongS, // 秒
    North,
    LatD,
    LatM,
    LatS,
}

#[derive(DeriveIden)]
pub enum User {
    Id,
    Table,
    Name,
    Password,
    Salt,
    CreateDate,
    LastLoginDate,
}
