mod models;

pub mod data;
mod db;
mod entity;
mod query;
#[cfg(test)]
mod test_query;

//pub mod record;

//pub mod orgs;

use query::DbAccess;

pub use db::Db;
pub use entity::TrailerEntity;
pub use models::{Bike, BikeData, Oven, OvenData, Solar, SolarData, Trailer, User};
pub type Id = u32;
pub type Timestamp = u64;

//pub use data::DataQuery;
//pub use orgs::DbEntity;
