use vos::database::models::post::{ NewPost, Post, FormatType };
use vos::database;
use chrono::prelude::*;
use diesel::prelude::*;

fn main() {
    use vos::schema::posts::dsl::*;
    let post = Post {
        id: 0,
        title: String::from("Hello, world!"),
        intro: Option::None,
        publish_time: Utc::now().naive_utc(),
        body: String::from("# 你好  \n这里是南方之诗！"),
        body_format: FormatType::Markdown
    };
    let np = NewPost::new("Hi!", "新的冒险已经开始了！", None);
    let conn = database::establish_connection();
    // let res = diesel::insert_into(posts)
    //    .values(&np)
    //    .get_result::<Post>(&conn);
    let p2 = posts.filter(id.eq(2))
        .load::<Post>(&conn).unwrap();
    let tag_of_id_2 = p2.get(0).unwrap().get_tags(&conn);
    println!("{:?}  //\n{:?}  //\n{:?}", post, np, tag_of_id_2);
}