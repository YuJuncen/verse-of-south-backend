use super::index_post::Post;
use super::comment::Comment;
use serde::ser::{ Serialize, Serializer};
use actix_web::*;
use std::sync::Arc;

pub use crate::database::models::types::FormatType;


fn deserizlize_pointer<T: Serialize, S: Serializer>(p: &std::sync::Arc<T>,  s: S) -> Result<S::Ok, S::Error> {
    p.serialize(s)
}

#[derive(Serialize, Debug)]
pub struct DetailedPost {
    #[serde(flatten)]
    pub base: Post,
    #[serde(serialize_with = "deserizlize_pointer")]
    pub content: Arc<String>,
    pub comments: Vec<Comment>,
    pub format_type: FormatType,
}

impl Responder for DetailedPost {
    type Item = HttpResponse;
    type Error = Error;

    fn respond_to<S: 'static>(self, _: &HttpRequest<S>) -> Result<Self::Item, Self::Error> {
        Ok(HttpResponse::Ok().json(self))
    }
}