use application::auth::JWT;
use shared::response_models::{Response, ResponseBody, NetworkResponse};
use shared::request_models::LoginRequest;
use application::user::{create, read, delete, login};
use domain::models::NewUser;
use rocket::{get, post};
use rocket::serde::json::Json;

#[get("/")]
pub fn list_users_handler() -> String {
    let users = read::list_users();

    let response = Response { body: ResponseBody::Users(users) };

    serde_json::to_string(&response).unwrap()
}

#[get("/posts/<user_id>")]
pub fn list_users_posts_handler(user_id: i32) -> Result<String, NetworkResponse> {
    let posts = read::list_users_posts(user_id)?;

    let response = Response { body: ResponseBody::Posts(posts) };

    Ok(serde_json::to_string(&response).unwrap())
}

#[get("/<user_id>")]
pub fn list_user_handler(user_id: i32) -> Result<String, NetworkResponse> {
    let user = read::list_user(user_id)?;

    let response = Response { body: ResponseBody::User(user) };

    Ok(serde_json::to_string(&response).unwrap())
}

#[get("/delete/<user_id>")]
pub fn delete_user_handler(user_id: i32, key: Result<JWT, NetworkResponse>) -> Result<String, NetworkResponse> {
    let key = match key {
        Ok(key) => key,
        Err(err) => return Err(err),
    };

    let users = delete::delete_user(user_id, key)?;

    let response = Response { body: ResponseBody::Users(users) };

    Ok(serde_json::to_string(&response).unwrap())
}

#[post("/register", format = "application/json", data = "<user>")]
pub fn create_user_handler(user: Json<NewUser>) -> Result<NetworkResponse, NetworkResponse> {
    Ok(create::create_user(user))?
}

#[post("/login", format = "application/json", data = "<user>")]
pub fn login_user_handler(user: Json<LoginRequest>) -> Result<String, NetworkResponse> {
    let token = login::login_user(user)?;

    let response = Response { body: ResponseBody::AuthToken(token) };

    Ok(serde_json::to_string(&response).unwrap())
}

