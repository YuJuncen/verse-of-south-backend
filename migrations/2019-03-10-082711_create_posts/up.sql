-- Your SQL goes here


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

  intro TEXT
);

/**
 * the content of a post.
 * stands for the body and some huge data that should not be displayed in the index page.
 * it is a weak entity.
 */
CREATE TABLE post_contents (
   is_for INTEGER
     REFERENCES posts(ID) ON DELETE CASCADE
     NOT NULL
     PRIMARY KEY ,
   body TEXT NOT NULL
);
