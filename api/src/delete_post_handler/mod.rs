use application::delete_post::delete_post;
use rocket::response::status::{NoContent, NotFound};
use rocket::get;

#[get("/delete/<post_id>")]
pub fn delete_post_handler(post_id: i32) -> Result<NoContent, NotFound<String>> {
    delete_post(post_id)
}
