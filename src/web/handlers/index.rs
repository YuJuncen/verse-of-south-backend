use actix_web::*;
use futures::future::*;
use crate::web::AppState;
use crate::wrapper::messages::*;
use crate::web::models::index_post::{ Post, Tag };
use std::collections::HashSet;


#[derive(Deserialize, Debug)]
pub struct ArchiveQuery {
    pub year: i32,
    pub month: i32,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct PageQuery {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}
#[derive(Deserialize, Debug)]
pub struct PredicateQuery {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub title: Option<String>,
    pub tags: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct QueryByPredicateResult {
    pub result: Vec<Post>,
    pub tags_not_use: HashSet<Tag>,
}

pub fn get_by_page((req, p): (HttpRequest<AppState>, Query<PageQuery>)) -> impl Future<Item=HttpResponse, Error=Error> {
    req.state().database.send::<GiveMePostOfPage>(p.into_inner().into())
        .from_err()
        .map(|ps| HttpResponse::Ok().json(ps))
}

pub fn get_by_pred((req, p): (HttpRequest<AppState>, Query<PredicateQuery>)) -> impl Future<Item=HttpResponse, Error=Error> {
    let pred : GiveMePostOfPageMatches = p.into_inner().into();
    let tags : HashSet<_> = pred.tags.iter().cloned().collect();
    debug!("{:?}", pred);
    req.state().database.send::<GiveMePostOfPageMatches>(pred)
        .from_err()
        .and_then(result)
        .and_then(move |ps| req.state().database.send(GiveMeAllTags{}).from_err().and_then(result).map(move |ts| QueryByPredicateResult {
            result: ps,
            tags_not_use: tags.difference(&(ts.into_iter().collect())).cloned().collect()
        }))
        .map(|ps| HttpResponse::Ok().json(ps))
        .from_err()
}

pub fn get_archives(req: HttpRequest<AppState>) -> impl Future<Item=HttpResponse, Error=Error> {
    req.state().database.send(GiveMeArchiveInfo{})
        .from_err()
        .map(|ps| HttpResponse::Ok().json(ps))
}

pub fn query_archives((req, q): (HttpRequest<AppState>, Path<ArchiveQuery>)) -> impl Future<Item=HttpResponse, Error=Error> {
    let q = q.into_inner();
    req.state().database.send(GiveMeArchiveOf {year: q.year, month: q.month, page: PageInfo{offset: 0, limit: 999}})
        .from_err()
        .map(|ps| HttpResponse::Ok().json(ps))
}
