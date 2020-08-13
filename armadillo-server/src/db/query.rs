use crate::db::{Bike, Oven, Solar, Trailer};
use crate::db::{BikeData, OvenData, SolarData};
use crate::db::{Id, Timestamp};

pub trait DbAccess {
    type E: std::error::Error;
    // trailer methods
    fn find_trailer(&self, id: Id) -> Result<Option<Trailer>, Self::E>;
    fn list_all_trailers(&self) -> Result<Vec<Trailer>, Self::E>;

    // bike methods
    fn find_bike(&self, id: Id) -> Result<Option<Bike>, Self::E>;
    fn find_trailer_bikes(&self, trailer_id: Id) -> Result<Vec<Bike>, Self::E>;

    // bike data methods
    fn find_bike_data(&self, bike_id: Id, from: Timestamp, until: Timestamp) -> Result<Vec<BikeData>, Self::E>;
    fn insert_bike_data(&self, bike_data: BikeData) -> Result<BikeData, Self::E>;

    // oven methods
    fn find_oven(&self, id: Id) -> Result<Option<Oven>, Self::E>;
    fn find_trailer_ovens(&self, trailer_id: Id) -> Result<Vec<Oven>, Self::E>;

    // oven data methods
    fn find_oven_data(&self, oven_id: Id, from: Timestamp, until: Timestamp) -> Result<Vec<OvenData>, Self::E>;
    fn insert_oven_data(&self, oven_data: OvenData) -> Result<OvenData, Self::E>;

    // solar methods
    fn find_solar(&self, id: Id) -> Result<Option<Solar>, Self::E>;
    fn find_trailer_solars(&self, trailer_id: Id) -> Result<Vec<Solar>, Self::E>;

    // solar data methods
    fn find_solar_data(&self, solar_id: Id, from: Timestamp, until: Timestamp) -> Result<Vec<SolarData>, Self::E>;
    fn insert_solar_data(&self, solar_data: SolarData) -> Result<SolarData, Self::E>;
}
