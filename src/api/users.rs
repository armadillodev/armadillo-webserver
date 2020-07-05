use crate::RecordsDbConn;

use rocket_contrib::json::Json;
use diesel::prelude::*;

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: Option<String>,
    pub org: i32,
}

#[get("/")]
pub fn get_all(conn: RecordsDbConn) -> Json<Vec<User>> {
    use super::schema::users::dsl::*;

    let results = users
        .load::<User>(&*conn)
        .expect("Database failed");

    Json(results)
}

#[get("/<id>")]
pub fn get(conn: RecordsDbConn, id: i32) -> Option<Json<User>> {
    use super::schema::users::dsl::*;

    let results = users
        .find(id)
        .load::<User>(&*conn)
        .expect("Database failed")
        .pop()?;

    Some(Json(results))
}
