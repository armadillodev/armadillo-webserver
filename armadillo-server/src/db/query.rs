use crate::db::{Bike, Oven, Solar, Trailer};
use crate::db::{Id, Timestamp};

pub trait DbAccess {
    type E: std::error::Error;
    // trailer methods
    fn find_trailer(&self, id: Id) -> Result<Option<Trailer>, Self::E>;
    fn list_all_trailers(&self) -> Result<Vec<Trailer>, Self::E>;

    // bike methods
    fn find_bike(&self, id: Id) -> Result<Option<Bike>, Self::E>;
    fn find_trailer_bikes(&self, trailer_id: Id) -> Result<Vec<Bike>, Self::E>;

    // oven methods
    fn find_oven(&self, id: Id) -> Result<Option<Oven>, Self::E>;
    fn find_trailer_ovens(&self, trailer_id: Id) -> Result<Vec<Oven>, Self::E>;

    // solar methods
    fn find_solar(&self, id: Id) -> Result<Option<Solar>, Self::E>;
    fn find_trailer_solars(&self, trailer_id: Id) -> Result<Vec<Solar>, Self::E>;
}
