pub mod actors;
pub mod messages {
    use actix::prelude::*;
    use crate::web::handlers::post::*;
    use crate::web::models::index_post::Post;
    use crate::web::models::detailed_post::DetailedPost;


    impl Message for PageQuery {
        type Result = Result<Vec<Post>, ()>;
    }

    impl Message for PredicateQueryMessage {
        type Result = Result<Vec<Post>, ()>;
    }

    impl Message for PostIdQuery {
        type Result = Result<DetailedPost, ()>;
    }
}