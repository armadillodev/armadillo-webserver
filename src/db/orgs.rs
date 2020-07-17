#![allow(dead_code)]

use diesel::prelude::*;
use diesel::PgConnection;
use diesel::result::Error;

use super::models::{ Org, Trailer, Bike };

pub fn find_orgs(conn: &PgConnection) -> Result<Vec<Org>, Error> {
    use super::schema::orgs::dsl::*;
    let results = orgs.load::<Org>(conn)?;

    Ok(results)
}

pub fn find_org_by_id(conn: &PgConnection, id: i32) -> Result<Option<Org>, Error> {
    use super::schema::orgs::dsl::*;
    let result = orgs
        .filter(org_id.eq(id))
        .first::<Org>(conn)
        .optional()?;

    Ok(result)
}

pub fn find_trailer_by_id(conn: &PgConnection, id: i32) -> Result<Option<Trailer>, Error> {
    use super::schema::trailers::dsl::*;
    let result = trailers
        .filter(trailer_id.eq(id))
        .first::<Trailer>(conn)
        .optional()?;

    Ok(result)
}

pub fn find_trailers_by_org_id(conn: &PgConnection, id: i32) -> Result<Vec<Trailer>, Error> {
    use super::schema::trailers::dsl::*;
    let results = trailers
        .filter(org.eq(id))
        .load::<Trailer>(conn);

    results
}

pub fn find_bike_by_id(conn: &PgConnection, id: i32) -> Result<Option<Bike>, Error> {
    use super::schema::bikes::dsl::*;
    let result = bikes
        .filter(bike_id.eq(id))
        .first::<Bike>(conn)
        .optional()?;

    Ok(result)
}

pub fn find_bikes_by_trailer_id(conn: &PgConnection, id: i32) -> Result<Vec<Bike>, Error> {
    use super::schema::bikes::dsl::*;
    let results = bikes
        .filter(trailer.eq(id))
        .load::<Bike>(conn);

    results
}