#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;

pub mod db;
pub mod schema;

pub use db::Db;

embed_migrations!();

fn run_db_migrations(pool: DbPool) -> Result<(), String> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    if let Err(e) = embedded_migrations::run(&conn) {
        return Err(format!("Failed to run database migrations: {:?}", e));
    }

    Ok(())
}

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn connect_to_pool() -> DbPool {
    // setup database connection pool
    let connspec =
        std::env::var("DATABASE_URL").unwrap_or("postgres://postgres:postgres@localhost/armadillo".to_string());
    let manager = ConnectionManager::<PgConnection>::new(connspec);
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to build pool");

    // run migrations
    run_db_migrations(pool.clone()).unwrap();

    pool
}
