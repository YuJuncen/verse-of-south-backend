use actix::prelude::*;
use futures::future::*;
use diesel::PgConnection;
use diesel::prelude::*;
use crate::wrapper::messages::*;
use crate::web::models::detailed_post::DetailedPost;
use crate::web::models::comment::Comment;
use crate::wrapper::actors::database::{ Database, FirstByDsl, LoadByDsl };
use super::{ ManagerError, merge_error };
use crate::wrapper::converting::*;
use crate::database::models as M;

pub struct PostManager {
    database: Addr<Database<PgConnection>>
}

impl Actor for PostManager {
    type Context = Context<Self>;
}

impl Handler<GiveMeFullPostOfId> for PostManager {
    type Result = Box<Future<Item=DetailedPost, Error=ManagerError>>;
    fn handle(&mut self, msg: GiveMeFullPostOfId, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::posts::dsl::*;
        use crate::database::models::comment::Comment;

        let post = merge_error(self.database.send(FirstByDsl::new(posts.filter(id.eq(msg.0)))));
        Box::new(post.and_then(|post: M::post::Post| {
        post.into_index_post(self.database.clone()).and_then(|base_post|
        merge_error(self.database.send(LoadByDsl::new(Comment::belonging_to(&post)))).map(|comments: Vec<M::comment::Comment>| 
            DetailedPost {
                base: base_post,
                content: std::sync::Arc::new(post.body),
                comments: comments.into_iter().map(|p| p.into()).collect(),
                format_type: post.body_format,
            }
        ))}))
    }
}

impl Handler<CommentToPost> for PostManager {
    type Result = Box<Future<Item=crate::web::models::comment::Comment, Error=ManagerError>>;
    fn handle(&mut self, msg)
}