-- Your SQL goes here
CREATE TABLE IF NOT EXISTS users (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  code_country VARCHAR NOT NULL,
  number integer
);

CREATE TABLE IF NOT EXISTS countries (
  id SERIAL PRIMARY KEY,
  code VARCHAR NOT NULL,
  name VARCHAR NOT NULL,
  capital VARCHAR NOT NULL
);