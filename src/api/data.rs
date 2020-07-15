use crate::RecordsDbConn;

use diesel::prelude::*;
use rocket_contrib::json::Json;

#[derive(Serialize, Deserialize)]
pub struct NewBikeData {
    pub voltage: Option<i32>,
    pub rpm: Option<i32>,
    pub current: Option<i32>,
}

/*
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
*/

#[post("/<bike_id>", format = "json", data = "<new_bike_data>")]
pub fn post_bike_data(
    conn: RecordsDbConn,
    bike_id: i32,
    new_bike_data: Json<NewBikeData>,
) -> Option<&'static str> {
    use super::schema::bike_data::dsl::*;
    diesel::insert_into(bike_data)
        .values((
            bike.eq(bike_id),
            voltage.eq(new_bike_data.voltage),
            current.eq(new_bike_data.current),
            rpm.eq(new_bike_data.rpm),
        ))
        .execute(&*conn)
        .expect("Error saving trailor data");

    Some("Ok")
}
