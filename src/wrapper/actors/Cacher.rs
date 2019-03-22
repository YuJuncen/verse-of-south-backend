use actix::prelude::*;

pub struct Cacher {

}

impl Actor for Cacher {
    type Context = Context<Self>;
}