pub mod handlers;
pub mod models;

use serde::ser::{ Serialize, Serializer, SerializeSeq };
use crate::wrapper::actors::index::Index;
use crate::wrapper::actors::post_actor::PostActor;
use actix::prelude::*;
use std::ops::Deref;

pub mod middlewares {
    use actix_web::*;
    use actix_web::middleware::{ Middleware, Started };
    use super::errors::Unauthorized;
    pub struct CommentFilter;
    impl <S> Middleware<S> for CommentFilter {
        fn start(&self, _req: &HttpRequest<S>) -> Result<Started> {
            Ok(Started::Done)
        }

        fn response(&self, req: &HttpRequest<S>, resp: HttpResponse) -> Result<middleware::Response> {
            let auth = req.headers().get(http::header::AUTHORIZATION);
            match auth {
                Some(_) => Ok(middleware::Response::Done(resp)),
                None => Err(Unauthorized{}.into())
            }
        }
    }
}

pub mod errors {
    use actix_web::*;

    #[derive(Fail, Debug)]
    #[fail(display = "fail to authorize.")]
    pub struct Unauthorized;
    
    impl error::ResponseError for Unauthorized {
        fn error_response(&self) -> HttpResponse {
            HttpResponse::new(http::StatusCode::FORBIDDEN)
        }
    }
}

pub struct AppState {
    pub index: Addr<Index>,
    pub post: Addr<PostActor>,
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