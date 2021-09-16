-- Your SQL goes here

CREATE TABLE message (id SERIAL PRIMARY KEY  , user_id SERIAL  NOT NULL  ,from_id SERIAL NOT NULL,message_type VARCHAR(20) not NULL, message_content TEXT );
