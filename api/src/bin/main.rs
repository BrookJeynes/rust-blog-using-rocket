#[macro_use] extern crate rocket;
use api::{
    list_posts_handler, 
    list_post_handler, 
    create_post_handler,
    publish_post_handler,
    delete_post_handler,
};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![
            list_posts_handler::list_posts_handler, 
            list_post_handler::list_post_handler,
            create_post_handler::create_post_handler,
            publish_post_handler::publish_post_handler,
            delete_post_handler::delete_post_handler,
        ])
}

