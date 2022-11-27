use domain::models::{Post, NewPost, NewPostRequest};
use shared::response_models::{Response, ResponseBody, NetworkResponse::{self, Created}};
use infrastructure::establish_connection;
use diesel::prelude::*;
use rocket::serde::json::Json;

use crate::auth::JWT;

pub fn create_post(post: Json<NewPostRequest>, key: JWT) -> NetworkResponse {
    use domain::schema::posts;

    let post = post.into_inner();

    let post = NewPost {
        user_id: key.claims.subject_id,
        title: post.title,
        body: post.body,
        genre: post.genre
    };

    match diesel::insert_into(posts::table).values(&post).get_result::<Post>(&mut establish_connection()) {
        Ok(post) => {
            let response = Response { body: ResponseBody::Post(post) };
            Created(serde_json::to_string(&response).unwrap())
        },
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}
