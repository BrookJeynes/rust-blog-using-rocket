use crate::schema::{posts, users};
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use std::cmp::{Ord, Eq, PartialOrd, PartialEq};

#[derive(Queryable, Serialize, Ord, Eq, PartialEq, PartialOrd)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub genre: String,
    pub published: bool,
    pub user_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost {
    pub user_id: i32,
    pub title: String,
    pub body: String,
    pub genre: String,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NewPostRequest {
    pub title: String,
    pub body: String,
    pub genre: String,
}

#[derive(Insertable, Queryable, Serialize, Ord, Eq, PartialEq, PartialOrd)]
pub struct User {
    pub id: i32,
    pub user_name: String,
    pub password: String,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = users)]
pub struct NewUser {
    pub user_name: String,
    pub password: String,
}
