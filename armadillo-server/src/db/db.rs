use diesel::prelude::*;
use diesel::result::Error;
use diesel::PgConnection;

use super::models::{Bike, Oven, Solar, Trailer};
use super::schema;
use super::DbAccess;
use super::Id;

pub struct Db<'a>(pub &'a PgConnection);

macro_rules! find_by_id {
    ($schema:ident, $model:ident, $conn:expr, $match_id:ident, $id:expr) => {{
        use schema::$schema::dsl::*;
        $schema.filter($match_id.eq($id)).first::<$model>($conn).optional()
    }};
}

macro_rules! find_by_trailer_id {
    ($schema:ident, $model:ident, $conn:expr, $trailer_id:expr) => {{
        use schema::$schema::dsl::*;
        $schema.filter(trailer.eq($trailer_id)).load::<$model>($conn)
    }};
}

impl<'a> DbAccess for Db<'a> {
    type E = Error;
    // trailer methods
    fn find_trailer(&self, id: Id) -> Result<Option<Trailer>, Self::E> {
        find_by_id!(trailers, Trailer, self.0, trailer_id, id as i32)
    }
    fn list_all_trailers(&self) -> Result<Vec<Trailer>, Self::E> {
        use schema::trailers::dsl::*;
        trailers.load::<Trailer>(self.0)
    }

    // bike methods
    fn find_bike(&self, id: Id) -> Result<Option<Bike>, Self::E> {
        find_by_id!(bikes, Bike, self.0, bike_id, id as i32)
    }
    fn find_trailer_bikes(&self, trailer_id: Id) -> Result<Vec<Bike>, Self::E> {
        find_by_trailer_id!(bikes, Bike, self.0, trailer_id as i32)
    }

    // oven methods
    fn find_oven(&self, id: Id) -> Result<Option<Oven>, Self::E> {
        find_by_id!(ovens, Oven, self.0, oven_id, id as i32)
    }
    fn find_trailer_ovens(&self, trailer_id: Id) -> Result<Vec<Oven>, Self::E> {
        find_by_trailer_id!(ovens, Oven, self.0, trailer_id as i32)
    }

    // solar methods
    fn find_solar(&self, id: Id) -> Result<Option<Solar>, Self::E> {
        find_by_id!(solar_microgrids, Solar, self.0, solar_microgrid_id, id as i32)
    }
    fn find_trailer_solars(&self, trailer_id: Id) -> Result<Vec<Solar>, Self::E> {
        find_by_trailer_id!(solar_microgrids, Solar, self.0, trailer_id as i32)
    }
}
