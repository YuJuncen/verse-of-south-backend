CREATE TYPE format AS ENUM ('Markdown', 'HTML', 'PlainText', 'WriteDone');
/**
* The post table.
* each relation maps to one post.
*/
CREATE TABLE posts(
  ID SERIAL
    PRIMARY KEY,

  publish_time TIMESTAMP WITH TIME ZONE
    NOT NULL
    DEFAULT now(),

  title VARCHAR
    NOT NULL ,

  intro TEXT ,

  body TEXT 
    NOT NULL ,

  body_format format
    NOT NULL 
    DEFAULT 'Markdown'
);
