    use actix::prelude::*;
    use crate::web::models::index_post::Tag;
    use crate::web::models::index_post::*;
    use crate::web::models::detailed_post::DetailedPost;
    use crate::wrapper::actors::pgdatabase::DatabaseError;
    use crate::web::models::comment::Comment;
    use crate::database::models::types::ArchiveInfo;

    #[derive(Debug)]
    pub struct GiveMeArchiveOf{
        pub year: i32,
        pub month: i32,
        pub page: PageInfo,
    }
    pub struct GiveMeArchiveInfo;

    #[derive(Debug)]
    pub struct GiveMeFullPostOfId(pub i32);

    #[derive(Debug)]
    pub struct PageInfo {
        pub offset: i64,
        pub limit: i64,
    }

    #[derive(Debug)]
    pub struct GiveMePostOfPageMatches {
        pub page: PageInfo,
        pub title: Option<String>,
        pub tags: Vec<Tag>
    }

    #[derive(Debug)]
    pub struct GiveMePostOfPage {
        pub page: PageInfo,
    }

    #[derive(Debug)]
    pub struct CommentToPost {
        pub publisher: String,
        pub publisher_email: Option<String>,
        pub content: String,
        pub to: i32,
        pub reply_to: Option<i32>
    }

    #[derive(Debug)]
    pub struct GiveMeAllTags;

    impl Message for GiveMeArchiveInfo {
        type Result = Result<Vec<ArchiveInfo>, DatabaseError>;
    }

    impl Message for GiveMeArchiveOf {
        type Result = Result<Vec<Post>, DatabaseError>;
    }

    impl Message for GiveMeFullPostOfId {
        type Result = Result<DetailedPost, DatabaseError>;
    }

    impl Message for CommentToPost {
        type Result = Result<Comment, DatabaseError>;
    }

    impl Message for GiveMePostOfPage {
        type Result = Result<Vec<Post>, DatabaseError>;
    }

    impl Message for GiveMePostOfPageMatches {
        type Result = Result<Vec<Post>, DatabaseError>;
    }

    impl Message for GiveMeAllTags {
        type Result = Result<Vec<Tag>, DatabaseError>;
    }