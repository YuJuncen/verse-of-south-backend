use crate::wrapper::actors::pgdatabase::DatabaseError;
use actix_web::*;
use futures::future::*;
use crate::web::AppState;
use crate::wrapper::messages::*;
use std::collections::HashSet;
use super::to_json;
use super::types::*;

pub fn get_by_page((req, p): (HttpRequest<AppState>, Query<PageQuery>)) -> impl Future<Item=HttpResponse, Error=Error> {
    let page : PageInfo = p.into_inner().into();
    req.state().database.send(GiveMePaginationOf {base_query: GiveMePostOfPageMatches{page: page.clone(), tags: vec![], title: None}})
        .from_err()
        .and_then(result)
        .and_then(move |pagination| 
        req.state().database.send(GiveMePostOfPage{page: page.clone()})
            .from_err()
            .and_then(result)
            .map(|ps| HttpResponse::Ok().json(
                QueryWithPagination {
                    result: ps,
                    pagination
                }
            )))
        .from_err()
}

pub fn get_by_pred((req, p): (HttpRequest<AppState>, Query<PredicateQuery>)) -> impl Future<Item=HttpResponse, Error=Error> {
    let pred : GiveMePostOfPageMatches = p.into_inner().into();
    let tags : HashSet<_> = pred.tags.iter().cloned().collect();
    debug!("{:?}", pred);
    req.state().database.send(GiveMePaginationOf {base_query: pred.clone()}).from_err().and_then(result).and_then(move |pagination|
    req.state().database.send::<GiveMePostOfPageMatches>(pred.clone())
        .from_err()
        .and_then(result)
        .and_then(move |ps| req.state().database.send(GiveMeAllTags{}).from_err().and_then(result).map(move |ts| QueryByPredicateResult {
            result: ps,
            tags_not_use: tags.difference(&(ts.into_iter().collect())).cloned().collect()
        }))
        .map(|ps| HttpResponse::Ok().json(QueryWithPagination {
                result: ps,
                pagination
        })))
        .from_err()
}

pub fn get_archives(req: HttpRequest<AppState>) -> impl Future<Item=HttpResponse, Error=Error> {
    to_json::<_, DatabaseError, _, _, _>(req.state().database.send(GiveMeArchiveInfo{}))
}

pub fn query_archives((req, q): (HttpRequest<AppState>, Path<ArchiveQuery>)) -> impl Future<Item=HttpResponse, Error=Error> {
    let q = q.into_inner();
    to_json::<_, DatabaseError, _, _, _>(req.state().database.send(GiveMeArchiveOf {year: q.year, month: q.month, page: PageInfo{offset: 0, limit: None}}))
}
