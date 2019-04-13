use diesel::sql_query;
use actix::prelude::*;
use vos::database::establish_connection;
use vos::wrapper::messages::*;
use vos::wrapper::actors::database::{ Database, FirstByDsl };
use futures::future::*;
use diesel::prelude::*;
use diesel::sql_types::*;

fn main() {
    ::std::env::set_var("RUST_LOG", "debug");
    use vos::database::models::comment::Comment;
    use vos::schema::comments;
    let sys = System::new("ayesha=pg");

    let mps = comments::table;
    let sys = System::new("ayesha=pg");
    let db = SyncArbiter::start(8, || Database::new(establish_connection()));
    Arbiter::spawn(db.send(FirstByDsl::new(mps)).and_then(
        |p: Result<Comment, diesel::result::Error>| {
        println!("{:?}", p);
        ok(())
    }).map_err(|_| ()));

    sys.run();
}
