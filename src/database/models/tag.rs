use crate::schema::{tags, tag_to};
use super::post::Post;


#[table_name = "tags"]
#[derive(Identifiable, Queryable, Eq, PartialEq, Debug)]
pub struct Tag {
    pub tag_name : String ,
    pub id : i32 ,
}

#[table_name = "tag_to"]
#[primary_key(tag_id, post_id)]
#[derive(Identifiable, Queryable, Eq, PartialEq, Debug, Associations)]
#[belongs_to(Tag)]
#[belongs_to(Post)]
pub struct TagTo {
    pub tag_id: i32 ,
    pub post_id: i32 ,
}
