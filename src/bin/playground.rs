#[allow(unused_imports)]
use vos::database::models::post::{ NewPost, Post };
use vos::database::models::tag::TagTo;
use vos::database::models::types::FormatType;
use vos::database;
use chrono::prelude::*;
use diesel::prelude::*;

fn main() {
    use vos::schema::posts::dsl::*;
    let conn = database::establish_connection();
/*     let post = Post {
        id: 0,
        title: String::from("Hello, world!"),
        intro: Option::None,
        publish_time: Utc::now().naive_utc(),
        body: String::from("# 你好  \n这里是南方之诗！"),
        body_format: FormatType::Markdown
    };
    let np = NewPost::new("Hi!", "新的冒险已经开始了！", None);
    let _res = diesel::insert_into(posts)
        .values(&np)
        .get_result::<Post>(&conn);
    let p2 = posts.filter(id.eq(1))
        .load::<Post>(&conn).unwrap();
    println!("{:?}  //\n{:?}  //\n{:?}", post, np, p2); */
    let ps = posts.load::<Post>(&conn).unwrap();
    let post_tag_ids = TagTo::belonging_to(&ps)
            .load::<TagTo>(&conn).unwrap()
            .grouped_by(&ps);
    println!("{:?}\n{:?}", ps, post_tag_ids);
}