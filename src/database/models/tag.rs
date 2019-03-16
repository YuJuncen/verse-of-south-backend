use crate::schema::{tags, tag_to};
use super::post::Post;

#[table_name = "tags"]
#[derive(Identifiable, Queryable, Eq, PartialEq, Debug)]
pub struct Tag {
    tag_name : String ,
    id : i32 ,
}

#[table_name = "tag_to"]
#[primary_key(tag_id, post_id)]
#[derive(Identifiable, Queryable, Eq, PartialEq, Debug, Associations)]
#[belongs_to(Tag)]
#[belongs_to(Post)]
pub struct TagTo {
    tag_id: i32 ,
    post_id: i32 ,
}
