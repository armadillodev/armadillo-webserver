mod models;
mod schema;

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
pub use models::Timestamp;
pub use models::{Bike, BikeData, Oven, OvenData, Solar, SolarData, Trailer, User};
pub type Id = u32;

//pub use data::DataQuery;
//pub use orgs::DbEntity;
