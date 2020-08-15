use diesel::Queryable;
use serde::Serialize;

type Id = i32;
type Timestamp = i64;

#[derive(Queryable, Serialize, Default, Debug, PartialEq, Clone)]
pub struct Trailer {
    pub id: Id,
    pub name: String,
    pub location: String,
}

#[derive(Queryable, Serialize, Default, Debug, PartialEq, Clone)]
pub struct Oven {
    pub id: Id,
    pub trailer: i32,
}

#[derive(Queryable, Serialize, Default, Debug, PartialEq, Clone)]
pub struct Bike {
    pub id: Id,
    pub trailer: i32,
}

#[derive(Queryable, Serialize, Default, Debug, PartialEq, Clone)]
pub struct Solar {
    pub id: Id,
    pub trailer: i32,
    pub capacity: Option<f32>,
}

#[derive(Queryable, Serialize, Default, Debug, PartialEq, Clone)]
pub struct BikeData {
    pub bike: Id,
    pub created_at: Timestamp,
    pub voltage: Option<i32>,
    pub rpm: Option<i32>,
    pub current: Option<i32>,
}

#[derive(Queryable, Serialize, Default, Debug, PartialEq, Clone)]
pub struct OvenData {
    pub oven: Id,
    pub created_at: Timestamp,
    pub temperature: Option<f32>,
}

#[derive(Queryable, Serialize, Default, Debug, PartialEq, Clone)]
pub struct SolarData {
    pub solar: Id,
    pub created_at: Timestamp,
    pub temperature: Option<f32>,
    pub power: Option<f32>,
}
