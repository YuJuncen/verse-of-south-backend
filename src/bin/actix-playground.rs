use actix::prelude::*;
use tokio::prelude::*;
use std::io;

struct Post { }
impl Actor for Post {
    type Context = Context<Self>;
}

struct Index { 
    posts: Vec<Addr<Post>>
}
impl Actor for Index {
    type Context = Context<Self>;
    fn started(&mut self, _ctx: &mut Self::Context){
        for _ in 0..3 {
            self.posts.push(Post{}.start());
        }
    }
}

#[derive(Debug)]
struct Read {}
impl Message for Read {
    type Result = std::result::Result<(), std::io::Error>;
}
impl Handler<Read> for Post {
    type Result = Box<Future<Item=(), Error=io::Error>>;
    fn handle(&mut self, msg: Read, _ctx: &mut Self::Context) -> Self::Result {
        println!("I am a post, I received {:?}.", msg);
        Box::new(futures::future::ok(()))
    }
}

#[derive(Debug)]
struct FetchPosts {}
impl Message for FetchPosts {
    type Result = Result<(), io::Error>;
}
impl Handler<FetchPosts> for Index {
    type Result = Result<(), io::Error>;
    fn handle(&mut self, msg: FetchPosts, _ctx: &mut Self::Context) -> Self::Result {
        println!("Get msg {:?}.", msg);
        for post in self.posts.iter() {
            Arbiter::spawn(post.send(Read {}).map(|_| ()).map_err(|_| ()));
        }
        Ok(())
    }
}

#[derive(Debug)]
struct GoodLuck {}
impl Message for GoodLuck {
    type Result = Result<Addr<Post>, ()>;
}
impl Handler<GoodLuck> for Index{
    type Result = Result<Addr<Post>, ()>;
    fn handle(&mut self, _msg: GoodLuck, _ctx: &mut Self::Context) -> Self::Result{
        Ok(self.posts.get(1).cloned().unwrap())
    }
}
fn main() {
    System::run(|| {
        let addr = Index{posts: vec![]}.start();
        Arbiter::spawn(addr.send(GoodLuck{}).map(|p| {
            match p {
                Ok(addr) => Arbiter::spawn(addr.send(Read{}).map(|_|()).map_err(|_|())),
                _ => unreachable!(),
            };
            println!("Get result.");
        }).map_err(|_| ()))
    });
}