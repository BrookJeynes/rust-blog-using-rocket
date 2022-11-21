use shared::response_models::{Response, ResponseBody};
use application::list_posts::list_posts;
use rocket::get;

#[get("/")]
pub fn list_posts_handler() -> String {
    let posts = list_posts();

    let response = Response { body: ResponseBody::Posts(posts) };

    serde_json::to_string(&response).unwrap()
}
