-- Your SQL goes here

CREATE TABLE comments (
  id SERIAL PRIMARY KEY ,
  publish_time TIMESTAMP
    NOT NULL
    DEFAULT now(),
  content TEXT NOT NULL ,
  publisher_name VARCHAR NOT NULL ,
  publisher_email VARCHAR,
  post_id INTEGER
    REFERENCES posts(id)
    NOT NULL,
  reply_to INTEGER
    REFERENCES comments(id),
  CHECK (LOWER(publisher_email) SIMILAR TO '([a-z0-9_\.-]+)@([\da-z\.-])+\.([a-z\.]{2,6})')
)