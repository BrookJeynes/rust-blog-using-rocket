use domain::models::NewPost;
use application::create_post::create_post;
use rocket::response::status::Created;
use rocket::post;
use rocket::serde::json::Json;

#[post("/new_post", format = "application/json", data = "<post>")]
pub fn create_post_handler(post: Json<NewPost>) -> Created<String> {
    create_post(post)
}
