#[get("/pages")]
pub fn pages() -> String {
  format!("Pages")
}

#[get("/page/<id>")]
pub fn page_by_id(id: i32) -> String {
  format!("Page is {}", &id)
}
