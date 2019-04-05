use actix_web::*;
use futures::future::*;
use crate::web::AppState;
use crate::wrapper::messages::*;

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct PageQuery {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
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
    req.state().index.send::<GiveMePostOfPage>(p.into_inner().into())
        .from_err()
        .map(|ps| HttpResponse::Ok().json(ps))
}

pub fn get_by_pred((req, p): (HttpRequest<AppState>, Query<PredicateQuery>)) -> impl Future<Item=HttpResponse, Error=Error> {
    req.state().index.send::<GiveMePostOfPageMatches>(p.into_inner().into())
        .from_err()
        .map(|ps| HttpResponse::Ok().json(ps))
}

pub fn get_post_by_id((req, p): (HttpRequest<AppState>, Query<PostIdQuery>)) -> impl Future<Item=HttpResponse, Error=Error> {
    use crate::wrapper::actors::pgdatabase::DatabaseError; 
    use diesel::result::Error::NotFound;   
    req.state().post.send::<GiveMeFullPostOfId>(p.into_inner().into())
        .from_err()
        .map(|p| match p {
                Ok(data) => HttpResponse::Ok().json(data),
                Err(e) => match e {
                    DatabaseError::DieselGoesWrong(NotFound) => HttpResponse::NotFound().json(e), 
                    _ => HttpResponse::InternalServerError().json(e) 
                }
        })
}