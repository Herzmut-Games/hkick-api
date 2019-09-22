-- Your SQL goes here
CREATE TABLE teams (
  id INTEGER PRIMARY KEY NOT NULL,
  player_1 INTEGER REFERENCES User(id) NOT NULL,
  player_2 INTEGER REFERENCES User(id) NOT NULL,
  rating FLOAT NOT NULL DEFAULT 1000,
  CHECK (player_1 < player_2)
);
INSERT INTO
  teams(player_1, player_2)
VALUES
  (1, 2);
