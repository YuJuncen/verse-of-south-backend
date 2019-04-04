use actix::prelude::*;
use crate::wrapper::messages::*;
use futures::future::*;
use super::pgdatabase::PGDatabase;
use crate::web::models::index_post::*;

pub struct Index {
    pub db: Addr<PGDatabase>,
}

impl Actor for Index {
    type Context = Context<Self>;
}

impl Handler<GiveMePostOfPage> for Index {
    type Result = Box<Future<Item=Vec<Post>, Error=()>>;
    fn handle(&mut self, msg: GiveMePostOfPage, _ctx: &mut Self::Context) -> Self::Result {
        Box::new(self.db.send(msg).map_err(|_| ()).and_then(|i| match i {
            Ok(o) => ok(o),
            Err(e) => err(e)
        }))
    }
}
