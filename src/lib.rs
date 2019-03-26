#[macro_use]
extern crate diesel;
#[macro_use]
extern crate futures;
extern crate actix;
extern crate dotenv;

pub mod database;
pub mod web;
pub mod wrapper;
pub mod schema;
