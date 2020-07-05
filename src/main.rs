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

#[database("sqlite_records")]
struct RecordsDbConn(diesel::SqliteConnection);

#[get("/")]
fn index() -> &'static str{
    "Hello World!"
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
