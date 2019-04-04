use diesel::prelude::*;
use actix::prelude::*;
use crate::database::models::tag as T;
use crate::wrapper::messages::*;
use crate::web::models::index_post::*;
use crate::database::models as M;
use crate::web::models::detailed_post::*;
use crate::database::models::comment::*;

pub struct PGDatabase {
    connection: PgConnection,
}

impl PGDatabase {
    pub fn new(c: PgConnection) -> PGDatabase {
        PGDatabase {connection : c}
    }
}

impl Actor for PGDatabase {
    type Context = SyncContext<Self>;
}

impl M::post::Post {
    pub fn get_tags<C, Tr>(&self, conn: &C) -> Vec<T::Tag> where 
    C: Connection<TransactionManager=Tr, Backend=diesel::pg::Pg>,
    Tr: diesel::connection::TransactionManager<C> {
        use diesel::dsl::any;
        use crate::schema::{ tags, tag_to };
        use diesel::prelude::*;

        let post_tag_ids = T::TagTo::belonging_to(self).select(tag_to::tag_id);

        tags::table.filter(tags::id.eq(any(post_tag_ids)))
            .load::<T::Tag>(conn)
            .expect("failed to load tags")
    }

    fn into_index_post(&self, conn: &PgConnection) -> Post {
        Post {
            title: self.title.clone(),
            publish_time: self.publish_time.clone(),
            intro: self.intro.clone(),
            tags: self.get_tags(conn).into_iter().map(|t| Tag {name: t.tag_name}).collect()
        }
    }
}

impl Into<crate::web::models::comment::Comment> for Comment {
    fn into(self) -> crate::web::models::comment::Comment {
        crate::web::models::comment::Comment {
                reply_to: self.reply_to,
                publish_time: self.publish_time,
                publisher_name: self.publisher_name,
                content: self.content,
                publisher_email: self.publisher_email
        }
    }
}

impl Handler<GiveMePostOfPage> for PGDatabase {
    type Result = Result<Vec<Post>, ()>;
    fn handle(&mut self, msg: GiveMePostOfPage, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::posts::dsl::*;
        posts.limit(msg.page.limit.into())
            .offset(msg.page.offset.into())
            .load::<M::post::Post>(&self.connection)
            .map(|v| v.into_iter().map(|p| p.into_index_post(&self.connection)).collect())
            .map_err(|_| ())
    }
}

impl Handler<GiveMeFullPostOfId> for PGDatabase {
    type Result = Result<DetailedPost, ()>;
    fn handle(&mut self, msg: GiveMeFullPostOfId, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::posts::dsl::*;
        use crate::database::models::comment::Comment;
        let post = posts.filter(id.eq(msg.0)).get_result::<crate::database::models::post::Post>(&self.connection).expect("failed to load post.");
        let comments = Comment::belonging_to(&post);
        let result = DetailedPost {
            base: post.into_index_post(&self.connection),
            content: std::sync::Arc::new(post.body.clone()),
            comments: comments.load::<Comment>(&self.connection).expect("failed to load comments.").into_iter().map(|p| p.into()).collect(),
            format_type: post.body_format,
        };
        Ok(result)
    }
}

impl Handler<CommentToPost> for PGDatabase {
    type Result = Result<crate::web::models::comment::Comment, ()>;
    fn handle(&mut self, msg: CommentToPost, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::comments::dsl::*;
        let comment_to_post = NewComment::new(&msg.content, &msg.publisher, msg.publisher_email.as_ref().map(|s| s.as_str()), msg.to, msg.reply_to);
        diesel::insert_into(comments)
            .values(comment_to_post)
            .get_result::<Comment>(&self.connection)
            .map(|p| p.into()).map_err(|_| ())
    }
}