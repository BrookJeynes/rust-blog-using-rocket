-- This file should undo anything in `up.sql`
DROP TABLE users

ALTER TABLE posts
DROP CONSTRAINT user_id 
