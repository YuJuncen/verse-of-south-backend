pub mod handlers;
pub mod models;

use crate::wrapper::actors::index::Index;
use crate::wrapper::actors::post_actor::PostActor;
use crate::wrapper::actors::search_actor::SearchActor;

use actix::prelude::*;

pub struct AppState {
    pub index: Addr<Index>,
    pub post: Addr<PostActor>,
    pub search: Addr<SearchActor>
}