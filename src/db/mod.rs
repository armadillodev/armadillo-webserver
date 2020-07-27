mod schema;
pub mod models;

pub mod data;
pub mod orgs;

pub use models::{
    Bike,
    User,
    Trailer,
    Org,
    Oven,
    SolarMicrogrid,
    BikeData,
    OvenData,
    SolarMicrogridData,
};

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