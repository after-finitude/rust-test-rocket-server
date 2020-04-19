#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[get("/")]
fn main_page() -> &'static str {
  "Main page!"
}

#[get("/other")]
fn other_page() -> &'static str {
  "Other page!"
}

fn main() {
  rocket::ignite()
    .mount("/", routes![main_page, other_page])
    .launch();
}
