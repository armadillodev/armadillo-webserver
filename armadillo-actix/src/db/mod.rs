pub mod models;
mod schema;

pub mod bike;
pub mod data;
pub mod orgs;

pub use models::{Bike, BikeData, Org, Oven, OvenData, SolarMicrogrid, SolarMicrogridData, Trailer, User};

#[derive(Hash, Copy, Clone, Debug, Eq, PartialEq)]
pub enum Address {
    Bike(i32),
    Oven(i32),
    Microgrid(i32),
}

pub use data::DataQuery;
pub use data::DbData;
pub use orgs::DbEntity;
//mod logs;
//mod users;
