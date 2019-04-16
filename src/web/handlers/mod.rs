pub mod comment;
pub mod index;
pub mod post;

use actix_web::*;
use futures::future::*;
use serde::ser::Serialize;

pub mod types {
        use crate::web::models::index_post::{Post, Tag};
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
        #[serde(rename_all = "camelCase")]
        pub struct QueryByPredicateResult {
                pub result: Vec<Post>,
                pub tags_not_use: HashSet<Tag>,
        }

        #[derive(Debug, Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct QueryWithPagination<T> {
                pub pagination: Pagination,
                pub result: T,
        }

        #[derive(Serialize, Deserialize, Debug)]
        pub struct NewComment {
                pub comment: String,
                pub publisher_name: String,
                pub publisher_email: Option<String>,
                pub to: i32,
                pub reply_to: Option<i32>,
        }

        #[derive(Debug, Serialize)]
        pub struct Pagination {
                offset: i64,
                total: usize,
                limit: Option<i64>,
        }

        impl Pagination {
                pub fn new(offset: i64, total: usize, limit: Option<i64>) -> Pagination {
                        Pagination {
                                offset,
                                total,
                                limit
                        }
                }
        }
}

pub fn to_request<
        U,
        E: From<E1> + From<E2> + ResponseError,
        E1,
        E2,
        Fut: Future<Item = Result<U, E1>, Error = E2>,
        F: FnOnce(U) -> HttpResponse,
>(
        f: Fut,
        mapper: F,
) -> impl Future<Item = HttpResponse, Error = Error> {
        f.map(|r| r.map_err(E::from))
                .from_err()
                .and_then(result)
                .map(mapper)
                .from_err()
}

pub fn to_json<
        U: Serialize,
        E: From<E1> + From<E2> + ResponseError,
        E1,
        E2,
        F: Future<Item = Result<U, E1>, Error = E2>,
>(
        f: F,
) -> impl Future<Item = HttpResponse, Error = Error> {
        to_request::<_, E, _, _, F, _>(f, |i| HttpResponse::Ok().json(i))
}
