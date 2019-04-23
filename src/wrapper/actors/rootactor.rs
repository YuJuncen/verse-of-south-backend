use std::net::ToSocketAddrs;
use ::actix::prelude::*;
use actix_web::*;
use actix_web::middleware::cors::Cors;
use crate::web::handlers::post::*;
use crate::web::handlers::comment::*;
use crate::web::handlers::index::*;
use crate::wrapper::actors::pgdatabase::PGDatabase;
use crate::database::establish_connection;
use crate::web::AppState;
use crate::web::middlewares::*;
use openssl::ssl::{ SslConnector, SslMethod };
use futures::future::*;
use std::net::{Ipv4Addr, SocketAddrV4};

pub struct RootActor<T> {
    listen_to: T
}

impl RootActor<SocketAddrV4> {
    pub fn listen_to_localhost(port: u16) -> Self {
        RootActor {listen_to : SocketAddrV4::new(Ipv4Addr::LOCALHOST, port)}
    }

    pub fn listen_to_all_network(port: u16) -> Self {
        RootActor {listen_to : SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port) }
    }
}


impl<T: ToSocketAddrs + Clone + 'static> Actor for RootActor<T> {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
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
                         .with_async_config(comment_to, |conf| {
                             (conf.0).1.limit(4096);
                         })
                        })
                    .resource("/{id}", |r| r.get().with_async(get_post_by_id))
                );
                if std::env::var("ENABLE_CORS").is_ok() {
                    app.middleware(Cors::default())
                } else { app }.finish()
        }).bind(self.listen_to.clone())
        .expect("Failed to bind.")
        .start();
    }


}
fn pot<T>(_req: HttpRequest<T>) -> impl Future<Item=HttpResponse, Error=Error>{
    ok(HttpResponse::build(http::StatusCode::IM_A_TEAPOT).body("may be short and stout"))
}