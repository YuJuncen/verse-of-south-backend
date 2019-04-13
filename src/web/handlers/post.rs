use actix_web::*;
use futures::future::*;
use crate::web::AppState;
use crate::wrapper::messages::*;

#[derive(Deserialize, Debug)]
pub struct PostIdQuery {
    pub id: u32,
}

pub fn get_post_by_id((req, p): (HttpRequest<AppState>, Path<PostIdQuery>)) -> impl Future<Item=HttpResponse, Error=Error> {
    use crate::wrapper::actors::pgdatabase::DatabaseError; 
    use diesel::result::Error::NotFound;   
    req.state().database.send::<GiveMeFullPostOfId>(p.into_inner().into())
        .from_err()
        .map(|p| match p {
                Ok(data) => HttpResponse::Ok().json(data),
                Err(e) => match e {
                    DatabaseError::DieselGoesWrong(NotFound) => HttpResponse::NotFound().json(e), 
                    _ => HttpResponse::InternalServerError().json(e) 
                }
        })
}