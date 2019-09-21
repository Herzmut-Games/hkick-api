-- Your SQL goes here
CREATE TABLE teams (
  id INTEGER PRIMARY KEY NOT NULL,
  player_1 INTEGER REFERENCES User(id) NOT NULL,
  player_2 INTEGER REFERENCES User(id) NOT NULL,
  rating INTEGER NOT NULL,
  CHECK (player_1 < player_2)
);
INSERT INTO
  teams(player_1, player_2, rating)
VALUES
  (1, 2, 1000);
