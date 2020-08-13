-- This file should undo anything in `up.sql`
ALTER TABLE bike_data 
ALTER COLUMN created_at SET DEFAULT now(),
ALTER COLUMN created_at TYPE TIMESTAMP USING now();

ALTER TABLE oven_data
ALTER COLUMN created_at SET DEFAULT now(),
ALTER COLUMN created_at TYPE TIMESTAMP USING now();

ALTER TABLE solar_microgrid_data
ALTER COLUMN created_at SET DEFAULT now(),
ALTER COLUMN created_at TYPE TIMESTAMP USING now();
