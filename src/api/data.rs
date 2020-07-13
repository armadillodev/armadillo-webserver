use crate::RecordsDbConn;

use diesel::prelude::*;
use rocket_contrib::json::Json;

#[derive(Serialize, Deserialize)]
pub struct NewTrailorData {
    pub trailor: i32,
    pub temperature: Option<i32>,
}

#[get("/")]
pub fn get_all(conn: RecordsDbConn) -> Json<Vec<TrailorData>> {
    use super::schema::trailor_data::dsl::*;

    let results = trailor_data
        .order(trailor_data_id.desc())
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

#[post("/", format = "json", data = "<new_trailor_data>")]
pub fn new_data(
    conn: RecordsDbConn,
    new_trailor_data: Json<NewTrailorData>,
) -> Option<&'static str> {
    use super::schema::trailor_data::dsl::*;
    diesel::insert_into(trailor_data)
        .values((
            trailor.eq(new_trailor_data.trailor),
            temperature.eq(new_trailor_data.temperature),
        ))
        .execute(&*conn)
        .expect("Error saving trailor data");

    Some("Ok")
}
