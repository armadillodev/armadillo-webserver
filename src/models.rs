#[derive(Queryable)]
pub struct Org {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable)]
pub struct Trailor {
    pub id: i32,
    pub name: String,
    pub location: String,
    pub org: Org,
}

#[derive(Queryable)]
pub struct TrailorData {
    pub id: i32,
    pub trailor: Trailor,
    pub timestamp: i32,
    pub temperature: Option<i32>,
}

#[derive(Queryable)]
pub struct TrailorLog {
    pub id: i32,
    pub user: User,
    pub trailor: Trailor,
    pub time_start: i32,
    pub time_end: i32,
}

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: Option<String>,
    pub org: i32,
}
