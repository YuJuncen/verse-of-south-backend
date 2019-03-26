use chrono::NaiveDateTime;
use std::sync::Arc;
pub use crate::database::models::types::FormatType;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Reader {
    pub ip: i64,
    pub name: String,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Comment {
    pub publisher: Reader,
    pub content: Arc<String>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Tag {
    pub tag_name: String,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct WebPost {
    pub id : i32,
    pub title: Arc<String>,
    pub intro: Option<String>,
    pub tags: Vec<Tag>,
    pub comments: Vec<Comment>,
    pub publish_time: NaiveDateTime,
    pub body: PostBody
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct PostBody {
    pub content: Arc<String>,
    pub format_type: FormatType,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub intro: Option<&'a str>,
    pub tags: Vec<Tag>,
    pub body: PostBody
}