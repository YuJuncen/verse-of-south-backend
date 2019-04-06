pub mod actors;
pub mod messages {
    use actix::prelude::*;
    use crate::web::handlers::post::*;
    use crate::web::handlers::comment::NewComment;
    use crate::web::models::index_post::*;
    use crate::web::models::detailed_post::DetailedPost;
    use crate::wrapper::actors::pgdatabase::DatabaseError;
    use crate::web::models::comment::Comment;
    use crate::database::models::types::ArchiveInfo;

    fn get_default_page_size() -> i64 { 4 }

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

    impl Into<GiveMeFullPostOfId> for PostIdQuery {
        fn into(self) -> GiveMeFullPostOfId {
            GiveMeFullPostOfId(self.id as i32)
        }
    }

    impl Into<CommentToPost> for NewComment {
        fn into(self) -> CommentToPost {
            CommentToPost {
                content: self.comment,
                publisher: self.publisher_name,
                publisher_email: self.publisher_email,
                to: self.to,
                reply_to: self.reply_to,
            }
        }
    }

    impl Into<PageInfo> for PageQuery {
        fn into(self) -> PageInfo {
            PageInfo { offset: self.offset.unwrap_or(0), limit: self.limit.unwrap_or(get_default_page_size())}
        }
    }

    impl Into<PageInfo> for Option<PageQuery> {
        fn into(self) -> PageInfo {
            self.map(Into::into).unwrap_or( PageInfo {offset: 0, limit: get_default_page_size()} )
        }
    }

    impl Into<GiveMePostOfPageMatches> for PredicateQuery {
        fn into(self) -> GiveMePostOfPageMatches {
            GiveMePostOfPageMatches {
                    page: self.page.into(),
                    title: self.title,
                    tags: self.tags.map(|s| s.split(":").map(|s| Tag { name: String::from(s) }).collect()).unwrap_or(vec![]),
            }
        }
    }

    impl Into<GiveMePostOfPage> for PageQuery {
        fn into(self) -> GiveMePostOfPage {
            GiveMePostOfPage {
                page: self.into()
            }
        }
    }
}