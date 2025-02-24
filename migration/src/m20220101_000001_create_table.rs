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
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Users::Username)
                            .string_len(64)
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(Users::PasswordHash)
                            .string_len(64)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Users::Salt).string_len(5).not_null())
                    .col(ColumnDef::new(Users::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Users::LastLoginAt).date_time())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Locations::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Locations::Id)
                            .unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Locations::Name).string_len(30).not_null())
                    .col(ColumnDef::new(Locations::IsEast).boolean().not_null())
                    .col(
                        ColumnDef::new(Locations::LongitudeDegree)
                            .small_unsigned()
                            .not_null()
                            .check(Expr::col(Locations::LongitudeDegree).lte(180)),
                    )
                    .col(
                        ColumnDef::new(Locations::LongitudeMinute)
                            .tiny_unsigned()
                            .not_null()
                            .check(Expr::col(Locations::LongitudeMinute).lte(59)),
                    )
                    .col(
                        ColumnDef::new(Locations::LongitudeSecond)
                            .tiny_unsigned()
                            .not_null()
                            .check(Expr::col(Locations::LongitudeSecond).lte(59)),
                    )
                    .col(ColumnDef::new(Locations::IsNorth).boolean().not_null())
                    .col(
                        ColumnDef::new(Locations::LatitudeDegree)
                            .tiny_unsigned()
                            .not_null()
                            .check(Expr::col(Locations::LatitudeDegree).lte(90)),
                    )
                    .col(
                        ColumnDef::new(Locations::LatitudeMinute)
                            .tiny_unsigned()
                            .not_null()
                            .check(Expr::col(Locations::LatitudeMinute).lte(59)),
                    )
                    .col(
                        ColumnDef::new(Locations::LatitudeSecond)
                            .tiny_unsigned()
                            .not_null()
                            .check(Expr::col(Locations::LatitudeSecond).lte(59)),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Horoscopes::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Horoscopes::Id)
                            .unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Horoscopes::Name).string_len(30).not_null())
                    .col(ColumnDef::new(Horoscopes::Gender).boolean().not_null())
                    .col(ColumnDef::new(Horoscopes::BirthYear).integer().not_null())
                    .col(
                        ColumnDef::new(Horoscopes::BirthMonth)
                            .tiny_unsigned()
                            .not_null()
                            .check(
                                Expr::col(Horoscopes::BirthMonth)
                                    .gte(1)
                                    .and(Expr::col(Horoscopes::BirthMonth).lte(12)),
                            ),
                    )
                    .col(
                        ColumnDef::new(Horoscopes::BirthDay)
                            .tiny_unsigned()
                            .not_null()
                            .check(
                                Expr::col(Horoscopes::BirthDay)
                                    .gte(1)
                                    .and(Expr::col(Horoscopes::BirthDay).lte(31)),
                            ),
                    )
                    .col(
                        ColumnDef::new(Horoscopes::BirthHour)
                            .tiny_unsigned()
                            .not_null()
                            .check(Expr::col(Horoscopes::BirthHour).lte(23)),
                    )
                    .col(
                        ColumnDef::new(Horoscopes::BirthMinute)
                            .tiny_unsigned()
                            .not_null()
                            .check(Expr::col(Horoscopes::BirthMinute).lte(59)),
                    )
                    .col(
                        ColumnDef::new(Horoscopes::BirthSecond)
                            .tiny_unsigned()
                            .not_null()
                            .check(Expr::col(Horoscopes::BirthSecond).lte(59)),
                    )
                    .col(
                        ColumnDef::new(Horoscopes::TimeZoneOffset)
                            .double()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Horoscopes::IsDst).boolean().not_null())
                    .col(
                        ColumnDef::new(Horoscopes::LocationId)
                            .unsigned()
                            .unique_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Horoscopes::Description).text().not_null())
                    .col(ColumnDef::new(Horoscopes::UserId).unsigned().not_null())
                    .col(ColumnDef::new(Horoscopes::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Horoscopes::UpdatedAt).date_time())
                    .foreign_key(
                        ForeignKey::create()
                            .name("native_geo")
                            .from(Horoscopes::Table, Horoscopes::LocationId)
                            .to(Locations::Table, Locations::Id)
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("native_user")
                            .from(Horoscopes::Table, Horoscopes::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Restrict),
                    )
                    .index(
                        Index::create()
                            .name("idx_native_user")
                            .col(Horoscopes::UserId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Horoscopes::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Locations::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Horoscopes {
    Table,
    Id,
    Name,
    Gender, //性别
    BirthYear,
    BirthMonth,
    BirthDay,
    BirthHour,
    BirthMinute,
    BirthSecond,
    TimeZoneOffset, //时区
    IsDst,          //夏令时
    LocationId,     // 出生地
    // HouseId,  //宫位，null表示非占星数据，不当有house，由使用者提供
    Description, // 说明文字
    UserId,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Locations {
    Id,
    Table,
    Name,
    IsEast,
    LongitudeDegree, // 度
    LongitudeMinute, // 分
    LongitudeSecond, // 秒
    IsNorth,
    LatitudeDegree,
    LatitudeMinute,
    LatitudeSecond,
}

#[derive(DeriveIden)]
pub enum Users {
    Id,
    Table,
    Username,
    PasswordHash,
    Salt,
    CreatedAt,
    LastLoginAt,
}
