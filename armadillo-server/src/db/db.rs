use diesel::prelude::*;
use diesel::result::Error;
use diesel::PgConnection;

use super::schema;
use super::DbAccess;
use super::{Bike, Oven, Solar, Trailer};
use super::{BikeData, OvenData, SolarData};
use super::{Id, Timestamp};

pub struct Db<'a>(pub &'a PgConnection);

macro_rules! find_by_id {
    ($schema:ident, $model:ident, $conn:expr, $id:expr) => {{
        use schema::$schema::dsl::*;
        $schema.find($id).first::<$model>($conn).optional()
    }};
}

macro_rules! find_by_trailer_id {
    ($schema:ident, $model:ident, $conn:expr, $trailer_id:expr) => {{
        use schema::$schema::dsl::*;
        $schema.filter(trailer.eq($trailer_id)).load::<$model>($conn)
    }};
}

macro_rules! find_data {
    ($schema:ident, $model:ident, $conn:expr, $match_id:ident, $id:expr, $from:expr, $until: expr) => {{
        use schema::$schema::dsl::*;
        $schema
            .filter($match_id.eq($id))
            .order(created_at.desc())
            .filter(created_at.ge($from))
            .filter(created_at.lt($until))
            .load::<$model>($conn)
    }};
}

macro_rules! insert_data {
    ($scheme:ident, $model:ident, $conn: expr, $data:expr) => {{
        diesel::insert_into($schema)
            .values($data)
            .get_result::<Option<$model>>($conn)
    }};
}

impl<'a> DbAccess for Db<'a> {
    type E = Error;
    // trailer methods
    fn find_trailer(&self, id: Id) -> Result<Option<Trailer>, Self::E> {
        find_by_id!(trailers, Trailer, self.0, id as i32)
    }
    fn list_all_trailers(&self) -> Result<Vec<Trailer>, Self::E> {
        use schema::trailers::dsl::*;
        trailers.load::<Trailer>(self.0)
    }

    // bike methods
    fn find_bike(&self, id: Id) -> Result<Option<Bike>, Self::E> {
        find_by_id!(bikes, Bike, self.0, id as i32)
    }
    fn find_trailer_bikes(&self, trailer_id: Id) -> Result<Vec<Bike>, Self::E> {
        find_by_trailer_id!(bikes, Bike, self.0, trailer_id as i32)
    }

    // bike data methods
    fn find_bike_data(&self, bike_id: Id, from: Timestamp, until: Timestamp) -> Result<Vec<BikeData>, Self::E> {
        find_data!(
            bike_data,
            BikeData,
            self.0,
            bike,
            bike_id as i32,
            from as i64,
            until as i64
        )
    }
    fn insert_bike_data(&self, data: BikeData) -> Result<BikeData, Self::E> {
        use schema::bike_data::dsl::*;

        diesel::insert_into(bike_data)
            .values((
                created_at.eq(data.created_at),
                bike.eq(data.bike),
                voltage.eq(data.voltage),
                current.eq(data.current),
                rpm.eq(data.rpm),
            ))
            .get_result::<BikeData>(self.0)
    }

    // oven methods
    fn find_oven(&self, id: Id) -> Result<Option<Oven>, Self::E> {
        find_by_id!(ovens, Oven, self.0, id as i32)
    }
    fn find_trailer_ovens(&self, trailer_id: Id) -> Result<Vec<Oven>, Self::E> {
        find_by_trailer_id!(ovens, Oven, self.0, trailer_id as i32)
    }

    // oven data methods
    fn find_oven_data(&self, oven_id: Id, from: Timestamp, until: Timestamp) -> Result<Vec<OvenData>, Self::E> {
        find_data!(
            oven_data,
            OvenData,
            self.0,
            oven,
            oven_id as i32,
            from as i64,
            until as i64
        )
    }
    fn insert_oven_data(&self, data: OvenData) -> Result<OvenData, Self::E> {
        use schema::oven_data::dsl::*;
        diesel::insert_into(oven_data)
            .values((
                created_at.eq(data.created_at),
                oven.eq(data.oven),
                temperature.eq(data.temperature),
            ))
            .get_result::<OvenData>(self.0)
    }

    // solar methods
    fn find_solar(&self, id: Id) -> Result<Option<Solar>, Self::E> {
        find_by_id!(solar_microgrids, Solar, self.0, id as i32)
    }
    fn find_trailer_solars(&self, trailer_id: Id) -> Result<Vec<Solar>, Self::E> {
        find_by_trailer_id!(solar_microgrids, Solar, self.0, trailer_id as i32)
    }

    // solar data methods
    fn find_solar_data(&self, solar_id: Id, from: Timestamp, until: Timestamp) -> Result<Vec<SolarData>, Self::E> {
        find_data!(
            solar_microgrid_data,
            SolarData,
            self.0,
            solar_microgrid,
            solar_id as i32,
            from as i64,
            until as i64
        )
    }
    fn insert_solar_data(&self, data: SolarData) -> Result<SolarData, Self::E> {
        use schema::solar_microgrid_data::dsl::*;

        diesel::insert_into(solar_microgrid_data)
            .values((
                created_at.eq(data.created_at),
                solar_microgrid.eq(data.solar),
                power.eq(data.power),
                temperature.eq(data.temperature),
            ))
            .get_result::<SolarData>(self.0)
    }
}
