#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[get("/")]
fn main_page() -> &'static str {
  "Main page!"
}

#[get("/page/<id>")]
fn page_by_id(id: i32) -> String {
  format!("Page is {}", &id)
}

fn main() {
  rocket::ignite()
    .mount("/", routes![main_page, page_by_id])
    .launch();
}
