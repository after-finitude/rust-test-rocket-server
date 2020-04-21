#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod routes;

fn main() {
  rocket::ignite()
    .mount(
      "/api",
      routes![
        routes::pages::pages,
        routes::pages::page_by_id,
        routes::other::main_page,
      ],
    )
    .register(catchers![routes::catchers::not_found])
    .launch();
}
