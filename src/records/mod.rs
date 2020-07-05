use rocket::Rocket;
use rocket_contrib::json::{Json, JsonValue};

#[derive(Deserialize, Serialize)]
#[repr(u8)]
enum RecordStatus {
    Ok,
}

#[derive(Deserialize, Serialize)]
struct RecordResponse {
    status: RecordStatus,
    value: u32,
}

#[get("/")]
fn get() -> Json<RecordResponse> {
    Json(RecordResponse {
        status: RecordStatus::Ok,
        value: 5,
    })
}

pub fn rocket(rocket: Rocket) -> Rocket {
    rocket.mount("/records", routes![get])
}
