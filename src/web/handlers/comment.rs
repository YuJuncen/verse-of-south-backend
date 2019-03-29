use actix_web::*;
use futures::future::{ok, Future};
use crate::web::AppState;

#[derive(Serialize, Deserialize, Debug)]
pub struct CommentTo {
    comment: String,
    publisher_name: String,
    comment_to: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SampleReturnValue {
    comment: String,
    publisher_name: String,
    publisher_addr: String,
}


pub fn comment_to((req, comment): (HttpRequest<AppState>, Json<CommentTo>)) -> impl Future<Item=HttpResponse, Error=Error> {
    ok(
        HttpResponse::Created()
            .json(SampleReturnValue {
                comment: comment.comment.clone(),
                publisher_name: comment.publisher_name.clone(),
                publisher_addr: req.peer_addr().map(|addr| format!("{:?}", addr.ip())).unwrap_or("UnKnown".to_string())
            })
    )
}