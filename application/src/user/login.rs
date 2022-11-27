use domain::models::User;
use shared::{response_models::{Response, ResponseBody, NetworkResponse::{self, NotFound}}, request_models::LoginRequest};
use infrastructure::establish_connection;
use diesel::prelude::*;
use rocket::serde::json::Json;
use crate::auth::create_jwt;

pub fn login_user(user: Json<LoginRequest>) -> Result<String, NetworkResponse> {
    use domain::schema::users;

    let user = user.into_inner();

    let user: User = match users::table.select(users::all_columns)
        .filter(users::user_name.eq(&user.user_name))
        .filter(users::password.eq(&user.password))
        .first::<User>(&mut establish_connection()) {
            Ok(user) => user,
            Err(err) => match err {
                diesel::result::Error::NotFound => {
                    let response = Response { body: ResponseBody::Message(format!("Error - Wrong username or password for user {}", &user.user_name))};
                    return Err(NotFound(serde_json::to_string(&response).unwrap()));
                },
                _ => {
                    panic!("Database error - {}", err);
                }        
            }
        };

    match create_jwt(user.id) {
        Ok(token) => Ok(token),
        Err(err) => Err(NetworkResponse::BadRequest(err.to_string())),
    }
}
