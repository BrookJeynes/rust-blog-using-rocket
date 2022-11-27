use shared::response_models::{Response, ResponseBody, NetworkResponse::{self, NotFound}};
use infrastructure::establish_connection;
use diesel::prelude::*;
use domain::models::Post;
use crate::auth::Claims;

pub fn delete_post(post_id: i32, key: Claims) -> Result<Vec<Post>, NetworkResponse> {
    use domain::schema::posts::dsl::*;
    use domain::schema::posts;

    // delete post where post_id and user_id
    let num_deleted = match diesel::delete(posts.filter(id.eq(post_id))).filter(user_id.eq(key.subject_id)).execute(&mut establish_connection()) {
        Ok(count) => count,
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response { body: ResponseBody::Message(String::from("Error deleting post - post not found"))};
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            },
            _ => {
                panic!("Database error - {}", err);
            }        
        }
    };

    if num_deleted > 0 {
        match posts::table.select(posts::all_columns).filter(posts::dsl::user_id.eq(key.subject_id)).load::<Post>(&mut establish_connection()) {
            Ok(mut posts_) => {
                posts_.sort();
                Ok(posts_)
            },
            Err(err) => match err {
                _ => {
                    panic!("Database error - {}", err);
                }
            }
        }
    } else {
        let response = Response { body: ResponseBody::Message(String::from("Error deleting post - post not found"))};
        Err(NotFound(serde_json::to_string(&response).unwrap()))
    } 
}
