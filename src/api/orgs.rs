use crate::RecordsDbConn;

use rocket_contrib::json::Json;
use diesel::prelude::*;

use super::models::{ Org, Trailer, Bike };

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

fn get_org_structure(conn: &RecordsDbConn, org_id: i32) -> Option<OrgNode> {
    let result;
    {
        use super::schema::orgs::dsl::*;
        result = orgs
            .select(name)
            .find(org_id)
            .load::<String>(&**conn)
            .expect("Database failed")
            .pop()?;
    }

    Some(OrgNode {
        id: org_id,
        name: result,
        trailers: get_trailers(conn, org_id),
    })
}
fn get_trailers(conn: &RecordsDbConn, org_id: i32) -> Vec<TrailerNode> {
    use super::schema::trailers::dsl::*;

    let results = trailers
        .filter(org.eq(org_id))
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
