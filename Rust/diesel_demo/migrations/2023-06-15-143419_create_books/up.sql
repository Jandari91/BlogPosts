-- Your SQL goes here
CREATE SCHEMA demo

CREATE TABLE demo.posts (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT FALSE
)