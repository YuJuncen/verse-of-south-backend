use vos::database::models::post::{ NewPost, Post, FormatType };
use vos::database;
use std::time::SystemTime;
use chrono::prelude::*;
use diesel::prelude::*;
fn main() {
    use vos::schema::posts::dsl::*;
    let post = Post {
        id: 0,
        title: String::from("Hello, world!"),
        intro: Option::None,
        publish_time: Utc::now().naive_utc(),
        body: String::from("# Hello!  \nKokowa verse of south!"),
        body_format: FormatType::Markdown
    };
    let np = NewPost::new("Hi!", "新的冒险要开始了！", None);
    let conn = database::establish_connection();
    let res = diesel::insert_into(posts)
        .values(&np)
        .get_result::<Post>(&conn);
        
    println!("{:?} //\n {:?}// \n {:?}", post, np, res);
}