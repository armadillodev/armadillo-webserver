use diesel::PgConnection;
use actix_web::{web, Error, HttpResponse, Responder};
use serde::Serialize;
use crate::DbPool;
use super::db;

#[derive(Serialize)]
struct OrgStructure {
    org: db::models::Org,
    trailers: Vec<db::models::Trailer>,
    bikes: Vec<db::models::Bike>,
}

impl OrgStructure {
    fn new(conn: &PgConnection, org_id: i32) -> Result<Option<Self>, diesel::result::Error> {
        let org = match db::orgs::find_org_by_id(conn, org_id)? {
            Some(org) => org,
            None => return Ok(None),
        };
        let trailers = db::orgs::find_trailers_by_org_id(conn, org_id)?;
        let bikes = trailers.iter()
            .map(|trailer| db::orgs::find_bikes_by_trailer_id(conn, trailer.id))
            .flatten()
            .flatten()
            .collect::<Vec<db::models::Bike>>();

        Ok(Some(OrgStructure { org, trailers, bikes }))
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