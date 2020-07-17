use diesel::prelude::*;
use diesel::PgConnection;

use super::models::{ Org, Trailer, Bike };

pub fn find_org_by_id(conn: &PgConnection, id: i32) -> Result<Option<Org>, diesel::result::Error> {
    use super::schema::orgs::dsl::*;
    let result = orgs
        .filter(org_id.eq(id))
        .first::<Org>(conn)
        .optional()?;

    Ok(result)
}

pub fn find_trailer_by_id(conn: &PgConnection, id: i32) -> Result<Option<Trailer>, diesel::result::Error> {
    use super::schema::trailers::dsl::*;
    let result = trailers
        .filter(trailer_id.eq(id))
        .first::<Trailer>(conn)
        .optional()?;

    Ok(result)
}

pub fn find_trailers_by_org_id(conn: &PgConnection, id: i32) -> Result<Vec<Trailer>, diesel::result::Error> {
    use super::schema::trailers::dsl::*;
    let results = trailers
        .filter(org.eq(id))
        .load::<Trailer>(conn);

    results
}

pub fn find_bike_by_id(conn: &PgConnection, id: i32) -> Result<Option<Bike>, diesel::result::Error> {
    use super::schema::bikes::dsl::*;
    let result = bikes
        .filter(bike_id.eq(id))
        .first::<Bike>(conn)
        .optional()?;

    Ok(result)
}

pub fn find_bikes_by_trailer_id(conn: &PgConnection, id: i32) -> Result<Vec<Bike>, diesel::result::Error> {
    use super::schema::bikes::dsl::*;
    let results = bikes
        .filter(trailer.eq(id))
        .load::<Bike>(conn);

    results
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