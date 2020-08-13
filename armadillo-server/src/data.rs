use actix_web::{web, Error, HttpResponse, Responder, http};
use serde::{Deserialize, Serialize};
use armadillo_database::{DbPool, Db};
use armadillo_core::data::*;
use armadillo_core::{Id, Timestamp};

use crate::time;

#[derive(Deserialize)]
pub struct Info {
    from: Option<Timestamp>,
    until: Option<Timestamp>,
}

pub async fn get_csv_data_route<D>(
    pool: web::Data<DbPool>,
    id: web::Path<Id>,
    info: web::Query<Info>,
) -> Result<impl Responder, Error>
where
    D: 'static + TrailerData + Serialize + Send,
{
    get_data::<D>(pool, id, info, true).await
}

pub async fn get_json_data_route<D>(
    pool: web::Data<DbPool>,
    id: web::Path<Id>,
    info: web::Query<Info>,
) -> Result<impl Responder, Error>
where
    D: 'static + TrailerData + Serialize + Send,
{
    get_data::<D>(pool, id, info, false).await
}

// route for getting data
pub async fn get_data<D>(
    pool: web::Data<DbPool>,
    id: web::Path<Id>,
    info: web::Query<Info>,
    csv: bool,
) -> Result<impl Responder, Error>
where
    D: 'static + TrailerData + Serialize + Send,
{
    let id = id.into_inner();
    let conn = pool.get().expect("couldn't get db connection from pool");
    let from = info.from.unwrap_or(time::now()-60*5);
    let until = info.until.unwrap_or(time::now());

    let data = web::block(move || {
        let db = Db(&conn);
        D::find(&db, id, from, until)
    }).await.map_err(|e| {
        error!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    if data.len() == 0 {
        Ok(HttpResponse::NotFound().body(format!("no data with id: {} was found for {}", id, std::any::type_name::<D>())))
    } else {
        if csv {
            // return csv record
            let mut wtr = csv::Writer::from_writer(vec![]);
            for data_point in data {
                wtr.serialize(data_point).unwrap();
            }
            let data = String::from_utf8(wtr.into_inner().unwrap()).unwrap();

            Ok(HttpResponse::Ok()
                .content_type("test/csv")
                .set_header(http::header::CONTENT_DISPOSITION, "attachment;filename=trailer_data.csv")
                .body(data)
            )
        } else {
            // return json data
            Ok(HttpResponse::Ok().json(data))
        }
    }
}

// route for posting data
pub async fn post_data<D>(
    pool: web::Data<DbPool>,
    id: web::Path<Id>,
    data: web::Json<D::NewData>,
) -> Result<impl Responder, Error>
where
    D: 'static + TrailerData + Serialize + Send,
{
    let id = id.into_inner();
    let conn = pool.get().expect("couldn't get connection from pool");
    let data = data.into_inner();

    let updated_data = web::block(move || {
        let db = Db(&conn);
        D::insert(&db, id, time::now(), data)
    }).await.map_err(|e| {
        error!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    // no need to send data back
    Ok(HttpResponse::Ok().json(updated_data))
}

/*
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
*/
