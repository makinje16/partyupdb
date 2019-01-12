-- Your SQL goes here
CREATE TABLE players (
  id SERIAL PRIMARY KEY,
  username VARCHAR NOT NULL,
  discord_name VARCHAR NOT NULL,
  discord_id INTEGER NOT NULL,
  rank INTEGER NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 'f'
)
