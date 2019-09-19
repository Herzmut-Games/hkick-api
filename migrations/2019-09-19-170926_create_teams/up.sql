-- Your SQL goes here
CREATE TABLE teams (
  player1 INTEGER REFERENCES User(id) NOT NULL,
  player2 INTEGER REFERENCES User(id) NOT NULL,
  rating INTEGER NOT NULL,
  PRIMARY KEY (player1, player2)
);
INSERT INTO
  teams(player1, player2, rating)
VALUES
  (1, 2, 1000);
