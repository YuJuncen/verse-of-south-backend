-- Your SQL goes here

CREATE TABLE tags (
    tag_name VARCHAR 
        NOT NULL ,
    id SERIAL
        PRIMARY KEY
);

CREATE TABLE tag_to (
    id SERIAL
        PRIMARY KEY, 
    the_tag INTEGER REFERENCES tags(id)
        ON DELETE CASCADE
        NOT NULL,
    the_post INTEGER REFERENCES posts(id)
        ON DELETE CASCADE
        NOT NULL
);