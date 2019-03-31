use actix::prelude::*;
use chrono::prelude::*;
use crate::wrapper::messages::*;
use crate::web::handlers::post::*;
use crate::web::models::index_post::*;

pub struct Index {

}

impl Actor for Index {
    type Context = SyncContext<Self>;
}

impl Handler<GiveMePostOfPage> for Index {
    type Result = Result<Vec<Post>, ()>;
    fn handle(&mut self, msg: GiveMePostOfPage, _ctx: &mut Self::Context) -> Self::Result {
        Ok(
            vec![Post {
            title: String::from("“Promise” for you."),
            intro: None,
            tags: vec![],
            publish_time: Utc::now().naive_utc(),
        }, Post {
            title: String::from("启航之日。"),
            intro: Some(format!["OFFSET: {:?} LIMIT: {:?}", msg.page.limit, msg.page.offset]),
            tags: vec![Tag {name: "OFFSET".to_string()}, Tag {name: "LIMIT".to_string()}],
            publish_time: Utc::now().naive_utc(),
        }]
        )
    } 
}
