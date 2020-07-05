#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;

mod records;
mod schema;
mod models;

use rocket::Rocket;
use rocket_contrib::serve::StaticFiles;

use diesel::prelude::*;

#[database("sqlite_records")]
struct RecordsDbConn(diesel::SqliteConnection);

#[get("/")]
fn index(conn: RecordsDbConn) -> String {
    use schema::users::dsl::*;
    use models::User;

    let results = users.limit(5).load::<User>(&*conn).expect("Error loading posts");
    let mut s = String::new();
    for user in results {
        s = format!("{}\n{}", s, user.first_name);
    }
    s
}

fn rocket() -> Rocket {
    rocket::ignite()
        .attach(RecordsDbConn::fairing())
        .mount("/", routes![index])
        .mount("/static", StaticFiles::from("static"))
}

fn main() {
    let mut app = rocket();
    app = records::rocket(app);
    app.launch();
}
