-- Your SQL goes here
CREATE TABLE players (
  id INTEGER PRIMARY KEY NOT NULL,
  first_name VARCHAR NOT NULL,
  surname VARCHAR NOT NULL,
  nickname VARCHAR NOT NULL,
  rating INTEGER NOT NULL
);
INSERT INTO
  players(first_name, surname, nickname, rating)
VALUES
  ('Marvin', 'Altemeier', 'Marv', 1000),
  ('Robert', 'Mueller', 'Robert', 1000);
