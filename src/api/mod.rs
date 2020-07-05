mod schema;
mod orgs;
mod users;
mod trailors;
mod logs;
mod data;

use rocket::Rocket;

pub fn rocket(rocket: Rocket) -> Rocket {
    rocket
        .mount("/api/user", routes![users::get, users::get_all])
        .mount("/api/org", routes![orgs::get, orgs::get_all])
        .mount("/api/trailor", routes![trailors::get, trailors::get_all])
        .mount("/api/log", routes![logs::get, logs::get_all])
        .mount("/api/data", routes![data::get, data::get_all])
}
