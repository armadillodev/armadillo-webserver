#![allow(dead_code)]

use diesel::prelude::*;
use diesel::result::Error;
use diesel::PgConnection;

use super::models::{Bike, Oven, Solar, Trailer};

pub trait DbEntity: Sized {
    fn by_id(conn: &PgConnection, id: i32) -> Result<Option<Self>, Error>;

    // whatever is the owner of this struct.
    // For Org there is nothing, but for a bike it would be the trailer
    fn by_parent_id(_conn: &PgConnection, _id: i32) -> Result<Vec<Self>, Error> {
        Ok(Vec::new())
    }
    fn all(_conn: &PgConnection) -> Result<Vec<Self>, Error> {
        Ok(Vec::new())
    }
}

impl DbEntity for Trailer {
    fn by_id(conn: &PgConnection, id: i32) -> Result<Option<Self>, Error> {
        use super::schema::trailers::dsl::*;
        let result = trailers.filter(trailer_id.eq(id)).first::<Trailer>(conn).optional()?;

        Ok(result)
    }

    fn all(conn: &PgConnection) -> Result<Vec<Self>, Error> {
        use super::schema::trailers::dsl::*;
        let results = trailers.load::<Trailer>(conn)?;

        Ok(results)
    }
}

impl DbEntity for Bike {
    fn by_id(conn: &PgConnection, id: i32) -> Result<Option<Self>, Error> {
        use super::schema::bikes::dsl::*;
        let result = bikes.filter(bike_id.eq(id)).first::<Bike>(conn).optional()?;

        Ok(result)
    }

    fn by_parent_id(conn: &PgConnection, id: i32) -> Result<Vec<Self>, Error> {
        use super::schema::bikes::dsl::*;
        let results = bikes.filter(trailer.eq(id)).load::<Bike>(conn);

        results
    }
}

impl DbEntity for Oven {
    fn by_id(conn: &PgConnection, id: i32) -> Result<Option<Self>, Error> {
        use super::schema::ovens::dsl::*;
        let result = ovens.filter(oven_id.eq(id)).first::<Oven>(conn).optional()?;

        Ok(result)
    }

    fn by_parent_id(conn: &PgConnection, id: i32) -> Result<Vec<Self>, Error> {
        use super::schema::ovens::dsl::*;
        let results = ovens.filter(trailer.eq(id)).load::<Oven>(conn);

        results
    }
}

impl DbEntity for Solar {
    fn by_id(conn: &PgConnection, id: i32) -> Result<Option<Self>, Error> {
        use super::schema::solar_microgrids::dsl::*;
        let result = solar_microgrids
            .filter(solar_microgrid_id.eq(id))
            .first::<Solar>(conn)
            .optional()?;

        Ok(result)
    }

    fn by_parent_id(conn: &PgConnection, id: i32) -> Result<Vec<Self>, Error> {
        use super::schema::solar_microgrids::dsl::*;
        let results = solar_microgrids.filter(trailer.eq(id)).load::<Solar>(conn);

        results
    }
}

pub fn find_org_id_by_bike_id(_conn: &PgConnection, _id: i32) -> Result<Option<i32>, Error> {
    todo!("This may be used for authentication");
    //    use super::schema::bikes::dsl::*;
    //    use super::schema::trailers::dsl::*;
    //    let result = bikes
    //        .filter(bike_id.eq(id))
    //        .inner_join(trailers)
    //        .select(org)
    //        .first::<i32>(conn)
    //        .optional()?;

    //    Ok(result)
}
