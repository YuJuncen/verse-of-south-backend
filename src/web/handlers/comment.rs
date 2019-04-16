use actix_web::*;
use futures::future::*;
use crate::web::AppState;
use crate::wrapper::messages::*;
use super::types::*;


pub fn comment_to((req, comment): (HttpRequest<AppState>, Json<NewComment>)) -> impl Future<Item=HttpResponse, Error=Error> {
    req.state().database.send::<CommentToPost>(comment.into_inner().into())
        .map(|c| c.unwrap())
        .map(|c| HttpResponse::Created()
            .json(c))
        .from_err()
}