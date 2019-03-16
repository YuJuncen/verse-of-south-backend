use crate::web::models::post::WebPost;
trait PostLike : Into<WebPost> {
    type Tagtype;
    type CommentType;
    type DiffType;
    fn attach_tags(&mut self, tags: &Vec<Self::Tagtype>);
    fn comment(&mut self, comment: Self::CommentType);
    // this function disabled temporarily.
    // fn become(&mut self, diff: Self::Difftype) -> Self;
}