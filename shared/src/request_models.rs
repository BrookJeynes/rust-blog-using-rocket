use rocket::serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub user_name: String,
    pub password: String,
}
