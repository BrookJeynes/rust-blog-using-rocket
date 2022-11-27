#[macro_use] extern crate rocket;
use api::{post_handler, user_handler};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api/post", routes![
            post_handler::list_posts_handler, 
            post_handler::list_post_handler,
            post_handler::create_post_handler,
            post_handler::publish_post_handler,
            post_handler::delete_post_handler,
        ])
        .mount("/api/user", routes![
            user_handler::list_users_handler, 
            user_handler::list_user_handler, 
            user_handler::list_users_posts_handler, 
            user_handler::create_user_handler,
            user_handler::delete_user_handler,
            user_handler::login_user_handler,
        ])
}

