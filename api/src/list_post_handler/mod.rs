use shared::response_models::{Response, ResponseBody};
use application::list_post::list_post;
use rocket::response::status::NotFound;
use rocket::get;

#[get("/post/<post_id>")]
pub fn list_post_handler(post_id: i32) -> Result<String, NotFound<String>> {
    let post = list_post(post_id)?;

    let response = Response { body: ResponseBody::Post(post) };

    Ok(serde_json::to_string(&response).unwrap())
}
