use diesel::sql_query;
use actix::prelude::*;
use vos::database::establish_connection;
use vos::wrapper::messages::*;
use vos::wrapper::actors::database::{ Database, LoadByDsl };
use futures::future::*;
use diesel::prelude::*;
use diesel::sql_types::*;

fn main() {
    ::std::env::set_var("RUST_LOG", "debug");
    let mps = sql_query("
        SELECT * FROM posts P1 WHERE 
            NOT EXISTS
            ((SELECT tags.id FROM tags WHERE tag_name = ANY (string_to_array($1, ':')::text[]))
            EXCEPT 
            (SELECT tag_id FROM posts P2 INNER JOIN tag_to ON post_id = P2.id WHERE P1.id = P2.id))
        AND
            P1.title LIKE ALL ( string_to_array($2, ' ')::text[] )
        LIMIT $3 OFFSET $4")
        .bind::<Text, String>("".to_string())
        .bind::<Text, _>(Some("');--").map(|t| t.trim().split(' ').map(|c| format!("%{}%", c)).collect::<Vec<_>>().join(" ")).unwrap_or_default())
        .bind::<BigInt, _>(999 as i64)
        .bind::<BigInt, _>(0 as i64);
    let sys = System::new("ayesha=pg");
    let db = SyncArbiter::start(8, || Database::new(establish_connection()));
    use vos::database::models::post::Post;
    Arbiter::spawn(db.send(LoadByDsl::new(mps)).and_then(
        |p: Result<std::vec::Vec<Post>, diesel::result::Error>| {
        println!("{:?}", p);
        ok(())
    }).map_err(|_| ()));

    sys.run();
}
