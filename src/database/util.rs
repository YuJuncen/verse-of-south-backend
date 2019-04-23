use diesel::insert_into;
use diesel::prelude::*;
use self::types::*;
use super::models::{ tag::{ Tag } , post::Post };

pub mod types {
    use crate::schema::tags;
    use crate::schema::posts;
    use crate::schema::tag_to;

    #[derive(Insertable, Debug)]
    #[table_name = "tags"]
    pub struct NewTag<'a> {
        pub tag_name: &'a str
    }

    #[derive(Insertable, Debug)]
    #[table_name = "posts"]
    pub struct NewPost<'a> {
        pub title: &'a str,
        pub intro: Option<&'a str>,
        pub body: &'a str,
    }

    #[derive(Insertable, Debug)]
    #[table_name = "tag_to"]
    pub struct NewPostTagBind {
        pub post_id: i32,
        pub tag_id: i32,
    }
}

macro_rules! load_table {
    ($table: ident to type $load_to: ident with connection $conn: ident) => {
        $table.load::<$load_to>($conn).expect("Failed to load table.")
    };
}

pub fn load_tags(conn: &PgConnection) -> Vec<Tag> {
    use crate::schema::tags::dsl::tags;
    load_table! [tags to type Tag with connection conn]
}

pub fn add_tag(conn: &PgConnection, tag_name: &str) -> Tag {
    use crate::schema::tags::dsl::tags;
    insert_into(tags)
        .values(NewTag { tag_name } )
        .get_result::<Tag>(conn)
        .expect("Failed to insert.")
}


pub fn publish_post(conn: &PgConnection, post: NewPost, with_tags: &[i32]) -> Post {
    conn.build_transaction()
        .run::<_, diesel::result::Error, _>(|| {
            use crate::schema::posts::dsl::posts;
            use crate::schema::tag_to::dsl::tag_to;
            let p = insert_into(posts)
                    .values(post)
                    .get_result::<Post>(conn)?;
            let _ = insert_into(tag_to)
                .values::<Vec<NewPostTagBind>>(with_tags.iter().map(|&t| NewPostTagBind { tag_id: t, post_id: p.id }).collect())
                .execute(conn)?;
            Ok(p)
        }
    ).expect("Failed to inerst...")
}
