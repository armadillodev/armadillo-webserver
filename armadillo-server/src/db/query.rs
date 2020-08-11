use crate::db::{Bike, Oven, Solar, Trailer};
use crate::db::{Id, Timestamp};

pub trait DbAccess {
    // trailer methods
    fn find_trailer(&self, id: Id) -> Option<Trailer>;
    fn list_all_trailers(&self) -> Vec<Trailer>;

    // bike methods
    fn find_bike(&self, id: Id) -> Option<Bike>;
    fn find_trailer_bikes(&self, trailer_id: Id) -> Vec<Bike>;

    // oven methods
    fn find_oven(&self, id: Id) -> Option<Oven>;
    fn find_trailer_ovens(&self, trailer_id: Id) -> Vec<Oven>;

    // solar methods
    fn find_solar(&self, id: Id) -> Option<Solar>;
    fn find_trailer_solars(&self, trailer_id: Id) -> Vec<Solar>;
}
