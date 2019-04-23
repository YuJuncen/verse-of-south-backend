#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_json;
extern crate futures;
extern crate actix;
extern crate dotenv;
extern crate env_logger;
extern crate crypto;
extern crate clap;

pub mod database;
pub mod web;
pub mod wrapper;
pub mod schema;