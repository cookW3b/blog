-- Your SQL goes here
CREATE TABLE users (
  id UUID PRIMARY KEY,
  username VARCHAR NOT NULL,
  name VARCHAR NOT NULL,
  password TEXT NOT NULL
)
