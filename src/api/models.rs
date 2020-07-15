use std::time::SystemTime;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Org {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Trailer {
    pub id: i32,
    pub name: String,
    pub location: String,
    pub org: i32,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub org: i32,
    pub first_name: String,
    pub last_name: Option<String>,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Bike {
    pub id: i32,
    pub trailor: i32,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct UserLog {
    pub id: i32,
    pub client: i32,
    pub bike: i32,
    pub time_start: i32,
    pub time_end: i32,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct TrailerData {
    pub id: i32,
    pub trailer: i32,
    pub created_at: i32,
    pub temperature: Option<i32>,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct BikeData {
    pub id: i32,
    pub bike: i32,
    pub created_at: SystemTime,
    pub voltage: Option<i32>,
    pub rpm: Option<i32>,
    pub current: Option<i32>,
}
