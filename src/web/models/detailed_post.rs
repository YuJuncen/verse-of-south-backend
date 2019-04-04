use super::index_post::Post;
use super::comment::Comment;
use serde::ser::{ Serialize, Serializer};
use actix_web::*;
use std::sync::Arc;

pub use crate::database::models::types::FormatType;
/* impl  Serialize for DetailedPost {
    fn serialize<Ser: Serializer>(&self, serializer: Ser) -> Result<Ser::Ok, Ser::Error>{
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("base", &self.base)?;
        map.serialize_entry("content", &*self.content)?;
        map.serialize_entry("comments", &self.comments)?;
        map.serialize_entry("format_type", &self.format_type)?;
        map.end()
    }
} */

fn deserizlize_pointer<T: Serialize, S: Serializer>(p: &std::sync::Arc<T>,  s: S) -> Result<S::Ok, S::Error> {
    p.serialize(s)
}

#[derive(Serialize, Debug)]
pub struct DetailedPost {
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