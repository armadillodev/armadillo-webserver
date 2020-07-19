use actix::Addr;
use crate::ws::{BikeServer, Update};
use crate::db::models::BikeData;
use actix_web::{web, Responder, Error, HttpResponse};

use crate::DbPool;
use super::db;

// routes for getting bike data
pub async fn get_bike_data(pool: web::Data<DbPool>, bike_id: web::Path<i32>) -> Result<impl Responder, Error> {
    let bike_id = bike_id.into_inner();
    let conn = pool.get().expect("couldn't get db connection from pool");

    let bikes = web::block(move || db::data::find_bike_data(&conn, bike_id, 100))
        .await
        .map_err(|e| {
            error!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    if bikes.len() == 0 {
        Ok(HttpResponse::NotFound().body(format!("no bike data with id: {} was found", bike_id)))
    } else {
        Ok(HttpResponse::Ok().json(bikes))
    }
}

pub async fn get_latest_bike_data(pool: web::Data<DbPool>, bike_id: web::Path<i32>) -> Result<impl Responder, Error> {
    let bike_id = bike_id.into_inner();
    let conn = pool.get().expect("couldn't get db connection from pool");

    let mut bike_data = web::block(move || db::data::find_bike_data(&conn, bike_id, 1))
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

// route for posting bike data
pub async fn add_bike_data(
    pool: web::Data<DbPool>,
    bike_id: web::Path<i32>,
    data: web::Json<db::data::CreateBikeData>,
    bike_update_server: web::Data<Addr<BikeServer>>,
) -> Result<impl Responder, Error> {
    let bike_id = bike_id.into_inner();
    let conn = pool.get().expect("couldn't get connection from pool");
    let data = data.into_inner();

    let updated_data = web::block(move || db::data::insert_new_bike_data(&conn, bike_id, data))
        .await
        .map_err(|e| {
            error!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    let conn = pool.get().expect("couldn't get connection from pool");

    let mut bike_data = web::block(move || db::data::find_bike_data(&conn, bike_id, 1))
        .await
        .map_err(|e| {
            error!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    if bike_data.len() != 0 {
        bike_update_server.do_send(Update(bike_data.pop().unwrap()));
    }

    // no need to send data back
    Ok(HttpResponse::Ok().finish())
}