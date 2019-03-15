-- Your SQL goes here

CREATE VIEW full_post(id, title, intro, body, publish_time) AS
  SELECT P.id, P.title, P.intro, P.body, P.publish_time FROM posts AS P;

CREATE VIEW comments_with_post(post_id, comment_id, comment_content, publisher_name, publish_time) AS
  SELECT P.id, C.id, C.content, C.publisher_name, C.publish_time FROM posts as P INNER JOIN comments as C ON C.is_for = P.id;