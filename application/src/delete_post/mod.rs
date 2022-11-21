use shared::response_models::{Response, ResponseBody};
use infrastructure::establish_connection;
use diesel::prelude::*;
use rocket::response::status::{NoContent, NotFound};

pub fn delete_post(post_id: i32) -> Result<NoContent, NotFound<String>> {
    use domain::schema::posts::dsl::*;

    let response: Response;

    let num_deleted = match diesel::delete(posts.filter(id.eq(post_id))).execute(&mut establish_connection()) {
        Ok(count) => count,
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response { body: ResponseBody::Message(format!("Error publishing post with id {} - {}", post_id, err))};
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            },
            _ => {
                panic!("Database error - {}", err);
            }        
        }
    };

    if num_deleted > 0 {
        Ok(NoContent)
    } else {
        response = Response { body: ResponseBody::Message(format!("Error - no post with id {}", post_id))};
        Err(NotFound(serde_json::to_string(&response).unwrap()))
    } 
}
