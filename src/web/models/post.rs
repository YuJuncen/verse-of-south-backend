use chrono::NaiveDateTime;
use crate::database::models::post::FormatType;
pub use FormatType;
use std::sync::Arc;

#[derive(Eq, PartialEq, Debug, Clone)]
struct Reader {
    pub ip: i64,
    pub name: String,
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct Comment {
    pub publisher: Reader,
    pub content: Arc<String>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct Tag {
    pub tag_name: String,
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct Post {
    pub id : i32,
    pub title: Arc<String>,
    pub intro: Option<String>,
    pub tags: Vec<Tag>,
    pub comments: Vec<Comment>,
    pub publish_time: NaiveDateTime,
    pub body: PostBody
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct PostBody {
    pub content: Arc<String>,
    pub format_type: FormatType,
}

struct NewPost<'a> {
    pub title: &'a str,
    pub intro: Option<&'a str>,
    pub tags: Vec<Tag>,
    pub body: PostBody
}