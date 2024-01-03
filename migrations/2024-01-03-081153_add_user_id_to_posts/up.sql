-- Your SQL goes here
ALTER TABLE posts
ADD COLUMN user_id UUID REFERENCES users(id) NOT NULL;
