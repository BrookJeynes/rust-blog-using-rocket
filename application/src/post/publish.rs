use domain::models::Post;
use shared::response_models::{Response, ResponseBody, NetworkResponse::{self, NotFound}};
use infrastructure::establish_connection;
use diesel::prelude::*;

use crate::auth::JWT;

pub fn publish_post(post_id: i32, key: JWT) -> Result<Post, NetworkResponse> {
    use domain::schema::posts::dsl::*;

    match diesel::update(posts.find(post_id)).set(published.eq(true)).filter(user_id.eq(key.claims.subject_id)).get_result::<Post>(&mut establish_connection()) {
        Ok(post) => Ok(post),
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response { body: ResponseBody::Message(format!("Error publishing post with id {} - {}", post_id, err))};
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            },
            _ => {
                panic!("Database error - {}", err);
            }        
        }
    }
}
