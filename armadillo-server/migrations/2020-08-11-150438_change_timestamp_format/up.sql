-- Your SQL goes here
ALTER TABLE bike_data 
ALTER COLUMN created_at DROP DEFAULT,
ALTER COLUMN created_at TYPE bigint 
USING extract(epoch from created_at);

ALTER TABLE oven_data
ALTER COLUMN created_at DROP DEFAULT,
ALTER COLUMN created_at TYPE bigint 
USING extract(epoch from created_at);

ALTER TABLE solar_microgrid_data
ALTER COLUMN created_at DROP DEFAULT,
ALTER COLUMN created_at TYPE bigint 
USING extract(epoch from created_at);
