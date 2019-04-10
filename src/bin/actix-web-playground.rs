use ::actix::prelude::*;
use actix_web::*;
use actix_web::middleware::cors::Cors;
use vos::web::handlers::post::*;
use vos::web::handlers::comment::*;
use vos::web::handlers::index::*;
use vos::wrapper::actors::index::Index;
use vos::wrapper::actors::post_actor::PostActor;
use vos::wrapper::actors::pgdatabase::PGDatabase;
use vos::database::establish_connection;
use vos::web::AppState;
use vos::web::middlewares::*;
use futures::future::*;

struct RootActor;
impl Actor for RootActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let db = SyncArbiter::start(8, || PGDatabase::new(establish_connection()) );
        let addr = Index { db : db.clone() }.start();
        let post_addr = PostActor { db : db.clone() }.start();

        server::new(move || {
            App::with_state(AppState {index: addr.clone(), post: post_addr.clone()})
                .middleware(middleware::Logger::default())
                .middleware(Cors::default())
                .prefix("/resources")
                .resource("/", |r| r.get().with_async(hello_async))
                .scope("/index", |s| {
                    s.resource("", |r| r.get().with_async(get_by_page))
                    .resource("/query", |r| r.get().with_async(get_by_pred))
                    .nested("/archive", |is| {
                        is.resource("", |r| r.get().with_async(get_archives))
                            .resource("/{year}/{month}", |r| r.get().with_async(query_archives))
                    })
                })
                .resource("/post/{id}", |r| r.get().with_async(get_post_by_id))
                .resource("/comment", |r| {
                    r.middleware(CommentFilter {});
                    r.method(http::Method::POST)
                    .with_async(comment_to)
                })
                .finish()      
        }).bind("127.0.0.1:8000")
        .expect("Failed to bind.")
        .start();
    }
}

fn hello_async(_req: HttpRequest<AppState>) -> impl Future<Item=Result<String, Error>, Error = Error> {
    ok(Ok(String::from("Hello, this is ‘promise’ for you.")))
}

fn main() {
    env_logger::init();
    let sys = actix::System::new("verse-of-south");
    let _ = RootActor {}.start();
    sys.run();
}