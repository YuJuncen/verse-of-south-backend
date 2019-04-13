use super::messages::*;
use crate::web::handlers::post::*;
use crate::web::handlers::index::*;
use crate::web::handlers::comment::NewComment;
use crate::web::models::index_post::{ Tag, Post };
use crate::database::models as M;
use M::tag as T;
use super::actors::ManagerError;
use super::actors::database::*;
use super::actors::merge_error;
use diesel::Connection;
use futures::future::*;
use actix::prelude::*;
use diesel::prelude::*;

fn get_default_page_size() -> i64 { 4 }

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
        PageInfo { offset: self.offset.unwrap_or(0), limit: self.limit.unwrap_or(get_default_page_size())}
    }
}

impl Into<PageInfo> for Option<PageQuery> {
    fn into(self) -> PageInfo {
        self.map(Into::into).unwrap_or( PageInfo {offset: 0, limit: get_default_page_size()} )
    }
}

impl Into<GiveMePostOfPageMatches> for PredicateQuery {
    fn into(self) -> GiveMePostOfPageMatches {
        GiveMePostOfPageMatches {
                page: self.page.into(),
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

impl M::post::Post {
    pub fn get_tags<C, Tr, D, Dsl>(&self, database: Addr<Database<C>>) -> impl Future<Item=Vec<T::Tag>, Error=ManagerError> where 
    C: Connection<TransactionManager=Tr, Backend=diesel::pg::Pg> + 'static,
    Tr: diesel::connection::TransactionManager<C> {
        use diesel::dsl::any;
        use crate::schema::{ tags, tag_to };
        use diesel::prelude::*;

        let post_tag_ids = T::TagTo::belonging_to(self).select(tag_to::tag_id);

        merge_error(database.send(LoadByDsl::new(tags::table.filter(tags::id.eq(any(post_tag_ids))))))
    }

    pub fn into_index_post(&self, database: Addr<Database<PgConnection>>) -> impl Future<Item=Post, Error=ManagerError> {
        self.get_tags(database).map(|ts| {
        Post {
            title: self.title.clone(),
            publish_time: self.publish_time.clone(),
            intro: self.intro.clone(),
            tags: ts.into_iter().map(|t| std::sync::Arc::new(Tag {name: t.tag_name})).collect()
        }})
    }

    pub fn batch_into_index_post(sf: Vec<Self>, database: Addr<Database<PgConnection>>) -> impl Future<Item=Vec<Post>, Error=ManagerError> {
        use std::collections::BTreeMap;
        use std::sync::Arc;
        use crate::schema::{ tags, tag_to };
        use diesel::dsl::any;
        let post_tag_info = merge_error(database.send(LoadByDsl::new(T::TagTo::belonging_to(&sf)))).map(|ts: Vec<T::TagTo>| ts.grouped_by(&sf));
            
        let post_tag_ids = T::TagTo::belonging_to(&sf)
            .select(tag_to::dsl::tag_id)
            .distinct();

        let tag_id_mapping = 
            merge_error(database.send(LoadByDsl::new(tags::table
            .filter(tags::dsl::id.eq(any(post_tag_ids))))))
            .map(|ts: Vec<T::Tag>| ts
                .into_iter()
                .map(|t| (t.id, Arc::new(Tag {name : t.tag_name})))
                .collect::<BTreeMap<_, _>>());
        
        let not_found = Arc::new(Tag{name: "<Tag not found>".to_string()});

        tag_id_mapping.and_then(|tag_id_mapping|
        post_tag_info.and_then(|post_tag_info| sf.into_iter().zip(post_tag_info).map(|(p, ts)| Post {
            title: p.title,
            publish_time: p.publish_time,
            intro: p.intro,
            tags: ts.into_iter().map(|tt| {
                tag_id_mapping.get(&tt.tag_id).map(Clone::clone).unwrap_or(not_found.clone()).clone()
            }).collect()
        }).collect()))
    }
}

