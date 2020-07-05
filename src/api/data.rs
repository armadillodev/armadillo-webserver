use crate::RecordsDbConn;

use rocket_contrib::json::Json;
use diesel::prelude::*;

#[derive(Queryable, Serialize, Deserialize)]
pub struct TrailorData {
    pub id: i32,
    pub trailor: i32,
    pub timestamp: i32,
    pub temperature: Option<i32>,
}

#[get("/")]
pub fn get_all(conn: RecordsDbConn) -> Json<Vec<TrailorData>> {
    use super::schema::trailor_data::dsl::*;

    let results = trailor_data
        .limit(100)
        .load::<TrailorData>(&*conn)
        .expect("Database failed");

    Json(results)
}

#[get("/<id>")]
pub fn get(conn: RecordsDbConn, id: i32) -> Option<Json<TrailorData>> {
    use super::schema::trailor_data::dsl::*;

    let results = trailor_data
        .find(id)
        .load::<TrailorData>(&*conn)
        .expect("Database failed")
        .pop()?;

    Some(Json(results))
}
