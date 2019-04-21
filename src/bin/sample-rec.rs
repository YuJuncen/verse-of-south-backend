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
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
    actix::run(|| {
        let ssl_conn = SslConnector::builder(SslMethod::tls()).unwrap().build();
        let conn = client::ClientConnector::with_connector(ssl_conn).start();    
        /*        
        client::post("https://www.recaptcha.net/recaptcha/api/siteverify")
        .set_header(http::header::ACCEPT, "application/json")
        .with_connector(conn)
        .timeout(std::time::Duration::from_secs(10))
        .form(json!({
            "secret": args.get(1),
            "response": args.get(2)
        }))
        .map(|req| {
            println!("REQUEST: {:?} WITH ARGS: {:?}", req, args);
            req
        })*/
        client::post(args.get(1).unwrap())
        .with_connector(conn)
        .form(json!({
            "secret": args.get(2),
            "response": args.get(3)
        }))
        .unwrap()
        .send()
        .map_err(|e| println!("failed: {:?}", e))
        .map(|r| {
            println!("RECEIVED: {:?}", r);
            r
        })
        .and_then(|r| r.json::<Value>().map(|i| println!("BODY: {:?}", i)).map_err(|e| println!("Error: {:?}", e)))
        .map_err(|_| ())
    })
}