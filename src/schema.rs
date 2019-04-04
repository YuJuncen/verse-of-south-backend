table! {
    comments (id) {
        id -> Int4,
        publish_time -> Timestamp,
        content -> Text,
        publisher_name -> Varchar,
        publisher_email -> Nullable<Varchar>,
        post_id -> Int4,
        reply_to -> Nullable<Int4>,
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
    tag_to (tag_id, post_id) {
        tag_id -> Int4,
        post_id -> Int4,
    }
}

table! {
    tags (id) {
        tag_name -> Varchar,
        id -> Int4,
    }
}

joinable!(comments -> posts (post_id));
joinable!(tag_to -> posts (post_id));
joinable!(tag_to -> tags (tag_id));

allow_tables_to_appear_in_same_query!(
    comments,
    posts,
    tag_to,
    tags,
);
