#[macro_use]
extern crate diesel;
extern crate futures;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
extern crate serde_json;
extern crate actix;
extern crate dotenv;
extern crate env_logger;

pub mod database;
pub mod web;
pub mod wrapper;
pub mod schema;