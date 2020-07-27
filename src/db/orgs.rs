#![allow(dead_code)]

use diesel::prelude::*;
use diesel::PgConnection;
use diesel::result::Error;

use super::models::{ Org, Trailer, Bike };

pub trait DbEntity: Sized {
    fn by_id(conn: &PgConnection, id: i32) -> Result<Option<Self>, Error>;

    // whatever is the owner of this struct.
    // For Org there is nothing, but for a bike it would be the trailer
    fn by_parent_id(_conn: &PgConnection, _id: i32) -> Result<Vec<Self>, Error> { Ok(Vec::new()) }
    fn all(_conn: &PgConnection) -> Result<Vec<Self>, Error> { Ok(Vec::new()) }
}

impl DbEntity for Org {
    fn by_id(conn: &PgConnection, id: i32) -> Result<Option<Self>, Error> {
        use super::schema::orgs::dsl::*;
        let result = orgs
            .filter(org_id.eq(id))
            .first::<Org>(conn)
            .optional()?;
    
        Ok(result)
    }

    fn all(conn: &PgConnection) -> Result<Vec<Self>, Error> {
        use super::schema::orgs::dsl::*;
        let results = orgs.load::<Org>(conn)?;
    
        Ok(results)    
    }

}

impl DbEntity for Trailer {
    fn by_id(conn: &PgConnection, id: i32) -> Result<Option<Self>, Error> {
        use super::schema::trailers::dsl::*;
        let result = trailers
            .filter(trailer_id.eq(id))
            .first::<Trailer>(conn)
            .optional()?;
    
        Ok(result)
    }

    fn by_parent_id(conn: &PgConnection, id: i32) -> Result<Vec<Self>, Error> {
        use super::schema::trailers::dsl::*;
        let results = trailers
            .filter(org.eq(id))
            .load::<Trailer>(conn);
    
        results    
    }
}

impl DbEntity for Bike {
    fn by_id(conn: &PgConnection, id: i32) -> Result<Option<Self>, Error> {
        use super::schema::bikes::dsl::*;
        let result = bikes
            .filter(bike_id.eq(id))
            .first::<Bike>(conn)
            .optional()?;
    
        Ok(result)
    }

    fn by_parent_id(conn: &PgConnection, id: i32) -> Result<Vec<Self>, Error> {
        use super::schema::bikes::dsl::*;
        let results = bikes
            .filter(trailer.eq(id))
            .load::<Bike>(conn);
    
        results
    }
}

pub fn find_org_id_by_bike_id(conn: &PgConnection, id: i32) -> Result<Option<i32>, Error> {
    todo!("This may be used for authentication");
    use super::schema::bikes::dsl::*;
    use super::schema::trailers::dsl::*;
    let result = bikes
        .filter(bike_id.eq(id))
        .inner_join(trailers)
        .select(org)
        .first::<i32>(conn)
        .optional()?;

    Ok(result)
}