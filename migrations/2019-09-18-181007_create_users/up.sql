-- Your SQL goes here
CREATE TABLE users (
  id INTEGER PRIMARY KEY NOT NULL,
  first_name VARCHAR NOT NULL,
  surname VARCHAR NOT NULL,
  rating INTEGER NOT NULL
);
INSERT INTO
  users(first_name, surname, rating)
VALUES
  ('Marvin', 'Altemeier', 1000);
INSERT INTO
  users(first_name, surname, rating)
VALUES
  ('Robert', 'Mueller', 1000);
