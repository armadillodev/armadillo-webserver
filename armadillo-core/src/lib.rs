mod models;

pub mod data;
mod entity;
mod query;
#[cfg(test)]
mod test_query;

//pub mod record;

//pub mod orgs;

pub use entity::TrailerEntity;
pub use query::DbAccess;

pub use models::{Bike, BikeData, Oven, OvenData, Solar, SolarData, Trailer, User};
pub type Id = u32;
pub type Timestamp = u64;
