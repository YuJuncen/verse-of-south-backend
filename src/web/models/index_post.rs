use chrono::NaiveDateTime;
use std::sync::Arc;
use crate::web::{ serialize_vec_of_pointer };

#[derive(Serialize, Debug, Eq, PartialEq, Hash, Clone)]
#[serde(rename_all="camelCase")]
pub struct Tag{
    pub name: String,
}

#[derive(Serialize, Debug, Eq, PartialEq)]
#[serde(rename_all="camelCase")]
pub struct Post {
    pub id: i32,
    pub title: String, 
    pub publish_time: NaiveDateTime,
    pub intro: Option<String>,
    #[serde(serialize_with = "serialize_vec_of_pointer")]
    pub tags: Vec<Arc<Tag>>
}