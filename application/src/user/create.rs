use domain::models::{User, NewUser};
use shared::response_models::{Response, ResponseBody, NetworkResponse::{self, Created, Conflict}};
use infrastructure::establish_connection;
use diesel::prelude::*;
use rocket::serde::json::Json;

pub fn create_user(user: Json<NewUser>) -> Result<NetworkResponse, NetworkResponse> {
    use domain::schema::users;

    let user = user.into_inner();

    // check if user already exists
    match users::table.select(users::all_columns)
        .filter(users::user_name.eq(&user.user_name))
        .first::<User>(&mut establish_connection()) {
            Ok(_) => {
                let response = Response { body: ResponseBody::Message(String::from("Error - Username already in use"))};
                return Err(Conflict(serde_json::to_string(&response).unwrap()));
            },
            Err(err) => match err {
                diesel::result::Error::NotFound => {
                    // Do nothing - move on with function
                },
                _ => {
                    panic!("Database error - {}", err);
                }        
            }
        }

    match diesel::insert_into(users::table).values(&user).get_result::<User>(&mut establish_connection()) {
        Ok(user) => {
            let response = Response { body: ResponseBody::User(user) };
            Ok(Created(serde_json::to_string(&response).unwrap()))
        },
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}
