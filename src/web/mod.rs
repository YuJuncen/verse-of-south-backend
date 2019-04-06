pub mod handlers;
pub mod models;

use serde::ser::{ Serialize, Serializer, SerializeSeq };
use crate::wrapper::actors::index::Index;
use crate::wrapper::actors::post_actor::PostActor;
use actix::prelude::*;
use std::ops::Deref;

pub struct AppState {
    pub index: Addr<Index>,
    pub post: Addr<PostActor>,
}

pub fn deserizlize_pointer<T: Serialize, S: Serializer, P: Deref<Target=T>>(p: &P,  s: S) -> Result<S::Ok, S::Error> {
    p.serialize(s)
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