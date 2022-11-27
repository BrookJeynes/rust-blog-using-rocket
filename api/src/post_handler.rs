use shared::response_models::{Response, ResponseBody, NetworkResponse};
use application::post::{create, read, publish, delete};
use application::auth::JWT;
use domain::models::NewPostRequest;
use rocket::{get, post};
use rocket::serde::json::Json;

#[get("/")]
pub fn list_posts_handler() -> String {
    let posts = read::list_posts();

    let response = Response { body: ResponseBody::Posts(posts) };

    serde_json::to_string(&response).unwrap()
}

#[get("/<post_id>")]
pub fn list_post_handler(post_id: i32) -> Result<String, NetworkResponse> {
    let post = read::list_post(post_id)?;

    let response = Response { body: ResponseBody::Post(post) };

    Ok(serde_json::to_string(&response).unwrap())
}

#[get("/publish/<post_id>")]
pub fn publish_post_handler(post_id: i32, key: Result<JWT, NetworkResponse>) -> Result<String, NetworkResponse> {
    let key = match key {
        Ok(key) => key,
        Err(err) => return Err(err),
    };

    let post = publish::publish_post(post_id, key)?; 

    let response = Response { body: ResponseBody::Post(post) };

    Ok(serde_json::to_string(&response).unwrap())
}

#[post("/create", format = "application/json", data = "<post>")]
pub fn create_post_handler(post: Json<NewPostRequest>, key: Result<JWT, NetworkResponse>) -> Result<NetworkResponse, NetworkResponse> {
    let key = match key {
        Ok(key) => key,
        Err(err) => return Err(err),
    };

    Ok(create::create_post(post, key))
}

#[get("/delete/<post_id>")]
pub fn delete_post_handler(post_id: i32, key: Result<JWT, NetworkResponse>) -> Result<String, NetworkResponse> {
    let key = match key {
        Ok(key) => key,
        Err(err) => return Err(err),
    };

    let posts = delete::delete_post(post_id, key.claims)?;

    let response = Response { body: ResponseBody::Posts(posts) };

    Ok(serde_json::to_string(&response).unwrap())
}
