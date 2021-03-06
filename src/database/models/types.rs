use std::io;
use diesel::sql_types::*;
use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, ToSql, Output};


#[derive(Copy, Debug, Eq, PartialEq, AsExpression, FromSqlRow, Clone, Serialize)]
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

#[derive(Debug, Clone, QueryableByName, Serialize)]
pub struct ArchiveInfo{
    #[column_name = "yer"]
    #[sql_type = "Double"]
    pub year: f64,
    #[column_name = "mon"]
    #[sql_type = "Double"]
    pub month: f64,
    #[column_name = "cnt"]
    #[sql_type = "BigInt"]
    pub count: i64,
}