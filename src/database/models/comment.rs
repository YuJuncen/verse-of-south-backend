use crate::schema::comments;
use crate::database::models::post::Post;
use chrono::prelude::*;

#[table_name = "comments"]
#[derive(Identifiable, Queryable, Debug, Eq, PartialEq, Associations)]
#[belongs_to(Post)]
pub struct Comment {
    pub id: i32,
    pub publish_time: NaiveDateTime,
    pub content: String,
    pub publisher_name: String,
    pub publisher_email: Option<String>,
    pub post_id: i32,
    pub reply_to: Option<i32>,
}


#[derive(Debug, Eq, PartialEq, Insertable)]
#[table_name = "comments"]
pub struct NewComment<'a> {
    content: &'a str,
    publisher_name: &'a str,
    publisher_email: Option<&'a str>,
    post_id: i32,
    reply_to: Option<i32>
}

impl<'a> NewComment<'a> {
    pub fn new(content: &'a str, publisher_name: &'a str, publisher_email: Option<&'a str>, is_for: i32, reply_to: Option<i32>) -> NewComment<'a> {
        NewComment {
            content, publisher_name, publisher_email, post_id: is_for, reply_to
        }        
    } 
}