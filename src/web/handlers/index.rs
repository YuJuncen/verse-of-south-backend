use actix_web::*;
use futures::future::*;
use crate::web::AppState;
use crate::wrapper::messages::*;
use crate::web::models::index_post::{ Tag, Post };

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
    #[serde(flatten)]
    pub page: Option<PageQuery>,
    pub title: Option<String>,
    pub tags: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct QueryResult {
    pub tags_no_usage: Vec<Tag>,
    pub result: Vec<Post>,
}

pub fn get_by_page((req, p): (HttpRequest<AppState>, Query<PageQuery>)) -> impl Future<Item=HttpResponse, Error=Error> {
    let r : GiveMePostOfPage = p.into_inner().into();
    debug!("[GET_BY_PAGE]RECEIVED: {:?}", r);
    req.state().index.send::<GiveMePostOfPage>(r)
        .from_err()
        .map(|ps| HttpResponse::Ok().json(ps))
}

pub fn get_by_pred((req, p): (HttpRequest<AppState>, Query<PredicateQuery>)) -> impl Future<Item=HttpResponse, Error=Error> {
    let r = p.into_inner();
    debug!("[GET_BY_PRED]RECEIVED: {:?}", r);
    req.state().index.send::<GiveMePostOfPageMatches>(r.into())
        .from_err()
        .map(|ps| HttpResponse::Ok().json(ps))
}

pub fn get_archives(req: HttpRequest<AppState>) -> impl Future<Item=HttpResponse, Error=Error> {
    req.state().index.send(GiveMeArchiveInfo{})
        .from_err()
        .map(|ps| HttpResponse::Ok().json(ps))
}

pub fn query_archives((req, q): (HttpRequest<AppState>, Path<ArchiveQuery>)) -> impl Future<Item=HttpResponse, Error=Error> {
    let q = q.into_inner();
    req.state().index.send(GiveMeArchiveOf {year: q.year, month: q.month, page: PageInfo{offset: 0, limit: 999}})
        .from_err()
        .map(|ps| HttpResponse::Ok().json(ps))
}
