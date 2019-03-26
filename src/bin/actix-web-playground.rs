use actix_web::{server, App, HttpRequest, Responder};
use futures::stream;

fn hello_async(_req: &HttpRequest) -> impl Responder {
    stream::once(Ok("Hello, this is promise for you."))
}

fn main() {
    server::new(|| {
        App::new()
            .resource("/", |r| r.f(hello_async))
            .finish()      
    }).bind("127.0.0.1:8000")
    .expect("Failed to bind.")
    .run();
}