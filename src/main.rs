#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::http::Cookies;
use rocket::Request;

#[get("/")]
fn main_page() -> String {
  format!("Main page!")
}

#[get("/pages")]
fn pages() -> String {
  format!("Pages")
}

#[get("/page/<id>")]
fn page_by_id(id: i32) -> String {
  format!("Page is {}", &id)
}

#[get("/cookies")]
fn cookies(cookies: Cookies) -> Option<String> {
  cookies
    .get("message")
    .map(|value| format!("Message: {}", value))
}

#[catch(404)]
fn not_found(req: &Request) -> String {
  format!("Page '{}' not found", req.uri())
}

fn main() {
  rocket::ignite()
    .mount("/", routes![main_page, pages, page_by_id, cookies])
    .register(catchers![not_found])
    .launch();
}
