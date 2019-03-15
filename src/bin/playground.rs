use std::time::Instant;
use verse_of_south_backend::database::models::post::{ Post, FormatType };
use diesel::prelude::*;
use verse_of_south_backend::schema::posts::dsl::*;
fn main() {
    let post = Post {
        id: 0,
        title: "Hello, world!",
        intro: Option::None,
        publish_time: Instant::now(),
        body: "# Hello!  \nKokowa verse of south!",
        body_format: FormatType::Markdown
    };
    println!("{:?}", post)
}