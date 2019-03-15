use std::vec::Vec;
use std::time::Instant;
use verse_of_south_backend::database::models::{ Post, FormatType };

pub struct MockPostRepository {
    mockItems: <Post> = vec![Post {
        id: 0,
        title: "Hello, world!",
        intro: Option::None,
        publish_time: Instant::now(),
        body: "# Hello!  \nKokowa verse of south!",
        body_format: FormatType::Markdown
    }]
}

impl PostRepository for MockPostRepository {
    fn getById(&self, ID: i32) {
        return self.mockItems.find()
    }
}