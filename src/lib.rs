#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
#[macro_use]
extern crate failure;
extern crate futures;
extern crate serde_json;
extern crate actix;
extern crate dotenv;
extern crate env_logger;

pub mod database;
pub mod web;
pub mod wrapper;
pub mod schema;