// #![feature(proc_macro_hygiene, decl_macro)]
// #![recursion_limit = "256"]

#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;
#[macro_use] extern crate log;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer, HttpResponse, Error, Responder};
use diesel::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use serde::Serialize;

mod db;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Serialize)]
struct OrgStructure {
    org: db::models::Org,
    trailers: Vec<db::models::Trailer>,
    bikes: Vec<db::models::Bike>,
}

impl OrgStructure {
    fn new(conn: &PgConnection, org_id: i32) -> Result<Option<Self>, diesel::result::Error> {
        let org = match db::orgs::find_org_by_id(conn, org_id)? {
            Some(org) => org,
            None => return Ok(None),
        };
        let trailers = db::orgs::find_trailers_by_org_id(conn, org_id)?;
        let bikes = trailers.iter()
            .map(|trailer| db::orgs::find_bikes_by_trailer_id(conn, trailer.id))
            .flatten()
            .flatten()
            .collect::<Vec<db::models::Bike>>();

        Ok(Some(OrgStructure { org, trailers, bikes }))
    }
}

// run migrations on database
embed_migrations!();

fn run_db_migrations(pool: DbPool) -> Result<(), String> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    if let Err(e) = embedded_migrations::run(&conn) {
        return Err(format!("Failed to run database migrations: {:?}", e));
    }

    Ok(())
}

// routes for getting bike data
async fn get_bike_data(pool: web::Data<DbPool>, bike_id: web::Path<i32>) -> Result<impl Responder, Error> {
    let bike_id = bike_id.into_inner();
    let conn = pool.get().expect("couldn't get db connection from pool");

    let bikes = web::block(move || db::data::find_bike_data(&conn, bike_id, 100))
        .await
        .map_err(|e| {
            error!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    if bikes.len() == 0 {
        Ok(HttpResponse::NotFound().body(format!("no bike data with id: {} was found", bike_id)))
    } else {
        Ok(HttpResponse::Ok().json(bikes))
    }
}

async fn get_latest_bike_data(pool: web::Data<DbPool>, bike_id: web::Path<i32>) -> Result<impl Responder, Error> {
    let bike_id = bike_id.into_inner();
    let conn = pool.get().expect("couldn't get db connection from pool");

    let mut bike = web::block(move || db::data::find_bike_data(&conn, bike_id, 1))
        .await
        .map_err(|e| {
            error!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    if bike.len() == 0 {
        return Ok(HttpResponse::NotFound().body(format!("no bike data with id: {} was found", bike_id)));
    }
    
    Ok(HttpResponse::Ok().json(bike.pop().unwrap()))
}

// route for posting bike data
async fn add_bike_data(
    pool: web::Data<DbPool>,
    bike_id: web::Path<i32>,
    data: web::Json<db::data::CreateBikeData>
) -> Result<impl Responder, Error> {
    let bike_id = bike_id.into_inner();
    let conn = pool.get().expect("couldn't get connection from pool");
    let data = data.into_inner();

    web::block(move || db::data::insert_new_bike_data(&conn, bike_id, data))
        .await
        .map_err(|e| {
            error!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    // no need to send data back
    Ok(HttpResponse::Ok().finish())
}

// get the structure of the organization
async fn get_org_structure(
    pool: web::Data<DbPool>,
    org_id: web::Path<i32>,
) -> Result<impl Responder, Error> {
    let org_id = org_id.into_inner();
    let conn = pool.get().expect("couldn't get connection from pool");

    let org = web::block(move || OrgStructure::new(&conn, org_id))
        .await
        .map_err(|e| {
            error!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    match org {
        Some(org) => Ok(HttpResponse::Ok().json(org)),
        None => Ok(HttpResponse::NotFound().body(format!("no org with id: {} was found", org_id))),
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");

    // setup database connection pool
    let connspec = std::env::var("DATABASE_URL")
        .unwrap_or("postgres://postgres:postgres@localhost/armadillo".to_string());
    let manager = ConnectionManager::<PgConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to build pool");


    let bind = "localhost:8000";

    println!("starting server at: {}", bind);

    // run migrations
    run_db_migrations(pool.clone()).unwrap();

    // start server
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(Logger::default())
            .service(web::scope("/data")
                .route("/bike/{bike_id}", web::get().to(get_bike_data))
                .route("/bike/{bike_id}/latest", web::get().to(get_latest_bike_data))
                .route("/bike/{bike_id}", web::post().to(add_bike_data))
            )
            .service(web::scope("/org")
                .route("/{org_id}", web::get().to(get_org_structure))
            )
    })
    .bind(&bind)?
    .run()
    .await
}
