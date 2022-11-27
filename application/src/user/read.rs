use domain::models::{User, Post};
use shared::response_models::{Response, ResponseBody, NetworkResponse::{self, NotFound}};
use infrastructure::establish_connection;
use diesel::prelude::*;

fn user_exists(user_id: i32) -> bool {
    use domain::schema::users;

    match users::table.find(user_id).first::<User>(&mut establish_connection()) {
        Ok(_) => true,
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                return false;
            },
            _ => {
                panic!("Database error - {}", err);
            }        
        }
    }
}

pub fn list_user(user_id: i32) -> Result<User, NetworkResponse> {
    use domain::schema::users;

    match users::table.find(user_id).first::<User>(&mut establish_connection()) {
        Ok(user) => Ok(user),
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response { body: ResponseBody::Message(format!("Error selecting user with id {} - {}", user_id, err))};
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            },
            _ => {
                panic!("Database error - {}", err);
            }        
        }
    }
}

pub fn list_users() -> Vec<User> {
    use domain::schema::users;

    match users::table.select(users::all_columns).load::<User>(&mut establish_connection()) {
        Ok(mut users) => {
            users.sort();
            users
        },
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}

pub fn list_users_posts(user_id_: i32) -> Result<Vec<Post>, NetworkResponse> {
    use domain::schema::posts::{self, user_id};

    // check if user exists before getting users posts
    if !user_exists(user_id_) {
        let response = Response { body: ResponseBody::Message(format!("Error selecting user with id {} - No user found", user_id_))};
        return Err(NotFound(serde_json::to_string(&response).unwrap()));
    }

    match posts::table.select(posts::all_columns).filter(user_id.eq(user_id_)).load::<Post>(&mut establish_connection()) {
        Ok(mut posts) => {
            posts.sort();
            Ok(posts)
        },
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}

