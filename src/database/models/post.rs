use chrono::{NaiveDateTime};
use super::types::FormatType;
use crate::schema::posts;
use diesel::pg::PgConnection;
use super::tag::{Tag, TagTo };

#[derive(Identifiable, Queryable, Debug, Eq, PartialEq)]
pub struct Post {
    pub id: i32 ,  
    pub publish_time: NaiveDateTime,
    pub title: String ,
    pub intro: Option<String> ,
    pub body: String ,
    pub body_format: FormatType ,
}

#[derive(Debug, Eq, PartialEq, Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub intro: Option<&'a str>,
    pub body: &'a str,
} 

impl<'a> NewPost<'a> {
    pub fn new(title: &'a str, body: &'a str, intro: Option<&'a str>) -> NewPost<'a> {
        NewPost {
            title, intro, body
        }
    }
}

impl Post {
    pub fn get_tags(&self, conn: &PgConnection) -> Vec<Tag> {
        use diesel::pg::expression::dsl::any;
        use crate::schema::{ tags, tag_to };
        use diesel::prelude::*;

        let post_tag_ids = TagTo::belonging_to(self).select(tag_to::tag_id);

        tags::table.filter(tags::id.eq(any(post_tag_ids)))
            .load::<Tag>(conn)
            .expect("failed to load tags")
    }

    pub fn attach_tags(tags: Vec<Tag>) {
        
    }
}