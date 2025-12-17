-- Add instructions column to recipes table
-- Stores a JSON array of instruction strings

ALTER TABLE recipes ADD COLUMN instructions TEXT NOT NULL DEFAULT '[]';
