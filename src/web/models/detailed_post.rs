use super::index_post::Post;
use super::comment::Comment;
pub use crate::database::models::types::FormatType;
use actix_web::*;

#[derive(Serialize, Debug)]
pub struct DetailedPost {
    pub base: Post,
    pub content: String,
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