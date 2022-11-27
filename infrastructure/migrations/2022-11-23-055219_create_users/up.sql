-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  user_name VARCHAR NOT NULL,
  password VARCHAR NOT NULL
);

ALTER TABLE posts
ADD user_id SERIAL NOT NULL;

ALTER TABLE posts
ADD CONSTRAINT fk_users
FOREIGN KEY(user_id)
REFERENCES users(id);
