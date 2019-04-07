use super::messages::*;
use crate::web::handlers::post::*;
use crate::web::handlers::index::*;
use crate::web::handlers::comment::NewComment;
use crate::web::models::index_post::Tag;

fn get_default_page_size() -> i64 { 4 }

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
