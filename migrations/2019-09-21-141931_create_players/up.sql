-- Your SQL goes here
CREATE TABLE players (
  id INTEGER PRIMARY KEY NOT NULL,
  first_name VARCHAR NOT NULL,
  surname VARCHAR NOT NULL,
  nickname VARCHAR NOT NULL,
  rating INTEGER NOT NULL DEFAULT (1000)
);
INSERT INTO
  players(first_name, surname, nickname)
VALUES
  ('Marvin', 'Altemeier', 'Marv'),
  ('Robert', 'Mueller', 'Robert');
