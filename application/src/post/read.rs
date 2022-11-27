use domain::models::Post;
use shared::response_models::{Response, ResponseBody, NetworkResponse::{self, NotFound}};
use infrastructure::establish_connection;
use diesel::prelude::*;

pub fn list_post(post_id: i32) -> Result<Post, NetworkResponse> {
    use domain::schema::posts;

    match posts::table.find(post_id).first::<Post>(&mut establish_connection()) {
        Ok(post) => Ok(post),
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response { body: ResponseBody::Message(format!("Error selecting post with id {} - {}", post_id, err))};
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            },
            _ => {
                panic!("Database error - {}", err);
            }        
        }
    }
}

pub fn list_posts() -> Vec<Post> {
    use domain::schema::posts;

    match posts::table.select(posts::all_columns).load::<Post>(&mut establish_connection()) {
        Ok(mut posts) => {
            posts.sort();
            posts
        },
        // doesn't seem like selecting everything will throw any errors, leaving room for specific error handling just in case though
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}
