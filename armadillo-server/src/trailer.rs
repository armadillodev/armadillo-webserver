use diesel::PgConnection;
use diesel::result::Error as DieselError;
use serde::Serialize;
use actix_web::{web, Error, Responder, HttpResponse};

use crate::db::{TrailerEntity, Db, Id};
use crate::db::{Bike, Oven, Solar, Trailer};
use crate::DbPool;


#[derive(Serialize)]
struct TrailerNode {
    name: String,
    location: String,
    bikes: Vec<i32>,
    ovens: Vec<i32>,
    microgrids: Vec<i32>,
}

impl TrailerNode {
    fn fetch(db: Db, trailer_id: Id) -> Result<Option<Self>, DieselError>{
        let trailer = Trailer::id(&db, trailer_id)?;

        if trailer.is_none() { return Ok(None); }

        let trailer = trailer.unwrap();

        let trailer_node = TrailerNode {
            name: trailer.name,
            location: trailer.location,
            bikes: Bike::trailer_id(&db, trailer_id)?
                .iter()
                .map(|bike| bike.id)
                .collect(),
            ovens: Oven::trailer_id(&db, trailer_id)?
                .iter()
                .map(|oven| oven.id)
                .collect(),
            microgrids: Solar::trailer_id(&db, trailer_id)?
                .iter()
                .map(|microgrid| microgrid.id)
                .collect(),
        };

        Ok(Some(trailer_node))
    }
}

pub async fn get_trailer_node(pool: web::Data<DbPool>, trailer_id: web::Path<Id>)
    -> Result<impl Responder, Error> {
    let trailer_id = trailer_id.into_inner();
    let conn = pool.get().expect("couldn't get connection from pool");

    let trailer_node = web::block(move || {
        let db = Db(&conn);
        TrailerNode::fetch(db, trailer_id)
    })
        .await
        .map_err(|e| {
            error!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    if trailer_node.is_none() {
        return Ok(HttpResponse::NotFound().body(format!("no trailer with id: {}", trailer_id)));
    }

    let trailer_node = trailer_node.unwrap();
    Ok(HttpResponse::Ok().json(trailer_node))
}

pub async fn get_trailer_list(pool: web::Data<DbPool>) -> Result<impl Responder, Error> {
    let conn = pool.get().expect("couldn't get connection from pool");

    let trailers = web::block(move || {
        let db = Db(&conn);
        Trailer::all(&db)
    }).await.map_err(|e| {
        error!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    if trailers.len() == 0 {
        return Ok(HttpResponse::NotFound().finish());
    }

    Ok(HttpResponse::Ok().json(trailers))
}
