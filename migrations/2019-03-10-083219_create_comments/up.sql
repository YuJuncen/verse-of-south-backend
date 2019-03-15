-- Your SQL goes here

CREATE TABLE comments (
  id SERIAL PRIMARY KEY ,
  publish_time TIMESTAMP
    NOT NULL
    DEFAULT now(),
  content TEXT NOT NULL ,
  publisher_name VARCHAR NOT NULL ,
  publisher INTEGER
    REFERENCES readers(IP),
  is_for INTEGER
    REFERENCES posts(id)
)