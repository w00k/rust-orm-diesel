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

INSERT INTO users (name, code_country, number) VALUES 
('Jane Doe', 'CA', 2000),
('Peter Smith', 'NZ', 3000),
('Susan Jones', 'CA', 4000),
('David Williams', 'UK', 9000),
('Emily Johnson', 'CA', 12000),
('Michael Garcia', 'CL', 23000);

INSERT INTO countries (code, name, capital) VALUES 
('US', 'United State', 'Washington, D.C.'),
('UK', 'Engle', 'London'),
('NZ', 'New Zeland', 'Wellington'),
('CL', 'Chile', 'Santiago'),
('CA', 'Canada', 'Ottawa');