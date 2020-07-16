// #![feature(proc_macro_hygiene, decl_macro)]
// #![recursion_limit = "256"]

// #[macro_use]
// extern crate rocket;
// #[macro_use]
// extern crate rocket_contrib;
// #[macro_use]
// extern crate serde_derive;
// #[macro_use]
// extern crate diesel;
// #[macro_use]
// extern crate diesel_migrations;
// #[macro_use]
// extern crate log;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};

struct AppState {
    name: String,
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

async fn index2(data: web::Data<AppState>) -> String {
    let app_name = &data.name;

    format!("Hello {}!", app_name)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(AppState {
                name: String::from("Actix-web")
            })
            .service(
                web::scope("/app")
                    .route("/", web::get().to(index))
                    .route("/again", web::get().to(index2))
            )
    })
    .bind("localhost:8000")?
    .run()
    .await
}