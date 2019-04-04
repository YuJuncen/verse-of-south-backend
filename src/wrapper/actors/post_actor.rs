use actix::prelude::*;
use futures::future::*;
use crate::wrapper::messages::*;
use crate::web::models::detailed_post::*;
use crate::web::models::comment::Comment;
use crate::wrapper::actors::pgdatabase;

pub struct PostActor {
    pub db: Addr<pgdatabase::PGDatabase>,
}

impl Actor for PostActor {
    type Context = Context<Self>;
}

impl Handler<GiveMeFullPostOfId> for PostActor {
    type Result = Box<Future<Item=DetailedPost, Error=()>>;
    fn handle(&mut self, msg: GiveMeFullPostOfId, _: &mut Self::Context) -> Self::Result {
        Box::new(self.db.send(msg).map(|p| p.unwrap()).map_err(|_| ()))
    }
}

impl Handler<CommentToPost> for PostActor {
    type Result = Box<Future<Item=Comment, Error=()>>;
    fn handle(&mut self, msg: CommentToPost, _: &mut Self::Context) -> Self::Result {
        Box::new(self.db.send(msg).map(|p| p.unwrap()).map_err(|_| ()))
    }
}