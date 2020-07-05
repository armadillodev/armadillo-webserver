use rocket::Rocket;
use rocket_contrib::json::{Json, JsonValue};

use crate::models::*;
use crate::RecordsDbConn;

use diesel::prelude::*;

#[get("/")]
fn get(conn: RecordsDbConn) -> Json<Vec<User>> {
    use crate::schema::users::dsl::*;

    let results = users.limit(5).load::<User>(&*conn).expect("Error loading posts");
    let mut s = String::new();

    Json(results)
}

pub fn rocket(rocket: Rocket) -> Rocket {
    rocket.mount("/records", routes![get])
}
