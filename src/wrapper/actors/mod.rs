pub mod index;
pub mod post_actor;
pub mod pgdatabase;
pub mod database;
pub mod new_post_actor;
use serde::ser::{ Serializer, Serialize };
use actix::MailboxError;
use futures::future::*;

fn serialize_intostr<T: ToString, S: Serializer>(s: &T, ser: S) -> Result<S::Ok, S::Error>{
    s.to_string().serialize(ser)
}

pub type ManagerResult<T> = Result<T, ManagerError>;

#[derive(Debug, Serialize)]
#[serde(tag = "failed", content = "info")]
pub enum ManagerError{
    #[serde(serialize_with = "serialize_intostr")]    
    DieselGoesWrong(diesel::result::Error),
    #[serde(serialize_with = "serialize_intostr")]
    ActorSystemGoesWrong(MailboxError),
    SomethingGoesWrong(String),
}

impl From<String> for ManagerError {
    fn from(s: String) -> Self {
        ManagerError::SomethingGoesWrong(s)
    }
}

impl From<MailboxError> for ManagerError {
    fn from(mbe: MailboxError) -> Self {
        ManagerError::ActorSystemGoesWrong(mbe)
    }
}

impl From<diesel::result::Error> for ManagerError {
    fn from(dbe: diesel::result::Error) -> Self {
        ManagerError::DieselGoesWrong(dbe)
    }
}

pub fn merge_error<I, E: From<E1> + From<E2>, E1, E2>(f: impl Future<Item=Result<I, E1>, Error=E2>) -> impl Future<Item=I, Error=E> {
    f.map(|i| i.map_err(Into::into)).from_err().and_then(result)
}