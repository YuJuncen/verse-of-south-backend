use actix::MailboxError;
use crate::wrapper::actors::pgdatabase::DatabaseError;
use super::messages::*;
use crate::web::handlers::post::*;
use crate::web::handlers::types::*;
use crate::web::models::index_post::Tag;
use crate::database::models::tag as T;
use crate::web::models::index_post::*;
use crate::database::models as M;
use diesel::{ Connection, pg::PgConnection };
use diesel::prelude::*;

impl M::post::Post {
    pub fn get_tags<C, Tr>(&self, conn: &C) -> Result<Vec<T::Tag>, DatabaseError> where 
    C: Connection<TransactionManager=Tr, Backend=diesel::pg::Pg>,
    Tr: diesel::connection::TransactionManager<C> {
        use diesel::dsl::any;
        use crate::schema::{ tags, tag_to };
        use diesel::prelude::*;

        let post_tag_ids = T::TagTo::belonging_to(self).select(tag_to::tag_id);

        tags::table.filter(tags::id.eq(any(post_tag_ids)))
            .load::<T::Tag>(conn)
            .map_err(|e| e.into())
    }

    pub fn into_index_post(&self, conn: &PgConnection) -> Result<Post, DatabaseError> {
        let ts = self.get_tags(conn)?;
        Ok(Post {
            id: self.id,
            title: self.title.clone(),
            publish_time: self.publish_time.clone(),
            intro: self.intro.clone(),
            tags: ts.into_iter().map(|t| std::sync::Arc::new(Tag {name: t.tag_name})).collect()
        })
    }

    pub fn batch_into_index_post(sf: Vec<Self>, conn: &PgConnection) -> Result<Vec<Post>, DatabaseError> {
        use std::collections::BTreeMap;
        use std::sync::Arc;
        use crate::schema::{ tags, tag_to };
        use diesel::dsl::any;
        let post_tag_info = T::TagTo::belonging_to(&sf)
            .load::<T::TagTo>(conn)?
            .grouped_by(&sf);
            
        let post_tag_ids = T::TagTo::belonging_to(&sf)
            .select(tag_to::dsl::tag_id)
            .distinct();

        let tag_id_mapping : BTreeMap<_, _> = tags::table
            .filter(tags::dsl::id.eq(any(post_tag_ids)))
            .load::<T::Tag>(conn)?
            .into_iter()
            .map(|t| (t.id, Arc::new(Tag {name : t.tag_name})))
            .collect();
        
        let not_found = Arc::new(Tag{name: "<Tag not found>".to_string()});

        Ok(sf.into_iter().zip(post_tag_info).map(|(p, ts)| Post {
            id: p.id,
            title: p.title,
            publish_time: p.publish_time,
            intro: p.intro,
            tags: ts.into_iter().map(|tt| {
                tag_id_mapping.get(&tt.tag_id).map(Clone::clone).unwrap_or(not_found.clone()).clone()
            }).collect()
        }).collect())
    }
}

impl Into<GiveMeFullPostOfId> for PostIdQuery {
    fn into(self) -> GiveMeFullPostOfId {
        GiveMeFullPostOfId(self.id as i32)
    }
}

impl Into<CommentToPost> for NewComment {
    fn into(self) -> CommentToPost {
        CommentToPost {
            content: self.comment,
            publisher: self.publisher_name,
            publisher_email: self.publisher_email,
            to: self.to,
            reply_to: self.reply_to,
        }
    }
}

impl Into<PageInfo> for PageQuery {
    fn into(self) -> PageInfo {
        PageInfo {
            limit: self.limit,
            offset: self.offset.unwrap_or(0)
        }
    }
}

impl Default for PageInfo {
    fn default() -> PageInfo {
        PageInfo {
            offset: 0,
            limit: None,
        }
    }
}

impl Into<GiveMePostOfPageMatches> for PredicateQuery {
    fn into(self) -> GiveMePostOfPageMatches {
        GiveMePostOfPageMatches {
            page:  PageInfo {
                limit: self.limit,
                offset: self.offset.unwrap_or(0)
            },
            title: self.title,
            tags: self.tags.map(|s| s.split(":").map(|s| Tag { name: String::from(s) }).collect()).unwrap_or(vec![]),
        }
    }
}

impl Into<GiveMePostOfPage> for PageQuery {
    fn into(self) -> GiveMePostOfPage {
        GiveMePostOfPage {
            page: self.into()
        }
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
