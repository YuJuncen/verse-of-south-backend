use std::time::Instant;

#[derive(Debug, Eq, PartialEq)]
pub enum FormatType {
    Markdown, HTML, PlainText, WriteDone
}

#[derive(Queryable, Debug, Eq, PartialEq)]
pub struct Post<'a> {
    pub id: i32 ,  
    pub title: &'a str ,
    pub intro: Option<&'a str> ,
    pub publish_time: Instant ,
    pub body: &'a str ,
    pub body_format: FormatType ,
}