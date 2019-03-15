table! {
    comments (id) {
        id -> Int4,
        publish_time -> Timestamp,
        content -> Text,
        publisher_name -> Varchar,
        publisher -> Nullable<Int4>,
        is_for -> Nullable<Int4>,
    }
}

table! {
    posts (id) {
        id -> Int4,
        publish_time -> Timestamp,
        title -> Varchar,
        intro -> Nullable<Text>,
        body -> Text,
        body_format -> Int2,
    }
}

table! {
    readers (ip) {
        ip -> Int4,
    }
}

table! {
    tags (id) {
        tag_name -> Varchar,
        id -> Int4,
    }
}

table! {
    tag_to (id) {
        id -> Int4,
        the_tag -> Int4,
        the_post -> Int4,
    }
}

joinable!(comments -> posts (is_for));
joinable!(comments -> readers (publisher));
joinable!(tag_to -> posts (the_post));
joinable!(tag_to -> tags (the_tag));

allow_tables_to_appear_in_same_query!(
    comments,
    posts,
    readers,
    tags,
    tag_to,
);
