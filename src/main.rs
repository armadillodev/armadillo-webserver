#![feature(proc_macro_hygiene, decl_macro)]
#![recursion_limit = "256"]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate log;

mod api;

use rocket::fairing::AdHoc;
use rocket::Rocket;

embed_migrations!();

#[database("sqlite_records")]
pub struct RecordsDbConn(diesel::SqliteConnection);

fn run_db_migrations(rocket: Rocket) -> Result<Rocket, Rocket> {
    let conn = RecordsDbConn::get_one(&rocket).expect("database connection");
    match embedded_migrations::run(&*conn) {
        Ok(()) => Ok(rocket),
        Err(e) => {
            error!("Failed to run database migrations: {:?}", e);
            Err(rocket)
        }
    }
}

fn rocket() -> Rocket {
    rocket::ignite()
        .attach(RecordsDbConn::fairing())
        .attach(AdHoc::on_attach("Database migrations", run_db_migrations))
}

fn main() {
    let mut app = rocket();
    app = api::rocket(app);

    app.launch();
}
