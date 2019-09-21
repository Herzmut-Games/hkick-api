-- Your SQL goes here
CREATE TABLE games(
  id INTEGER PRIMARY KEY NOT NULL,
  match_id INTEGER REFERENCES Match(id) NOT NULL,
  score_team_1 INTEGER NOT NULL,
  score_team_2 INTEGER NOT NULL,
  timestamp DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
  CHECK (score_team_1 != score_team_2)
)
