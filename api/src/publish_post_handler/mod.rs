use shared::response_models::{Response, ResponseBody};
use application::publish_post::publish_post;
use rocket::get;
use rocket::response::status::NotFound;

#[get("/publish/<post_id>")]
pub fn publish_post_handler(post_id: i32) -> Result<String, NotFound<String>> {
    let post = publish_post(post_id)?; 

    let response = Response { body: ResponseBody::Post(post) };

    Ok(serde_json::to_string(&response).unwrap())
}
