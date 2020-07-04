#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

mod records;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    let mut app = rocket::ignite();
    app = records::rocket(app);
    app = app.mount("/", routes![index]);

    app.launch();
}
