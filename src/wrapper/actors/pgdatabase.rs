use crate::web::handlers::types::Pagination;
use diesel::prelude::*;
use actix::prelude::*;
use serde::ser::{ Serialize, Serializer};
use crate::database::models::tag as T;
use crate::wrapper::messages::*;
use crate::web::models::index_post::*;
use crate::database::models as M;
use crate::web::models::detailed_post::*;
use crate::database::models::comment::*;
use crate::database::models::types::ArchiveInfo;
use std::fmt;

fn serialize_intostr<T: ToString, S: Serializer>(s: &T, ser: S) -> Result<S::Ok, S::Error>{
    s.to_string().serialize(ser)
}

#[fail(display = "when query, something wrong happens.")]
#[derive(Fail, Debug, Serialize)]
pub enum DatabaseError{
    #[serde(serialize_with = "serialize_intostr")]    
    DieselGoesWrong(diesel::result::Error),
    #[serde(serialize_with = "serialize_intostr")]
    ActorSystemGoesWrong(MailboxError),
    Because(String),
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "when query, something goes wrong.")
    }
}

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
    fn started(&mut self, _ctx: &mut Self::Context) {
        info!("database started; testing sql execution: {:?}", self.connection.execute("SELECT 1;"));
    }
}

impl Handler<GiveMePostOfPage> for PGDatabase {
    type Result = Result<Vec<Post>, DatabaseError>;
    fn handle(&mut self, msg: GiveMePostOfPage, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::posts::dsl::*;
        let ps = posts
            .limit(msg.page.limit.unwrap_or(i64::max_value()))
            .offset(msg.page.offset)
            .order_by(publish_time.desc())
            .load::<M::post::Post>(&self.connection)?;
        M::post::Post::batch_into_index_post(ps, &self.connection)    
    }
}

impl Into<crate::web::models::comment::Comment> for Comment {
    fn into(self) -> crate::web::models::comment::Comment {
        use crypto::md5::Md5;
        use crypto::digest::Digest;
        let mut hasher = Md5::new();
        let mut hashed_email = [0u8; 16];
        debug!("Hashing Email address :: origin: {:?}", self.publisher_email);
        let email_addr = self.publisher_email.map(|e| {
            hasher.input(e.to_lowercase().as_bytes());
            hasher.result(&mut hashed_email);
            debug!("Hashed Email address :: {:?}", hashed_email);
            hashed_email.iter().map(|d| format!["{:0>2x}", d]).collect()
        });
        crate::web::models::comment::Comment {
                id: self.id,
                reply_to: self.reply_to,
                publish_time: self.publish_time,
                publisher_name: self.publisher_name,
                content: self.content,
                publisher_email: email_addr
        }
    }
}

