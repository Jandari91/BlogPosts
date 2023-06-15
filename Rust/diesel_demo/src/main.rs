use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
mod lib;
use crate::lib::*;
use self::models::*;
use self::connection::*;

fn main() {
    dotenv().ok();

    let db_url = env::var("DB_URL").expect("DB_URL must be set");
    let connection =&mut establish_connection(&db_url);

    println!("Success Connection");
    use self::schema::posts::dsl::*;
    let results = posts
        .filter(published.eq(true))
        .limit(5).select(Post::as_select()).load(connection).expect("Err loading posts");

    println!("Displaying {} posts", results.len());
}
