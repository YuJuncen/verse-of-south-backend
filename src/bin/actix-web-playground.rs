use ::actix::prelude::*;
use actix_web::*;
use actix_web::middleware;
use vos::web::handlers::post::*;
use vos::web::handlers::comment::*;
use vos::wrapper::actors::index::Index;
use vos::wrapper::actors::post_actor::PostActor;
use vos::wrapper::actors::search_actor::SearchActor;
use vos::web::AppState;
use futures::future::*;

fn hello_async(_req: HttpRequest<AppState>) -> impl Future<Item=Result<String, Error>, Error = Error> {
    ok(Ok(String::from("Hello, this is ‘promise’ for you.")))
}


fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let sys = actix::System::new("verse-of-south");
    let addr = SyncArbiter::start(1, move || Index {});
    let post_addr = SyncArbiter::start(1, move || PostActor {});
    let search_addr = SyncArbiter::start(1, move || SearchActor {});

    server::new(move || {
        App::with_state(AppState {index: addr.clone(), post: post_addr.clone(), search: search_addr.clone()})
            .middleware(middleware::Logger::default())
            .resource("/", |r| r.with_async(hello_async))
            .scope("/index", |s| {
                s.resource("/", |r| r.with_async(get_by_page))
                    .resource("/query", |r| r.with_async(get_by_pred))})
            .resource("/post", |r| r.with_async(get_post_by_id))
            .resource("/comment", |r| 
                r.method(http::Method::POST)
                    .with_async(comment_to))
            .finish()      
    }).bind("127.0.0.1:8000")
    .expect("Failed to bind.")
    .start();

    let _ = sys.run();
}