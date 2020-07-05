use crate::RecordsDbConn;

use rocket_contrib::json::Json;
use diesel::prelude::*;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Org {
    pub id: i32,
    pub name: String,
}

#[get("/")]
pub fn get_all(conn: RecordsDbConn) -> Json<Vec<Org>> {
    use super::schema::orgs::dsl::*;

    let results = orgs
        .load::<Org>(&*conn)
        .expect("Database failed");

    Json(results)
}

#[get("/<id>")]
pub fn get(conn: RecordsDbConn, id: i32) -> Option<Json<Org>> {
    use super::schema::orgs::dsl::*;

    let results = orgs 
        .find(id)
        .load::<Org>(&*conn)
        .expect("Database failed")
        .pop()?;

    Some(Json(results))
}
