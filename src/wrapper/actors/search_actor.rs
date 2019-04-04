use actix::prelude::*;
use chrono::prelude::*;
use crate::wrapper::messages::*;
use crate::web::models::index_post::*;
use futures::future::*;

pub struct SearchActor {

}

impl Actor for SearchActor {
    type Context = Context<Self>;
}

impl Handler<GiveMePostOfPageMatches> for SearchActor {
    type Result = Box<Future<Item=Vec<Post>, Error=()>>;
    fn handle(&mut self, msg: GiveMePostOfPageMatches, _ctx: &mut Self::Context) -> Self::Result {
        Box::new(ok(
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
        ))
    } 
}