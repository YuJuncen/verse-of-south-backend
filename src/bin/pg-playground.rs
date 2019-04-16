use actix::prelude::*;
use vos::database::establish_connection;
use vos::wrapper::messages::*;
use vos::wrapper::actors::pgdatabase::PGDatabase;
use futures::future::*;

fn main() {
    ::std::env::set_var("RUST_LOG", "debug");
    let sys = System::new("welbell=pg");
    let db = SyncArbiter::start(8, || PGDatabase::new(establish_connection()));
    Arbiter::spawn(db.send(GiveMeArchiveInfo {}).and_then(|p| {
        println!("{:?}", p);
        ok(())
    }).map_err(|_| ()));
    Arbiter::spawn(db.send(GiveMeArchiveOf {page: PageInfo {offset: 0, limit: None}, month: 7, year: 2019}).map(|e| println!("{:?}", e))
        .map_err(|_| ()));
    sys.run();
}
