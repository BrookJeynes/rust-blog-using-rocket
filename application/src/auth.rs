use chrono::Utc;
use rocket::http::Status;
use rocket::request::{Outcome, Request, FromRequest};
use rocket::serde::{Deserialize, Serialize};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use jsonwebtoken::errors::{Error, ErrorKind};
use shared::response_models::{Response, ResponseBody, NetworkResponse};
use std::env;
use dotenvy::dotenv;

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub subject_id: i32,
    exp: usize
}

#[derive(Debug)]
pub struct JWT {
    pub claims: Claims
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JWT {
    type Error = NetworkResponse;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, NetworkResponse> {
        fn is_valid(key: &str) -> Result<Claims, Error> {
            Ok(decode_jwt(String::from(key))?)
        }

        match req.headers().get_one("authorization") {
            None => {
                let response = Response { body: ResponseBody::Message(String::from("Error validating JWT token - No token provided"))};
                Outcome::Failure((Status::BadRequest, NetworkResponse::BadRequest(serde_json::to_string(&response).unwrap()))) 
            },
            Some(key) => match is_valid(key) {
                Ok(claims) => Outcome::Success(JWT {claims}),
                Err(err) => match &err.kind() {
                    jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                        let response = Response { body: ResponseBody::Message(format!("Error validating JWT token - Expired Token"))};
                        Outcome::Failure((Status::BadRequest, NetworkResponse::BadRequest(serde_json::to_string(&response).unwrap()))) 
                    },
                    jsonwebtoken::errors::ErrorKind::InvalidToken => {
                        let response = Response { body: ResponseBody::Message(format!("Error validating JWT token - Invalid Token"))};
                        Outcome::Failure((Status::BadRequest, NetworkResponse::BadRequest(serde_json::to_string(&response).unwrap()))) 
                    },
                    _ => {
                        let response = Response { body: ResponseBody::Message(format!("Error validating JWT token - {}", err))};
                        Outcome::Failure((Status::BadRequest, NetworkResponse::BadRequest(serde_json::to_string(&response).unwrap()))) 
                    }
                }
            },
        }
    }
}

fn decode_jwt(token: String) -> Result<Claims, ErrorKind> {
    dotenv().ok();

    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set.");
    let token = token.trim_start_matches("Bearer").trim();

    match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS512),
    ) {
            Ok(token) => Ok(token.claims),
            Err(err) => Err(err.kind().to_owned())
        }
}

pub fn create_jwt(id: i32) -> Result<String, Error> {
    dotenv().ok();

    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set.");

    let expiration = Utc::now().checked_add_signed(chrono::Duration::seconds(60)).expect("Invalid timestamp").timestamp();
    let claims = Claims {
        subject_id: id,
        exp: expiration as usize
    }; 

    let header = Header::new(Algorithm::HS512);

    encode(&header, &claims, &EncodingKey::from_secret(secret.as_bytes()))
}

