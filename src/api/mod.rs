mod data;
mod logs;
mod orgs;
mod schema;
mod trailors;
mod users;

use rocket::Rocket;

pub fn rocket(rocket: Rocket) -> Rocket {
    rocket
        .mount("/user", routes![users::get, users::get_all])
        .mount("/org", routes![orgs::get, orgs::get_all])
        .mount("/trailor", routes![trailors::get, trailors::get_all])
        .mount("/log", routes![logs::get, logs::get_all])
        .mount("/data", routes![data::get, data::get_all, data::new_data])
}
