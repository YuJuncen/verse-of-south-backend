use actix_web::{server, App, HttpRequest, Responder, HttpResponse, Error};
use futures::future::*;
use vos::web::models::index_post;
use chrono::prelude::*;

fn hello_async(_req: HttpRequest) -> impl Future<Item=Result<String, Error>, Error = Error> {
    ok(Ok(String::from("Hello, this is ‘promise’ for you.")))
}

fn post_async(_req: &HttpRequest) -> Result<HttpResponse, Error> {
    Ok(
        HttpResponse::Ok() 
        .json(index_post::Post{ title: String::from("‘promise’ for you"), intro: None, tags: vec![], publish_time: Utc::now().naive_utc()})
    )
}

fn main() {
    server::new(|| {
        App::new()
            .resource("/", |r| r.with_async(hello_async))
            .resource("/post", |r| r.f(post_async))
            .finish()      
    }).bind("127.0.0.1:8000")
    .expect("Failed to bind.")
    .run();
}