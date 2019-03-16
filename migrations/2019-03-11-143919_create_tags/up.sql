-- Your SQL goes here

CREATE TABLE tags (
    tag_name VARCHAR 
        NOT NULL ,
    id SERIAL
        PRIMARY KEY
);

CREATE TABLE tag_to (
    tag_id INTEGER REFERENCES tags(id)
        ON DELETE CASCADE
        NOT NULL,
    post_id INTEGER REFERENCES posts(id)
        ON DELETE CASCADE
        NOT NULL,
    PRIMARY KEY (tag_id, post_id)
);