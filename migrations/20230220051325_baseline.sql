-- Add migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create user table
CREATE TABLE "user" (
	id UUID NOT NULL PRIMARY KEY DEFAULT uuid_generate_v4(),
	username VARCHAR ( 255 ) UNIQUE NOT NULL,
	created_on TIMESTAMP NOT NULL,
	updated_on TIMESTAMP NOT NULL
);

-- Create user_movie table
CREATE TABLE "user_movie" (
	id SERIAL PRIMARY KEY,
	movie_id INTEGER NOT NULL,
	user_id UUID,
	title VARCHAR ( 255 ) NOT NULL,
	seen BOOLEAN NOT NULL DEFAULT FALSE,
	watch_again BOOLEAN NOT NULL DEFAULT FALSE,
	rating INTEGER,
	created_on TIMESTAMP NOT NULL,
	updated_on TIMESTAMP NOT NULL
);

-- Insert sample data for user_movie table
INSERT INTO user_movie (movie_id, user_id, title, seen, watch_again, rating, created_on, updated_on)
VALUES
  (1, '84cb5b5c-70cb-4c5f-b9a7-86efbe541b1a', 'Her', true, true, 9, NOW(), NOW()),
  (2, '84cb5b5c-70cb-4c5f-b9a7-86efbe541b1a', 'Eternal Sunshine of the Spotless Mind', true, true, 8, NOW(), NOW()),
  (3, 'f69b2273-3a98-4cb9-9f1c-d4d4d4b4cc22', 'Lost in Translation', true, false, 7, NOW(), NOW()),
  (4, 'f69b2273-3a98-4cb9-9f1c-d4d4d4b4cc22', 'Moonlight', false, false, null, NOW(), NOW()),
  (5, '5e5f5e9f-cfb5-4c77-8f07-2f60e4569305', 'Moonrise Kingdom', false, false, null, NOW(), NOW());

-- Insert sample data for user table
INSERT INTO "user" (id, username, created_on, updated_on)
VALUES
  ('84cb5b5c-70cb-4c5f-b9a7-86efbe541b1a', 'diane', NOW(), NOW()),
  ('f69b2273-3a98-4cb9-9f1c-d4d4d4b4cc22', 'bojack', NOW(), NOW()),
  ('5e5f5e9f-cfb5-4c77-8f07-2f60e4569305', 'princess carolyn', NOW(), NOW());