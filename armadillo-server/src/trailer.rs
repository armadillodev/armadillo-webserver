use diesel::PgConnection;
use serde::Serialize;

use crate::db::{Bike, DbEntity, Oven, SolarMicrogrid, Trailer};
use crate::DbPool;
use actix_web::{web, Error, Responder, HttpResponse};

type Id = i32;

#[derive(Serialize)]
struct TrailerNode {
    name: String,
    location: String,
    bikes: Vec<Id>,
    ovens: Vec<Id>,
    microgrids: Vec<Id>,
}

impl TrailerNode {
    fn fetch(conn: &PgConnection, trailer_id: Id) -> Result<Option<Self>, diesel::result::Error> {
        let trailer = Trailer::by_id(conn, trailer_id)?;

        // trailer
        if trailer.is_none() {
            return Ok(None);
        }

        let trailer = trailer.unwrap();

        let trailer_node = TrailerNode {
            name: trailer.name,
            location: trailer.location,
            bikes: Bike::by_parent_id(conn, trailer_id)?
                .iter()
                .map(|bike| bike.id)
                .collect(),
            ovens: Oven::by_parent_id(conn, trailer_id)?
                .iter()
                .map(|oven| oven.id)
                .collect(),
            microgrids: SolarMicrogrid::by_parent_id(conn, trailer_id)?
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

    let trailer_node = web::block(move || TrailerNode::fetch(&conn, trailer_id))
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

    let trailers = web::block(move || Trailer::all(&conn)).await.map_err(|e| {
        error!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    if trailers.len() == 0 {
        return Ok(HttpResponse::NotFound().finish());
    }

    Ok(HttpResponse::Ok().json(trailers))
}