macro_rules! query_to_sql {
    ($msg: ident) => {
        {use diesel::dsl::*;
        use diesel::sql_types::{ BigInt, Text };
        sql_query("
        SELECT * FROM posts P1 WHERE 
            NOT EXISTS
            ((SELECT tags.id FROM tags WHERE tag_name = ANY (string_to_array($1, ':')::text[]))
            EXCEPT 
            (SELECT tag_id FROM posts P2 INNER JOIN tag_to ON post_id = P2.id WHERE P1.id = P2.id))
        AND
            P1.title LIKE ALL ( string_to_array($2, ' ')::text[] )
        ORDER BY publish_time DESC
        LIMIT $3 OFFSET $4")
        .bind::<Text, String>($msg.tags.into_iter().map(|t| t.name).collect::<Vec<String>>().join(":"))
        .bind::<Text, _>($msg.title.map(|t| t.trim().split(' ').map(|c| format!("%{}%", c)).collect::<Vec<_>>().join(" ")).unwrap_or_default())
        .bind::<BigInt, _>($msg.page.limit.map(|i| i as i64).unwrap_or(i64::max_value()))
        .bind::<BigInt, _>($msg.page.offset as i64)}
    };
}

impl Handler<GiveMePostOfPageMatches> for PGDatabase {
    type Result = Result<Vec<Post>, DatabaseError>;
    fn handle(&mut self, msg: GiveMePostOfPageMatches, _ctx: &mut Self::Context) -> Self::Result {
        use crate::database::models::post::Post as DPost;

        let mps = query_to_sql!(msg);
        debug!("{:?}", diesel::debug_query::<diesel::pg::Pg, _>(&mps));
        let result = mps.load::<DPost>(&self.connection);
        debug!("{:?}", result);
        result
            .map_err(|e : diesel::result::Error| e.into())
            .and_then(|v| M::post::Post::batch_into_index_post(v, &self.connection))
    }
}

impl Handler<GiveMeFullPostOfId> for PGDatabase {
    type Result = Result<DetailedPost, DatabaseError>;
    fn handle(&mut self, msg: GiveMeFullPostOfId, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::posts::dsl::*;
        use crate::database::models::comment::Comment;
        let post = posts.filter(id.eq(msg.0)).get_result::<crate::database::models::post::Post>(&self.connection)?;
        let comments = Comment::belonging_to(&post).load::<Comment>(&self.connection)?;
        let base_post = post.into_index_post(&self.connection)?;
        Ok(DetailedPost {
            base: base_post,
            content: std::sync::Arc::new(post.body),
            comments: comments.into_iter().map(|p| p.into()).collect(),
            format_type: post.body_format,
        })
    }
}

impl Handler<CommentToPost> for PGDatabase {
    type Result = Result<crate::web::models::comment::Comment, DatabaseError>;
    fn handle(&mut self, msg: CommentToPost, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::comments::dsl::*;
        let comment_to_post = NewComment::new(
            // 在数据库附近这样做真的好吗？
            // 鉴定是否出问题的责任完全到前端去了。
            // 但是 actix 似乎要求静态的消息；
            // 也就是说，似乎没办法把这个消息中的字符串给换成 &str。
            // 也许 Arc 真的是几乎唯一的解决方案了……
            // 但是这样重构的话很不容易。
            &msg.content[..], 
            &msg.publisher[..], 
            msg.publisher_email.as_ref().map(|s| &s.as_str()[..]), 
            msg.to, msg.reply_to);
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
            ORDER BY publish_time DESC
            OFFSET $3 LIMIT $4;
        ").bind::<Integer, _>(msg.year)
        .bind::<Integer, _>(msg.month)
        .bind::<BigInt, _>(msg.page.offset)
        .bind::<BigInt, _>(msg.page.limit.unwrap_or(i64::max_value()));
        debug!("selecting archives by sql: {:?}", diesel::debug_query::<diesel::pg::Pg, _>(&posts));
        posts
            .load::<DPost>(&self.connection)
            .map_err(|e| e.into())
            .and_then(|v| M::post::Post::batch_into_index_post(v, &self.connection))
    }
}

impl Handler<GiveMeAllTags> for PGDatabase {
    type Result = Result<Vec<Tag>, DatabaseError>;
    fn handle(&mut self, _msg: GiveMeAllTags, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::tags::table;
        let tags = table.load::<T::Tag>(&self.connection)?;
        Ok(tags.into_iter().map(|t| Tag {name: t.tag_name}).collect())
    }
}

impl Handler<GiveMePaginationOf<GiveMePostOfPageMatches>> for PGDatabase {
    type Result = Result<Pagination, DatabaseError>;
    fn handle(&mut self, msg: GiveMePaginationOf<GiveMePostOfPageMatches>, _ctx: &mut Self::Context) -> Self::Result {
        use diesel::dsl::*;
        use diesel::sql_types::{ Text };
        let sql = sql_query("
        SELECT * FROM posts P1 WHERE 
            NOT EXISTS
            ((SELECT tags.id FROM tags WHERE tag_name = ANY (string_to_array($1, ':')::text[]))
            EXCEPT 
            (SELECT tag_id FROM posts P2 INNER JOIN tag_to ON post_id = P2.id WHERE P1.id = P2.id))
        AND
            P1.title LIKE ALL ( string_to_array($2, ' ')::text[] )")
        .bind::<Text, String>(msg.base_query.tags.into_iter().map(|t| t.name).collect::<Vec<String>>().join(":"))
        .bind::<Text, _>(msg.base_query.title.map(|t| t.trim().split(' ').map(|c| format!("%{}%", c)).collect::<Vec<_>>().join(" ")).unwrap_or_default());
        let count = self.connection.execute_returning_count(&sql)?;
        Ok(Pagination::new(
            msg.base_query.page.offset,
            count,
            msg.base_query.page.limit,
        ))
    }
}