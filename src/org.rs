use diesel::PgConnection;
use actix_web::{web, Error, HttpResponse, Responder};
use serde::Serialize;
use crate::DbPool;
use super::db::{self, DbEntity, Org, Bike, Trailer};

#[derive(Serialize)]
struct OrgStructure {
    id: i32,
    name: String,
    trailers: Vec<TrailerNode>,
}

#[derive(Serialize)]
pub struct TrailerNode {
    id: i32,
    name: String,
    location: String,
    bikes: Vec<BikeNode>,
    ovens: Vec<OvenNode>,
    microgrids: Vec<MicrogridNode>,
}
#[derive(Serialize)]
pub struct BikeNode {
    id: i32,
}

#[derive(Serialize)]
pub struct OvenNode {
    id: i32,
}

#[derive(Serialize)]
pub struct MicrogridNode {
    id: i32,
}

impl OrgStructure {
    fn new(conn: &PgConnection, org_id: i32) -> Result<Option<Self>, diesel::result::Error> {
        let org = match Org::by_id(conn, org_id)? {
            Some(org) => org,
            None => return Ok(None),
        };
        let trailers = Trailer::by_parent_id(conn, org_id)?;
        let trailers = trailers.into_iter()
            .map(|trailer| Ok(TrailerNode {
                id: trailer.id,
                name: trailer.name,
                location: trailer.location,
                bikes: Bike::by_parent_id(conn, trailer.id)?
                    .iter()
                    .map(|bike| BikeNode { id: bike.id })
                    .collect::<Vec<_>>(),
                ovens: Vec::new(),
                microgrids: Vec::new(),
            }))
            .collect::<Result<Vec<_>, diesel::result::Error>>()?;

        Ok(Some(OrgStructure {
            id: org_id,
            name: org.name,
            trailers: trailers,
        }))
    }
}

// get the structure of the organization
pub async fn get_org_structure(
    pool: web::Data<DbPool>,
    org_id: web::Path<i32>,
) -> Result<impl Responder, Error> {
    let org_id = org_id.into_inner();
    let conn = pool.get().expect("couldn't get connection from pool");

    let org = web::block(move || OrgStructure::new(&conn, org_id))
        .await
        .map_err(|e| {
            error!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    match org {
        Some(org) => Ok(HttpResponse::Ok().json(org)),
        None => Ok(HttpResponse::NotFound().body(format!("no org with id: {} was found", org_id))),
    }
}

// get a list of orgs
pub async fn get_org_list(
    pool: web::Data<DbPool>,
) -> Result<impl Responder, Error> {
    let conn = pool.get().expect("couldn't get connection from pool");

    let orgs = web::block(move || Org::all(&conn))
        .await
        .map_err(|e| {
            error!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    if orgs.len() == 0 {
        return Ok(HttpResponse::NotFound().finish());
    }

    Ok(HttpResponse::Ok().json(orgs))
}

// get org id of a bike for authentication
pub async fn get_org_id_for_bike(
    pool: web::Data<DbPool>,
    bike_id: web::Path<i32>,
) -> Result<impl Responder, Error> {
    unimplemented!("This is not used");
    let bike_id = bike_id.into_inner();
    let conn = pool.get().expect("couldn't get connection from pool");

    let org_id = web::block(move || db::orgs::find_org_id_by_bike_id(&conn, bike_id))
        .await
        .map_err(|e| {
            error!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(match org_id {
        None => HttpResponse::NotFound().finish(),
        Some(org_id) => HttpResponse::Ok().json(org_id)
    })
}