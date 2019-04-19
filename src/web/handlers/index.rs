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
    // NOTE：这里可以优化。
    //       但是现在看来没有什么大用；因此将其挂起了。
    // 看哪，这就是单子的回调地狱！
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
    debug!("GET SEARCH QUERY：{:?}", pred);
    // NOTE：用一种合理的抽象（诸如 Do Notation 一类）来解决这些代码的复杂性其实是一相当有趣的事情。
    //       不过这段旅途很可能充满危险。
    //       失败的例子就是下面的 to_json 函数（追忆 Haskell 的 RankNTypes 和全局类型推导）。
    //       或许将它作为一个宏会好一些。
    req.state().database.send(GiveMePaginationOf {base_query: pred.clone()}).from_err().and_then(result)
        .and_then(move |pagination| // pagnation <- database.send(GiveMePaginationOf {base_query: pred.clone()})
            req.state().database.send::<GiveMePostOfPageMatches>(pred.clone()).from_err().and_then(result)
            .and_then(move |ps| // ps <- database.send::<GiveMePostOfPageMatches>(pred.clone())
                req.state().database.send(GiveMeAllTags{}).from_err().and_then(result)
                .map(move |ts| QueryByPredicateResult { // ts <- database.send(GiveMeAllTags{})
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
