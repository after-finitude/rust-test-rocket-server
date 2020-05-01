use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub hash: String,
}

#[derive(Serialize)]
pub struct UserAuth<'a> {
    username: &'a str,
    email: &'a str,
    token: String,
}

impl User {
    pub fn to_user_auth(&self, secret: &[u8]) -> UserAuth {
        UserAuth {
            username: &self.username,
            email: &self.email,
            token: "",
        }
    }
}
