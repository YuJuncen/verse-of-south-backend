use chrono::{NaiveDateTime};
use super::types::FormatType;
use crate::schema::posts;


#[derive(Identifiable, Queryable, Debug, Eq, PartialEq)]
#[table_name="posts"]
pub struct Post {
    pub id: i32 ,  
    pub publish_time: NaiveDateTime,
    pub title: String,
    pub intro: Option<String>,
    pub body: String,
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