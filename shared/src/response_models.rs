use domain::models::{User, Post};
use rocket::serde::Serialize;
use rocket::Responder;

#[derive(Serialize)]
pub enum ResponseBody {
    Message(String),
    AuthToken(String),

    Post(Post),
    Posts(Vec<Post>),

    User(User),
    Users(Vec<User>),
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    pub body: ResponseBody,
}

#[derive(Responder, Debug)]
pub enum NetworkResponse {
    #[response(status = 201)]
    Created(String),
    #[response(status = 400)]
    BadRequest(String),
    #[response(status = 401)]
    Unauthorized(String),
    #[response(status = 404)]
    NotFound(String),
    #[response(status = 409)]
    Conflict(String),
}
