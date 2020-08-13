use serde::Serialize;

type Id = i32;
pub type Timestamp = i64;

#[derive(Queryable, Serialize, Default, Debug, PartialEq, Clone)]
pub struct Trailer {
    pub id: Id,
    pub name: String,
    pub location: String,
}

#[derive(Queryable, Serialize, Default, Debug, PartialEq, Clone)]
pub struct User {
    pub id: Id,
    pub org: i32,
    pub first_name: String,
    pub last_name: Option<String>,
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
pub struct UserLog {
    pub id: Id,
    pub client: i32,
    pub bike: i32,
    pub time_start: i32,
    pub time_end: i32,
}

#[derive(Queryable, Serialize, Default, Debug, PartialEq, Clone)]
pub struct TrailerData {
    pub id: Id,
    pub trailer: i32,
    pub created_at: i32,
    pub temperature: Option<i32>,
}

#[derive(Queryable, Serialize, Default, Debug, PartialEq, Clone)]
pub struct BikeData {
    pub id: Id,
    pub bike: i32,
    pub created_at: Timestamp,
    pub voltage: Option<i32>,
    pub rpm: Option<i32>,
    pub current: Option<i32>,
}

#[derive(Queryable, Serialize, Default, Debug, PartialEq, Clone)]
pub struct OvenData {
    pub id: Id,
    pub oven: i32,
    pub created_at: Timestamp,
    pub temperature: Option<f32>,
}

#[derive(Queryable, Serialize, Default, Debug, PartialEq, Clone)]
pub struct SolarData {
    pub id: Id,
    pub solar: Id,
    pub created_at: Timestamp,
    pub temperature: Option<f32>,
    pub power: Option<f32>,
}
