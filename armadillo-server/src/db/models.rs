use serde::Serialize;
use std::time::SystemTime;

#[derive(Queryable, Serialize)]
pub struct Org {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Serialize)]
pub struct Trailer {
    pub id: i32,
    pub name: String,
    pub location: String,
}

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub org: i32,
    pub first_name: String,
    pub last_name: Option<String>,
}

#[derive(Queryable, Serialize)]
pub struct Oven {
    pub id: i32,
    pub trailer: i32,
}

#[derive(Queryable, Serialize)]
pub struct Bike {
    pub id: i32,
    pub trailer: i32,
}

#[derive(Queryable, Serialize)]
pub struct SolarMicrogrid {
    pub id: i32,
    pub trailer: i32,
    pub capacity: Option<f32>,
}

#[derive(Queryable, Serialize)]
pub struct UserLog {
    pub id: i32,
    pub client: i32,
    pub bike: i32,
    pub time_start: i32,
    pub time_end: i32,
}

#[derive(Queryable, Serialize)]
pub struct TrailerData {
    pub id: i32,
    pub trailer: i32,
    pub created_at: i32,
    pub temperature: Option<i32>,
}

#[derive(Queryable, Serialize, Clone)]
pub struct BikeData {
    pub id: i32,
    pub bike: i32,
    pub created_at: SystemTime,
    pub voltage: Option<i32>,
    pub rpm: Option<i32>,
    pub current: Option<i32>,
}

#[derive(Queryable, Serialize)]
pub struct OvenData {
    pub id: i32,
    pub oven: i32,
    pub created_at: SystemTime,
    pub temperature: Option<f32>,
}

#[derive(Queryable, Serialize)]
pub struct SolarMicrogridData {
    pub id: i32,
    pub solar_microgrid: i32,
    pub created_at: SystemTime,
    pub temperature: Option<f32>,
    pub power: Option<f32>,
}
