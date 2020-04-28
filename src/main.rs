#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod routes;

fn main() {
  rocket::ignite()
    .mount("/api", routes![])
    .register(catchers![routes::catchers::not_found])
    .launch();
}
