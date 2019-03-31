pub mod actors;
pub mod messages {
    use actix::prelude::*;
    use crate::web::handlers::post::*;
    use crate::web::models::index_post::*;
    use crate::web::models::detailed_post::DetailedPost;
    
    fn get_default_page_size() -> u32 { 4 }

    #[derive(Debug)]
    pub struct PageInfo {
        pub offset: u32,
        pub limit: u32,
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

    impl Message for GiveMePostOfPage {
        type Result = Result<Vec<Post>, ()>;
    }

    impl Message for GiveMePostOfPageMatches {
        type Result = Result<Vec<Post>, ()>;
    }

    impl Message for PostIdQuery {
        type Result = Result<DetailedPost, ()>;
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