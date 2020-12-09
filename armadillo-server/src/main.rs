#![allow(dead_code)]
#![allow(unused_imports)]

#[macro_use]
extern crate log;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use armadillo_database::{DbPool, connect_to_pool};
use armadillo_core::{BikeData, SolarData, OvenData};

mod data;
mod trailer;
mod time;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // setup logging
    //std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    // setup database connection pool
    let pool = connect_to_pool();

    // start server
    let bind = std::env::var("PORT").unwrap_or(String::from("0.0.0.0:3001"));
    let bind = format!("0.0.0.0:{}", bind);

    println!("starting server at: {}", bind);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(Logger::default())
            .service(
                web::scope("/data")
                    .route("/bike/{bike_id}", web::get().to(data::get_json_data_route::<BikeData>))
                    .route("/bike/{bike_id}/csv", web::get().to(data::get_csv_data_route::<BikeData>))
                    .route("/bike/{bike_id}", web::post().to(data::post_data::<BikeData>))
                    //.route("/bike/{bike_id}/latest", web::get().to(data::get_latest_bike_data))
                    .route("/oven/{oven_id}", web::get().to(data::get_json_data_route::<OvenData>))
                    .route("/oven/{oven_id}", web::post().to(data::post_data::<OvenData>))
                    .route("/solar/{solar_id}", web::get().to(data::get_json_data_route::<SolarData>))
                    .route("/solar/{solar_id}", web::post().to(data::post_data::<SolarData>))
            )
            .service(
                web::scope("/trailer")
                    .route("", web::get().to(trailer::get_trailer_list))
                    .route("/{trailer_id}", web::get().to(trailer::get_trailer_node)),
            )
            .route("/time", web::get().to(time::time_route))
    })
    .bind(&bind)?
    .run()
    .await
}
