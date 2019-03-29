use actix_web::*;
use futures::future::*;
use crate::web::models::index_post::*;
use crate::web::models::detailed_post::*;
use crate::web::AppState;

#[derive(Deserialize, Debug)]
pub struct PageQuery {
    pub offset: Option<u32>,
    pub limit: Option<u32>,
}

#[derive(Deserialize, Debug)]
pub struct PredicateQuery {
    pub title: Option<String>,
    pub tags: Option<String>,
}

impl Into<PredicateQueryMessage> for PredicateQuery {
    fn into(self) -> PredicateQueryMessage {
        PredicateQueryMessage {
        title: self.title,
        tags: self.tags.map(|s| s.split("+").map(|s| Tag { name: String::from(s) }).collect()).unwrap_or(vec![]),
    }
    }
}

pub struct PredicateQueryMessage {
    pub title: Option<String>,
    pub tags: Vec<Tag>
}

#[derive(Deserialize, Debug)]
pub struct PostIdQuery {
    pub id: u32,
}

pub fn get_by_page((req, p): (HttpRequest<AppState>, Query<PageQuery>)) -> impl Future<Item=HttpResponse, Error=Error> {
    req.state().index.send(p.into_inner())
        .from_err()
        .map(|ps| HttpResponse::Ok().json(ps))
}

pub fn get_by_pred((req, p): (HttpRequest<AppState>, Query<PredicateQuery>)) -> impl Future<Item=HttpResponse, Error=Error> {
    req.state().search.send::<PredicateQueryMessage>(p.into_inner().into())
        .from_err()
        .map(|ps| HttpResponse::Ok().json(ps))
}

pub fn get_post_by_id((req, p): (HttpRequest<AppState>, Query<PostIdQuery>)) -> impl Future<Item=DetailedPost, Error=Error> {
    req.state().post.send(p.into_inner())
        .from_err()
        .map(|p| p.unwrap())
}