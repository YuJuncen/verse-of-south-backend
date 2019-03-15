-- Your SQL goes here

CREATE TABLE tags (
    tag_name VARCHAR 
        NOT NULL ,
    ID SERIAL 
        PRIMARY KEY
);

CREATE TABLE tag_to (
    ID SERIAL
        PRIMARY KEY, 
    the_tag INTEGER REFERENCES tags(ID) 
        ON DELETE CASCADE
        NOT NULL,
    the_post INTEGER REFERENCES posts(ID)
        ON DELETE CASCADE
        NOT NULL
);