use shared::response_models::{Response, ResponseBody, NetworkResponse::{self, Unauthorized, NotFound}};
use infrastructure::establish_connection;
use diesel::prelude::*;
use domain::models::User;
use crate::auth::JWT;

pub fn delete_user(user_id: i32, key: JWT) -> Result<Vec<User>, NetworkResponse> {
    use domain::schema::users;
    use domain::schema::users::dsl::*;
    use domain::schema::posts::dsl::{posts, user_id as id};

    if user_id != key.claims.subject_id {
        let response = Response { body: ResponseBody::Message(String::from("You do not have permissions to perform that action"))};
        return Err(Unauthorized(serde_json::to_string(&response).unwrap()));
    }

    // Delete all posts associated to the user
    match diesel::delete(posts.filter(id.eq(user_id))).execute(&mut establish_connection()) {
        Ok(count) => count,
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }        
        }
    };

    // Delete user
    match diesel::delete(users.filter(users::id.eq(user_id))).execute(&mut establish_connection()) {
        Ok(count) => count,
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response { body: ResponseBody::Message(format!("Error - no user with id {}", user_id))};
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            },
            _ => {
                panic!("Database error - {}", err);
            }        
        }
    };

    // Return updated list of users
    match users::table.select(users::all_columns).load::<User>(&mut establish_connection()) {
        Ok(mut users_) => {
            users_.sort();
            Ok(users_)
        },
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}
