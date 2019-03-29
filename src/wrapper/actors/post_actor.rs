use actix::prelude::*;
use chrono::prelude::*;
use crate::web::handlers::post::*;
use crate::web::models::index_post::*;
use crate::web::models::detailed_post::*;

pub struct PostActor {}

impl Actor for PostActor {
    type Context = SyncContext<Self>;
}

impl Handler<PostIdQuery> for PostActor {
    type Result = Result<DetailedPost, ()>;
    fn handle(&mut self, _msg: PostIdQuery, _: &mut Self::Context) -> Self::Result {
        Ok(DetailedPost {
            base: Post {
                title: String::from("“Promise” for you."),
                intro: None,
                tags: vec![],
                publish_time: Utc::now().naive_utc(),
            },
            content: "# Sample  \nHello, world!".to_string(),
            format_type: FormatType::Markdown,
            comments: vec![],
        })
    }
}