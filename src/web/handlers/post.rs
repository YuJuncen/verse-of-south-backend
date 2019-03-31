use actix_web::*;
use futures::future::*;
use crate::web::models::index_post::*;
use crate::web::models::detailed_post::*;
use crate::web::AppState;
use crate::wrapper::messages::*;

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct PageQuery {
    pub offset: Option<u32>,
    pub limit: Option<u32>,
}

#[derive(Deserialize, Debug)]
pub struct PostIdQuery {
    pub id: u32,
}

#[derive(Deserialize, Debug)]
pub struct PredicateQuery {
    pub page: Option<PageQuery>,
    pub title: Option<String>,
    pub tags: Option<String>,
}





pub fn get_by_page((req, p): (HttpRequest<AppState>, Query<PageQuery>)) -> impl Future<Item=HttpResponse, Error=Error> {
    req.state().index.send(p.into_inner().into())
        .from_err()
        .map(|ps| HttpResponse::Ok().json(ps))
}

pub fn get_by_pred((req, p): (HttpRequest<AppState>, Query<PredicateQuery>)) -> impl Future<Item=HttpResponse, Error=Error> {
    req.state().search.send::<GiveMePostOfPageMatches>(p.into_inner().into())
        .from_err()
        .map(|ps| HttpResponse::Ok().json(ps))
}

pub fn get_post_by_id((req, p): (HttpRequest<AppState>, Query<PostIdQuery>)) -> impl Future<Item=DetailedPost, Error=Error> {
    req.state().post.send(p.into_inner())
        .from_err()
        .map(|p| p.unwrap())
}