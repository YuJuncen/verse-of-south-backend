extern crate actix_web;
#[macro_use]
extern crate serde_json;
extern crate openssl;
extern crate actix;
use openssl::ssl::{ SslConnector, SslMethod };
use std::env;
use serde_json::Value;
use actix_web::*;
use futures::future::Future;
use actix::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    actix::run(|| {
        let ssl_conn = SslConnector::builder(SslMethod::tls()).unwrap().build();
        let client_conn = client::ClientConnector::with_connector(ssl_conn).start();
        client::post("https://www.recaptcha.net/recaptcha/api/siteverify")
        .with_connector(client_conn)
        .timeout(std::time::Duration::from_secs(10))
        .form(json!({
            "secret": args[1],
            "response": args[2]
        }))
        .map(|req| {
            println!("REQUEST: {:?} WITH ARGS: {:?}", req, args);
            req
        })
        .unwrap()
        .send()
        .map_err(|e| println!("failed: {:?}", e))
        .and_then(|r| r.json::<Value>().and_then(|i| {
            println!("SUCCESS: {:?}", i);
            Ok(())
        }).map_err(|e| println!("FAILED: {:?}", e)))
        .map_err(|e| println!("FAILED: {:?}", e))
    })
}