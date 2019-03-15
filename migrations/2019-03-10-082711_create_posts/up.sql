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

  /**
  * Diesel seems not support custom type in SQL.
  * Using SMALLINT to present format type.
  * Where WriteDone is a markup language made for 
  *   literature writing by me.
  * ENUM TYPE
  * 1 : MARKDOWN
  * 2 : HTML
  * 3 : PlainText
  * 4 : WriteDone
  */
  body_format SMALLINT
    NOT NULL 
    DEFAULT 1
);
