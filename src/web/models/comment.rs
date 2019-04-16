use chrono::prelude::*;


#[derive(Serialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct Comment {
    pub publisher_name: String,
    pub publish_time: NaiveDateTime,
    pub publisher_email: Option<String>,
    pub content: String,
    pub reply_to: Option<i32>,
    pub id: i32,
}