use actix::Addr;
use crate::ws::{UpdateServer, Update};
use actix_web::{web, Responder, Error, HttpResponse};
use serde::Serialize;
use std::sync::Arc;

use crate::DbPool;
use super::db::DataQuery;

// route for getting data
pub async fn get_data<D> (
    pool: web::Data<DbPool>,
    id: web::Path<i32>,
) -> Result<impl Responder, Error> 
where D: 'static + DataQuery + Serialize + Send
{
    let id = id.into_inner();
    let conn = pool.get().expect("couldn't get db connection from pool");

    let data = web::block(move || D::find(&conn, id, 100))
        .await
        .map_err(|e| {
            error!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    if data.len() == 0 {
        Ok(HttpResponse::NotFound().body(format!("no data with id: {} was found", id)))
    } else {
        Ok(HttpResponse::Ok().json(data))
    }
}

// route for posting data
pub async fn post_data<D> (
    pool: web::Data<DbPool>,
    id: web::Path<i32>,
    data: web::Json<D::NewData>,
    update_server: web::Data<Addr<UpdateServer>>,
) -> Result<impl Responder, Error>
where D: 'static + DataQuery + Serialize + Send
{
    let id = id.into_inner();
    let conn = pool.get().expect("couldn't get connection from pool");
    let data = data.into_inner();

    let _updated_data = web::block(move || D::insert(&conn, id, data))
        .await
        .map_err(|e| {
            error!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    let conn = pool.get().expect("couldn't get connection from pool");

    let mut data = web::block(move || D::find(&conn, id, 1))
        .await
        .map_err(|e| {
            error!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    // send data to update server
    if data.len() != 0 {
        let data = Arc::new(data.pop().unwrap());
        update_server.do_send(Update(data));
    }

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