use ::actix::prelude::*;
use actix_web::*;
use actix_web::middleware::cors::Cors;
use vos::web::handlers::post::*;
use vos::web::handlers::comment::*;
use vos::web::handlers::index::*;
use vos::wrapper::actors::pgdatabase::PGDatabase;
use vos::database::establish_connection;
use vos::web::AppState;
use vos::web::middlewares::*;
use openssl::ssl::{ SslConnector, SslMethod };
use futures::future::*;

struct RootActor;
impl Actor for RootActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        let args : Vec<String> = std::env::args().collect();
        let db = SyncArbiter::start(8, || PGDatabase::new(establish_connection()) );
        server::new(move || {
            let ssl_conn = SslConnector::builder(SslMethod::tls()).unwrap().build();
            let conn = client::ClientConnector::with_connector(ssl_conn).start();   
            let app = App::with_state(AppState {database: db.clone()})
                .middleware(middleware::Logger::default())
                .prefix("/resources")
                .resource("/", |r| r.get().with_async(pot))
                .scope("/index", |s| {
                    s.resource("", |r| r.get().with_async(get_by_page))
                    .resource("/query", |r| r.get().with_async(get_by_pred))
                    .nested("/archive", |is| {
                        is.resource("", |r| r.get().with_async(get_archives))
                            .resource("/{year}/{month}", |r| r.get().with_async(query_archives))
                    })
                })
                .scope("/post", |s| 
                    s.resource("/tag", |r| r.get().with_async(get_all_tags))
                    .resource("/comment", |r| {
                        if std::env::var("DISABLE_RECAPTCHA").is_err() {
                            r.middleware(CommentFilter(conn, std::env::var("RECAPTCHA_SECRET").expect("No recaptcha secret got! You can set env DISABLE_RECAPTCHA or RECAPTCHA_SECRET.")));
                        }
                        r.method(http::Method::POST)
                         .with_async(comment_to)})
                         .resource("/{id}", |r| r.get().with_async(get_post_by_id))
                );
                if std::env::var("ENABLE_CORS").is_ok() {
                    app.middleware(Cors::default())
                } else { app }.finish()
        }).bind("127.0.0.1:8000")
        .expect("Failed to bind.")
        .start();
    }
}

fn pot<T>(_req: HttpRequest<T>) -> impl Future<Item=HttpResponse, Error=Error>{
    ok(HttpResponse::build(http::StatusCode::IM_A_TEAPOT).body("may be short and stout"))
}

fn main() {
    ::std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let sys = actix::System::new("verse-of-south");
    let _ = RootActor {}.start();
    sys.run();
}
