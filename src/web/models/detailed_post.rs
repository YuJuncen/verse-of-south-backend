use super::index_post::Post;
use super::comment::Comment;
use actix_web::*;
use std::sync::Arc;
use crate::web::deserizlize_pointer;

pub use crate::database::models::types::FormatType;


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