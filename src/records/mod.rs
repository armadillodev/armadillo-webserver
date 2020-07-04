use rocket::Rocket;
use rocket_contrib::json::{Json, JsonValue};

#[get("/")]
fn get() -> JsonValue {
    json!({"id": 5})
}

pub fn rocket(rocket: Rocket) -> Rocket {
    rocket.mount("/records", routes![get])
}
