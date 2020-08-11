-- This file should undo anything in `up.sql`
ALTER TABLE trailers;
ADD org INTEGER NOT NULL REFERENCES orgs(org_id) ON DELETE CASCADE DEFAULT 1;

ALTER TABLE users
ADD org INTEGER NOT NULL REFERENCES orgs(org_id) ON DELETE CASCADE DEFAULT 1;

ALTER TABLE users;
DROP COLUMN trailer;
