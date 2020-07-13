use crate::RecordsDbConn;

use rocket_contrib::json::Json;
use diesel::prelude::*;

#[get("/")]
pub fn get_all(conn: RecordsDbConn) -> Json<Vec<TrailorLog>> {
    use super::schema::bike_logs::dsl::*;

    let results = trailor_logs
        .limit(100)
        .load::<TrailorLog>(&*conn)
        .expect("Database failed");

    Json(results)
}

#[get("/<id>")]
pub fn get(conn: RecordsDbConn, id: i32) -> Option<Json<TrailorLog>> {
    use super::schema::trailor_logs::dsl::*;

    let results = trailor_logs
        .find(id)
        .load::<TrailorLog>(&*conn)
        .expect("Database failed")
        .pop()?;

    Some(Json(results))
}
