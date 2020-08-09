// #![feature(proc_macro_hygiene, decl_macro)]
// #![recursion_limit = "256"]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate log;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;

mod data;
mod db;
mod org;
mod trailer;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

// run migrations on database
embed_migrations!();

fn run_db_migrations(pool: DbPool) -> Result<(), String> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    if let Err(e) = embedded_migrations::run(&conn) {
        return Err(format!("Failed to run database migrations: {:?}", e));
    }

    Ok(())
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // setup logging
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    // setup database connection pool
    let connspec =
        std::env::var("DATABASE_URL").unwrap_or("postgres://postgres:postgres@localhost/armadillo".to_string());
    let manager = ConnectionManager::<PgConnection>::new(connspec);
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to build pool");

    // run migrations
    run_db_migrations(pool.clone()).unwrap();

    // start server
    let bind = std::env::var("BIND_TO").unwrap_or(String::from("0.0.0.0:3001"));

    println!("starting server at: {}", bind);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(Logger::default())
            .service(
                web::scope("/data")
                    .route("/bike/{bike_id}", web::get().to(data::get_data::<db::BikeData>))
                    .route("/bike/{bike_id}", web::post().to(data::post_data::<db::BikeData>))
                    .route("/bike/{bike_id}/latest", web::get().to(data::get_latest_bike_data))
                    .route("/bike/{bike_id}/org", web::get().to(org::get_org_id_for_bike))
                    .route("/oven/{oven_id}", web::get().to(data::get_data::<db::OvenData>))
                    .route("/oven/{oven_id}", web::post().to(data::post_data::<db::OvenData>))
                    .route(
                        "/microgrid/{microgrid_id}",
                        web::get().to(data::get_data::<db::SolarMicrogridData>),
                    )
                    .route(
                        "/microgrid/{microgrid_id}",
                        web::post().to(data::post_data::<db::SolarMicrogridData>),
                    ),
            )
            .service(
                web::scope("/trailer")
                    .route("", web::get().to(trailer::get_trailer_list))
                    .route("/{trailer_id}", web::get().to(trailer::get_trailer_node)),
            )
    })
    .bind(&bind)?
    .run()
    .await
}
