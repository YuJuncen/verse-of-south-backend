use diesel::prelude::*;
use actix::prelude::*;
use serde::ser::{ Serialize, Serializer};
use crate::database::models::tag as T;
use crate::wrapper::messages::*;
use crate::web::models::index_post::*;
use crate::database::models as M;
use crate::web::models::detailed_post::*;
use crate::database::models::comment::*;

fn serialize_intostr<T: ToString, S: Serializer>(s: &T, ser: S) -> Result<S::Ok, S::Error>{
    s.to_string().serialize(ser)
}

#[derive(Debug, Serialize)]
pub enum DatabaseError{
    #[serde(serialize_with = "serialize_intostr")]    
    DieselGoesWrong(diesel::result::Error),
    #[serde(serialize_with = "serialize_intostr")]
    ActorSystemGoesWrong(MailboxError),
    Because(String),
}

pub struct PGDatabase {
    connection: PgConnection,
}

impl PGDatabase {
    pub fn new(c: PgConnection) -> PGDatabase {
        PGDatabase {connection : c}
    }
}

impl From<String> for DatabaseError {
    fn from(s: String) -> Self {
        DatabaseError::Because(s)
    }
}

impl From<MailboxError> for DatabaseError {
    fn from(mbe: MailboxError) -> Self {
        DatabaseError::ActorSystemGoesWrong(mbe)
    }
}

impl From<diesel::result::Error> for DatabaseError {
    fn from(dbe: diesel::result::Error) -> Self {
        DatabaseError::DieselGoesWrong(dbe)
    }
}

impl Actor for PGDatabase {
    type Context = SyncContext<Self>;
    fn started(&mut self, _ctx: &mut Self::Context) {
        info!("database started; testing sql execution: {:?}", self.connection.execute("SELECT 1;"));
    }
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
    type Result = Result<Vec<Post>, DatabaseError>;
    fn handle(&mut self, msg: GiveMePostOfPage, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::posts::dsl::*;
        posts.limit(msg.page.limit.into())
            .offset(msg.page.offset.into())
            .load::<M::post::Post>(&self.connection)
            .map(|v| v.into_iter().map(|p| p.into_index_post(&self.connection)).collect())
            .map_err(|e| e.to_string().into())
    }
}

impl Handler<GiveMePostOfPageMatches> for PGDatabase {
    type Result = Result<Vec<Post>, DatabaseError>;
    fn handle(&mut self, msg: GiveMePostOfPageMatches, _ctx: &mut Self::Context) -> Self::Result {
        use crate::database::models::post::Post as DPost;
        use diesel::dsl::*;
        use diesel::sql_types::{ BigInt, Text };
        let mps = sql_query("
        SELECT * FROM posts P1 WHERE 
            NOT EXISTS
            ((SELECT tags.id FROM tags WHERE tag_name = ANY (string_to_array($1, ':')::text[]))
            EXCEPT 
            (SELECT tag_id FROM posts P2 INNER JOIN tag_to ON post_id = P2.id WHERE P1.id = P2.id))
        AND
            P1.title LIKE $2
        LIMIT $3 OFFSET $4")
        .bind::<Text, String>(msg.tags.into_iter().map(|t| t.name).collect::<Vec<String>>().join(":"))
        .bind::<Text, _>(format!("%{}%", msg.title.unwrap_or_default()))
        .bind::<BigInt, _>(msg.page.limit as i64)
        .bind::<BigInt, _>(msg.page.offset as i64);
        debug!("{:?}", diesel::debug_query::<diesel::pg::Pg, _>(&mps));
        let result = mps.load::<DPost>(&self.connection);
        debug!("{:?}", result);
        result.map(|ps : Vec<DPost>| ps.into_iter().map(|p| p.into_index_post(&self.connection)).collect())
            .map_err(|e : diesel::result::Error| e.into())
    }
}

impl Handler<GiveMeFullPostOfId> for PGDatabase {
    type Result = Result<DetailedPost, DatabaseError>;
    fn handle(&mut self, msg: GiveMeFullPostOfId, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::posts::dsl::*;
        use crate::database::models::comment::Comment;
        let post = posts.filter(id.eq(msg.0)).get_result::<crate::database::models::post::Post>(&self.connection);
        post.and_then(
            |post| Comment::belonging_to(&post).load::<Comment>(&self.connection).map(
            |comment| DetailedPost {
                base: post.into_index_post(&self.connection),
                content: std::sync::Arc::new(post.body.clone()),
                comments: comment.into_iter().map(|p| p.into()).collect(),
                format_type: post.body_format,
        })).map_err(|e| e.into() )
    }
}

impl Handler<CommentToPost> for PGDatabase {
    type Result = Result<crate::web::models::comment::Comment, DatabaseError>;
    fn handle(&mut self, msg: CommentToPost, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::comments::dsl::*;
        let comment_to_post = NewComment::new(&msg.content, &msg.publisher, msg.publisher_email.as_ref().map(|s| s.as_str()), msg.to, msg.reply_to);
        diesel::insert_into(comments)
            .values(comment_to_post)
            .get_result::<Comment>(&self.connection)
            .map(|p| p.into()).map_err(|e| e.to_string().into())
    }
}

impl Handler<GiveMeArchiveInfo> for PGDatabase {
    type Result = Result<Vec<ArchiveInfo>, DatabaseError>;
    fn handle(&mut self, _msg: GiveMeArchiveInfo, _ctx: &mut Self::Context) -> Self::Result {
        diesel::sql_query("SELECT * FROM archives")
            .load::<ArchiveInfo>(&self.connection)
            .map(|infos| infos.into_iter().map(|info| ArchiveInfo {
                month: info.month,
                year: info.year,
                count: info.count
            }).collect())
            .map_err(|e| e.into())
    }
}

impl Handler<GiveMeArchiveOf> for PGDatabase {
    type Result = Result<Vec<Post>, DatabaseError>;
    fn handle(&mut self, msg: GiveMeArchiveOf, _ctx: &mut Self::Context) -> Self::Result {
        use crate::database::models::post::Post as DPost;
        use diesel::sql_types::{BigInt, Integer};
        let posts = diesel::sql_query("
        SELECT * FROM posts WHERE EXTRACT(YEAR FROM publish_time) = $1 AND EXTRACT(MONTH FROM publish_time) = $2
            OFFSET $3 LIMIT $4;
        ").bind::<Integer, _>(msg.year)
        .bind::<Integer, _>(msg.month)
        .bind::<BigInt, _>(msg.page.offset)
        .bind::<BigInt, _>(msg.page.limit);
        println!("{:?}", diesel::debug_query::<diesel::pg::Pg, _>(&posts));
        posts
            .load::<DPost>(&self.connection)
            .map(|ps| ps.into_iter().map(|p| p.into_index_post(&self.connection)).collect())
            .map_err(|e| e.into())
    }
}