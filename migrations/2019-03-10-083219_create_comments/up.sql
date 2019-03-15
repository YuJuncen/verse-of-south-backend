-- Your SQL goes here

CREATE TABLE comments (
  ID SERIAL PRIMARY KEY ,
  publish_time TIMESTAMP WITH TIME ZONE
    NOT NULL
    DEFAULT now(),
  content TEXT NOT NULL ,
  publisher_name VARCHAR NOT NULL ,
  publisher INTEGER
    REFERENCES readers(IP),
  is_for INTEGER
    REFERENCES posts(ID)
)