use chrono::NaiveDateTime;

#[derive(Serialize, Debug, Eq, PartialEq)]
pub struct Tag{
    pub name: String,
}

#[derive(Serialize, Debug, Eq, PartialEq)]
pub struct Post {
    pub title: String, 
    pub publish_time: NaiveDateTime,
    pub intro: Option<String>,
    pub tags: Vec<Tag>
}