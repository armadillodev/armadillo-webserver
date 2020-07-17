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

/*
#[derive(Serialize, Deserialize)]
pub struct OrgNode {
    id: i32,
    name: String,
    trailers: Vec<TrailerNode>,
}
#[derive(Serialize, Deserialize)]
pub struct TrailerNode {
    id: i32,
    name: String,
    location: String,
    bikes: Vec<BikeNode>,
}
#[derive(Serialize, Deserialize)]
pub struct BikeNode {
    id: i32,
}


fn get_org_structure(conn: &RecordsDbConn, org_id_value: i32) -> Option<OrgNode> {
    let result;
    {
        use super::schema::orgs::dsl::*;
        result = orgs
            .select(name)
            .find(org_id_value)
            .load::<String>(&**conn)
            .expect("Database failed")
            .pop()?;
    }

    Some(OrgNode {
        id: org_id_value,
        name: result,
        trailers: get_trailers(conn, org_id_value),
    })
}
fn get_trailers(conn: &RecordsDbConn, org_id_value: i32) -> Vec<TrailerNode> {
    use super::schema::trailers::dsl::*;

    let results = trailers
        .filter(org.eq(org_id_value))
        .load::<Trailer>(&**conn)
        .expect("Database failed");

    results.into_iter().map(|trailer| {
        TrailerNode {
            id: trailer.id,
            name: trailer.name,
            location: trailer.location,
            bikes: get_bikes(conn, trailer.id),
        }
    }).collect()
}
fn get_bikes(conn: &RecordsDbConn, trailer_id: i32) -> Vec<BikeNode> {
    use super::schema::bikes::dsl::*;

    let results = bikes
        .filter(trailer.eq(trailer_id))
        .load::<Bike>(&**conn)
        .expect("Database failed");

    results.iter().map(|bike| {
        BikeNode {
            id: bike.id,
        }
    }).collect()
}

fn get_org_list(conn: RecordsDbConn) -> Vec<Org> {
    use super::schema::orgs::dsl::*;

    let results = orgs
        .load::<Org>(&*conn)
        .expect("Database failed");

    results
}

#[get("/")]
pub fn org_list_view(conn: RecordsDbConn) -> Json<Vec<Org>> {
    Json(get_org_list(conn))
}

#[get("/<id>")]
pub fn org_structure_view(conn: RecordsDbConn, id: i32) -> Option<Json<OrgNode>> {
    let structure = get_org_structure(&conn, id)?;

    Some(Json(structure))
}

*/