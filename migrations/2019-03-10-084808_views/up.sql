-- Your SQL goes here

CREATE VIEW full_post(id, title, intro, body, publish_time) AS
  SELECT P.ID, P.title, P.intro, C.body, P.publish_time FROM posts as P INNER JOIN post_contents AS C ON C.is_for = P.ID;

CREATE VIEW comments_with_post(post_id, comment_id, comment_content, publisher_name, publish_time) AS
  SELECT P.ID, C.ID, C.content, C.publisher_name, C.publish_time FROM posts as P INNER JOIN comments as C ON C.is_for = P.ID;