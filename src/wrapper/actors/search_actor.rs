use actix::prelude::*;
use chrono::prelude::*;
use crate::web::handlers::post::*;
use crate::web::models::index_post::*;

pub struct SearchActor {

}

impl Actor for SearchActor {
    type Context = SyncContext<Self>;
}

impl Handler<PredicateQueryMessage> for SearchActor {
    type Result = Result<Vec<Post>, ()>;
    fn handle(&mut self, msg: PredicateQueryMessage, _ctx: &mut Self::Context) -> Self::Result {
        Ok(
            vec![Post {
            title: String::from("“Promise” for you."),
            intro: None,
            tags: vec![],
            publish_time: Utc::now().naive_utc(),
        }, Post {
            title: String::from("启航之日。"),
            intro: Some(format!["TITLE: {:?} TAGS: {:?}", msg.title, msg.tags]),
            tags: vec![Tag {name: "OFFSET".to_string()}, Tag {name: "LIMIT".to_string()}],
            publish_time: Utc::now().naive_utc(),
        }]
        )
    } 
}