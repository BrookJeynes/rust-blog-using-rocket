use domain::models::Post;
use infrastructure::establish_connection;
use diesel::prelude::*;

pub fn list_posts() -> Vec<Post> {
    use domain::schema::posts;

    match posts::table.select(posts::all_columns).load::<Post>(&mut establish_connection()) {
        Ok(mut posts) => {
            posts.sort();
            posts
        },
        // doesn't seem like selecting everything will throw any errors, leaving room for specific error handling just in case though
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}
