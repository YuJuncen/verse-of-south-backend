pub mod handlers;
pub mod models;

use serde::ser::{ Serialize, Serializer, SerializeSeq };
use crate::wrapper::actors::pgdatabase::PGDatabase;
use std::ops::Deref;
use actix::prelude::*;
use std::hash::Hash;

pub mod middlewares {
    use actix_web::*;
    use actix_web::middleware::{ Middleware, Started };
    use super::errors::Unauthorized;
    pub struct CommentFilter;
    impl <S> Middleware<S> for CommentFilter {

        fn start(&self, req: &HttpRequest<S>) -> Result<Started> {
            let auth = req.headers().get(http::header::AUTHORIZATION);
            match auth {
                Some(_) => Ok(Started::Done),
                None => Err(Unauthorized{}.into())
            }
        }
    }
}

pub mod errors {
    use actix_web::*;
    use crate::wrapper::actors::pgdatabase::DatabaseError;

    #[derive(Fail, Debug)]
    #[fail(display = "fail to authorize.")]
    pub struct Unauthorized;
    
    impl error::ResponseError for Unauthorized {
        fn error_response(&self) -> HttpResponse {
            HttpResponse::new(http::StatusCode::FORBIDDEN)
        }
    }

    impl error::ResponseError for DatabaseError {
        fn error_response(&self) -> HttpResponse {
            match *self {
                DatabaseError::DieselGoesWrong(diesel::result::Error::NotFound) => HttpResponse::build(http::StatusCode::NOT_FOUND).json(self),
                _ => HttpResponse::build(http::StatusCode::INTERNAL_SERVER_ERROR).json(self)
            }
        }
    }
}

pub struct AppState {
    pub database: Addr<PGDatabase>
}

pub fn deserizlize_pointer<T: Serialize, S: Serializer, P: Deref<Target=T>>(p: &P,  s: S) -> Result<S::Ok, S::Error> {
    p.serialize(s)
}

pub fn serialize_real_to_integer<S: Serializer>(p: &f64, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_i64(*p as i64)
}

pub fn serialize_vec_of_pointer<
    T: Serialize, 
    S: Serializer, 
    P: Deref<Target=T>, 
    >(ps: &Vec<P>, s: S) -> Result<S::Ok, S::Error> {
    let i = ps.iter();
    let mut seq = s.serialize_seq(i.size_hint().1)?;
    for ele in i {
        seq.serialize_element(&**ele)?;
    }
    seq.end()
}


pub fn serialize_set_of_pointer<
    T: Serialize, 
    S: Serializer, 
    P: Deref<Target=T> + Eq + Hash, 
    >(ps: &std::collections::HashSet<P>, s: S) -> Result<S::Ok, S::Error> {
    let i = ps.iter();
    let mut seq = s.serialize_seq(i.size_hint().1)?;
    for ele in i {
        seq.serialize_element(&**ele)?;
    }
    seq.end()
}