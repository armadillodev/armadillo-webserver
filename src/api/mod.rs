mod schema;
mod models;

mod data;
//mod logs;

mod orgs;
//mod users;

use rocket::Rocket;

pub fn rocket(rocket: Rocket) -> Rocket {
    rocket
        .mount("/org", routes![orgs::org_list_view, orgs::org_structure_view])
        .mount("/data/bike", routes![data::post_bike_data])
}
