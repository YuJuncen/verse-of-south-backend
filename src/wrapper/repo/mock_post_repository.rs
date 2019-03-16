use std::vec::*;
use std::time::Instant;
use super::models::{ Post, NewPost, FormatType };
use std::any::Any;

pub struct MockPostRepository {
    mock_items: Vec<Post>
}

impl Repository for MockPostRepository {
    type Entity = Post;
    type NewEntity = NewPost;
    type Key = i32;

    fn get_by_id(&mut self, id: i32) -> Option<Post> {
        self.sample.iter().find(|p| p.id == id)
    }
    fn get_all(&mut self) -> impl Iterator<Item=Post> {
        self.mock_items.iter()
    }
    fn save(&mut self, item: Self::NewEntity) {

    }
}

impl MockPostRepository {
    pub fn new() -> MockPostRepository {
        let sample = vec![Post {
            id: 0,
            title: "Hello, world!",
            intro: Option::None,
            publish_time: Instant::now(),
            body: "# Hello!  \nKokowa verse of south!",
            body_format: FormatType::Markdown
        }];
        MockPostRepository {mock_items : sample}
    }
}