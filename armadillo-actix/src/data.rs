use actix_web::{web, Error, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use super::db::DataQuery;
use crate::DbPool;

#[derive(Deserialize)]
pub struct Info {
    count: Option<i32>,
}

// route for getting data
pub async fn get_data<D>(
    pool: web::Data<DbPool>,
    id: web::Path<i32>,
    info: web::Query<Info>,
) -> Result<impl Responder, Error>
where
    D: 'static + DataQuery + Serialize + Send,
{
    let id = id.into_inner();
    let conn = pool.get().expect("couldn't get db connection from pool");
    let count = info.count.unwrap_or(100);

    let data = web::block(move || D::find(&conn, id, count)).await.map_err(|e| {
        error!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    if data.len() == 0 {
        Ok(HttpResponse::NotFound().body(format!("no data with id: {} was found for {}", id, std::any::type_name::<D>())))
    } else {
        Ok(HttpResponse::Ok().json(data))
    }
}

// route for posting data
pub async fn post_data<D>(
    pool: web::Data<DbPool>,
    id: web::Path<i32>,
    data: web::Json<D::NewData>,
) -> Result<impl Responder, Error>
where
    D: 'static + DataQuery + Serialize + Send,
{
    let id = id.into_inner();
    let conn = pool.get().expect("couldn't get connection from pool");
    let data = data.into_inner();

    let _updated_data = web::block(move || D::insert(&conn, id, data)).await.map_err(|e| {
        error!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    // no need to send data back
    Ok(HttpResponse::Ok().finish())
}

pub async fn get_latest_bike_data(pool: web::Data<DbPool>, bike_id: web::Path<i32>) -> Result<impl Responder, Error> {
    let bike_id = bike_id.into_inner();
    let conn = pool.get().expect("couldn't get db connection from pool");

    let mut bike_data = web::block(move || crate::db::BikeData::find(&conn, bike_id, 1))
        .await
        .map_err(|e| {
            error!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    if bike_data.len() == 0 {
        return Ok(HttpResponse::NotFound().body(format!("no bike data with id: {} was found", bike_id)));
    }

    Ok(HttpResponse::Ok().json(bike_data.pop().unwrap()))
}
