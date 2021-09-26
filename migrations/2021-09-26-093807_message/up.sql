-- Your SQL goes here

CREATE TABLE message (id SERIAL PRIMARY KEY  , user_id SERIAL  NOT NULL  ,destination_id SERIAL NOT NULL,message_type VARCHAR(10) not NULL, message_content TEXT NOT NULL,destination_type VARCHAR(20) NOT NULL,arrive_time TIMESTAMP NOT NULL );
