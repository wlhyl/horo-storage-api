use chrono::Local;
use log::debug;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, ModelTrait, PaginatorTrait,
    QueryFilter, TransactionTrait,
};

use crate::{
    error::Error,
    request::{HoroscopeRequest, PageQueryparams, UpdateHoroscopeRequest},
    response::{Horoscope, PageResponser},
};

pub async fn get_pages(
    db: &sea_orm::DatabaseConnection,
    user_id: u32,
    params: &PageQueryparams,
) -> Result<PageResponser<Vec<Horoscope>>, Error> {
    let horoscope_location_pages = entity::horoscopes::Entity::find()
        .find_also_related(entity::locations::Entity)
        .filter(entity::horoscopes::Column::UserId.eq(user_id))
        .paginate(db, params.size);

    let total_pages = horoscope_location_pages.num_pages().await?;
    let horoscopes: Result<Vec<Horoscope>, Error> = horoscope_location_pages
        .fetch_page(params.page)
        .await?
        .into_iter()
        .map(|(horoscope, location)| {
            location.ok_or_else(|| {
                debug!(
                    "数据库中horoscopes表与locations表的一对一约束异常，horoscope id={}的记录没有对应的location",
                    horoscope.id
                );
                Error::Internal(
                    "数据异常：horoscope记录没有对应location记录，查看debug日志了解详情！".into(),
                )
            })
            .map(|location| Horoscope::new(horoscope, location))
        })
        .collect();

    Ok(PageResponser {
        data: horoscopes?,
        total: total_pages,
    })
}

pub async fn get_by_id(
    db: &sea_orm::DatabaseConnection,
    id: u32,
    user_id: u32,
) -> Result<Horoscope, Error> {
    let (horoscope, location) = entity::horoscopes::Entity::find_by_id(id)
        .find_also_related(entity::locations::Entity)
        .filter(entity::horoscopes::Column::UserId.eq(user_id))
        .one(db)
        .await?
        .ok_or_else(|| Error::NotFound(format!("未找到ID为{}的记录", id)))?;

    let location = location.ok_or_else(|| {
        debug!(
            "数据完整性错误: horoscope(id={})缺少对应的location记录",
            horoscope.id
        );
        Error::Internal("数据完整性错误".to_string())
    })?;

    Ok(Horoscope::new(horoscope, location))
}

pub async fn update(
    db: &sea_orm::DatabaseConnection,
    id: u32,
    user_id: u32,
    req: &UpdateHoroscopeRequest,
) -> Result<(), Error> {
    let txn = db.begin().await?;

    let (horoscope, location) = entity::horoscopes::Entity::find_by_id(id)
        .find_also_related(entity::locations::Entity)
        .filter(entity::horoscopes::Column::UserId.eq(user_id))
        .one(&txn)
        .await?
        .ok_or_else(|| Error::BadRequest("horoscope不存在".into()))?;

    let location = location
        .ok_or_else(|| Error::Internal("数据异常：horoscope记录没有对应location记录".into()))?;

    // 更新location字段
    if let Some(location_update) = req.location.as_ref() {
        let mut location: entity::locations::ActiveModel = location.into();

        // 直接使用LocationRequest的字段进行更新
        location.name = ActiveValue::Set(location_update.name.clone());
        location.is_east = ActiveValue::Set(location_update.is_east);
        location.longitude_degree = ActiveValue::Set(location_update.longitude_degree);
        location.longitude_minute = ActiveValue::Set(location_update.longitude_minute);
        location.longitude_second = ActiveValue::Set(location_update.longitude_second);
        location.is_north = ActiveValue::Set(location_update.is_north);
        location.latitude_degree = ActiveValue::Set(location_update.latitude_degree);
        location.latitude_minute = ActiveValue::Set(location_update.latitude_minute);
        location.latitude_second = ActiveValue::Set(location_update.latitude_second);

        location.update(&txn).await?;
    }

    // 更新horoscope字段
    let mut horoscope: entity::horoscopes::ActiveModel = horoscope.into();

    if let Some(name) = &req.name {
        horoscope.name = ActiveValue::Set(name.clone());
    }
    if let Some(gender) = req.gender {
        horoscope.gender = ActiveValue::Set(gender);
    }
    if let Some(birth_year) = req.birth_year {
        horoscope.birth_year = ActiveValue::Set(birth_year);
    }
    if let Some(birth_month) = req.birth_month {
        horoscope.birth_month = ActiveValue::Set(birth_month);
    }
    if let Some(birth_day) = req.birth_day {
        horoscope.birth_day = ActiveValue::Set(birth_day);
    }
    if let Some(birth_hour) = req.birth_hour {
        horoscope.birth_hour = ActiveValue::Set(birth_hour);
    }
    if let Some(birth_minute) = req.birth_minute {
        horoscope.birth_minute = ActiveValue::Set(birth_minute);
    }
    if let Some(birth_second) = req.birth_second {
        horoscope.birth_second = ActiveValue::Set(birth_second);
    }
    if let Some(time_zone_offset) = req.time_zone_offset {
        horoscope.time_zone_offset = ActiveValue::Set(time_zone_offset);
    }
    if let Some(is_dst) = req.is_dst {
        horoscope.is_dst = ActiveValue::Set(is_dst);
    }
    if let Some(description) = &req.description {
        horoscope.description = ActiveValue::Set(description.clone());
    }

    horoscope.updated_at = ActiveValue::Set(Some(Local::now().naive_local()));
    horoscope.update(&txn).await?;

    txn.commit().await?;
    Ok(())
}

