use crate::RecordsDbConn;

use rocket_contrib::json::Json;
use diesel::prelude::*;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Trailor {
    pub id: i32,
    pub name: String,
    pub location: String,
    pub org: i32,
}

#[get("/")]
pub fn get_all(conn: RecordsDbConn) -> Json<Vec<Trailor>> {
    use super::schema::trailors::dsl::*;

    let results = trailors
        .load::<Trailor>(&*conn)
        .expect("Database failed");

    Json(results)
}

#[get("/<id>")]
pub fn get(conn: RecordsDbConn, id: i32) -> Option<Json<Trailor>> {
    use super::schema::trailors::dsl::*;

    let results = trailors
        .find(id)
        .load::<Trailor>(&*conn)
        .expect("Database failed")
        .pop()?;

    Some(Json(results))
}
