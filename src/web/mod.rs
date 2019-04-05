pub mod handlers;
pub mod models;

use crate::wrapper::actors::index::Index;
use crate::wrapper::actors::post_actor::PostActor;

use actix::prelude::*;

pub struct AppState {
    pub index: Addr<Index>,
    pub post: Addr<PostActor>,
}