pub async fn delete(db: &sea_orm::DatabaseConnection, id: u32, user_id: u32) -> Result<(), Error> {
    let txn = db.begin().await?;

    let horoscope = entity::horoscopes::Entity::find_by_id(id)
        .filter(entity::horoscopes::Column::UserId.eq(user_id))
        .one(&txn)
        .await?
        .ok_or_else(|| Error::BadRequest("horoscope不存在".into()))?;

    let location_id = horoscope.location_id;
    horoscope.delete(&txn).await?;

    entity::locations::Entity::delete_by_id(location_id)
        .exec(&txn)
        .await?;

    txn.commit().await?;
    Ok(())
}

pub async fn insert(
    db: &sea_orm::DatabaseConnection,
    req: HoroscopeRequest,
    user_id: u32,
) -> Result<Horoscope, Error> {
    let txn = db.begin().await?;

    let location = entity::locations::ActiveModel {
        name: ActiveValue::Set(req.location.name),
        is_east: ActiveValue::Set(req.location.is_east),
        longitude_degree: ActiveValue::Set(req.location.longitude_degree),
        longitude_minute: ActiveValue::Set(req.location.longitude_minute),
        longitude_second: ActiveValue::Set(req.location.longitude_second),
        is_north: ActiveValue::Set(req.location.is_north),
        latitude_degree: ActiveValue::Set(req.location.latitude_degree),
        latitude_minute: ActiveValue::Set(req.location.latitude_minute),
        latitude_second: ActiveValue::Set(req.location.latitude_second),
        ..Default::default()
    }
    .insert(&txn)
    .await?;

    let horoscope = entity::horoscopes::ActiveModel {
        name: ActiveValue::Set(req.name),
        gender: ActiveValue::Set(req.gender),
        birth_year: ActiveValue::Set(req.birth_year),
        birth_month: ActiveValue::Set(req.birth_month),
        birth_day: ActiveValue::Set(req.birth_day),
        birth_hour: ActiveValue::Set(req.birth_hour),
        birth_minute: ActiveValue::Set(req.birth_minute),
        birth_second: ActiveValue::Set(req.birth_second),
        time_zone_offset: ActiveValue::Set(req.time_zone_offset),
        is_dst: ActiveValue::Set(req.is_dst),
        location_id: ActiveValue::Set(location.id),
        description: ActiveValue::Set(req.description),
        user_id: ActiveValue::Set(user_id),
        created_at: ActiveValue::Set(Local::now().naive_local()),
        ..Default::default()
    }
    .insert(&txn)
    .await?;

    txn.commit().await?;
    let response = Horoscope::new(horoscope, location);
    Ok(response)
}
