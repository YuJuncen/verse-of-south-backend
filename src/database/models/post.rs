use std::time::{Instant, SystemTime};
use std::io;
use diesel::sql_types::*;
use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, ToSql, Output};
use chrono::{NaiveDateTime, Utc};
use crate::schema::posts;

#[derive(Debug, Eq, PartialEq, AsExpression, FromSqlRow)]
#[sql_type = "SmallInt"]
pub enum FormatType {
    Markdown, HTML, PlainText, WriteDone
}

impl <DB: Backend> ToSql<SmallInt, DB> for FormatType
    where i16: ToSql<SmallInt, DB>,
{
    fn to_sql<W: io::Write>(&self, out : &mut Output<W, DB>) -> serialize::Result {
        let v = match *self {
            FormatType::Markdown  => 1,
            FormatType::HTML      => 2,
            FormatType::PlainText => 3,
            FormatType::WriteDone => 4,
        };
        v.to_sql(out)
    }
}

impl <DB: Backend> FromSql<SmallInt, DB> for FormatType 
    where i16: FromSql<SmallInt, DB>, {
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        let v = i16::from_sql(bytes)?;
        Ok(match v {
            1 => FormatType::Markdown,
            2 => FormatType::HTML,
            3 => FormatType::PlainText,
            4 => FormatType::WriteDone,
            _ => unreachable!()
        })
    }
}

#[derive(Queryable, Debug, Eq, PartialEq)]
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