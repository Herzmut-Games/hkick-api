-- Your SQL goes here

CREATE TABLE players
(
  id INTEGER PRIMARY KEY NOT NULL,
  first_name VARCHAR NOT NULL,
  surname VARCHAR NOT NULL,
  nickname VARCHAR NOT NULL,
  team_rating INTEGER NOT NULL DEFAULT 1000,
  solo_rating INTEGER NOT NULL DEFAULT 1000
);

CREATE TABLE teams
(
  id INTEGER PRIMARY KEY NOT NULL,
  player_1 INTEGER NOT NULL REFERENCES players(id),
  player_2 INTEGER NOT NULL REFERENCES players(id),
  rating INTEGER NOT NULL DEFAULT 1000,

  CHECK (player_1 < player_2)
);

CREATE TABLE matches
(
  id INTEGER PRIMARY KEY NOT NULL,
  team_1 INTEGER NOT NULL REFERENCES teams(id),
  team_2 INTEGER NOT NULL REFERENCES teams(id),
  timestamp DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,

  CHECK (team_1 != team_2)
);

CREATE TABLE duels
(
  id INTEGER PRIMARY KEY NOT NULL,
  player_1 INTEGER NOT NULL REFERENCES players(id),
  player_2 INTEGER NOT NULL REFERENCES players(id),
  timestamp DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,

  CHECK (player_1 != player_2)
);

CREATE TABLE games
(
  id INTEGER PRIMARY KEY NOT NULL,
  match_id INTEGER REFERENCES matches(id),
  duel_id INTEGER REFERENCES duels(id),
  score_1 INTEGER NOT NULL,
  score_2 INTEGER NOT NULL,
  timestamp DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,

  CHECK (score_1 != score_2)
);

INSERT INTO
  players
  (first_name, surname, nickname)
VALUES
  ('Marvin', 'Altemeier', 'Marv'),
  ('Patrick', 'Schaffrath', 'Patrick'),
  ('Joshua', 'Grimm', 'Joshi'),
  ('Robert', 'Müller', 'Robert'),
  ('Anni', 'Iltgen', 'Anni'),
  ('Christopher', 'Stöckl', 'Stöckl');
