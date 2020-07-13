CREATE TABLE orgs (
    org_id SERIAL PRIMARY KEY,
    name TEXT NOT NULL unique
);

CREATE TABLE trailers (
    trailer_id SERIAL NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    location TEXT NOT NULL,
    org INTEGER NOT NULL REFERENCES orgs(org_id)
);

CREATE TABLE users (
    user_id SERIAL PRIMARY KEY,
    org INTEGER NOT NULL REFERENCES orgs(org_id),
    first_name TEXT NOT NULL,
    last_name TEXT
);

CREATE TABLE bikes (
  bike_id SERIAL PRIMARY KEY,
  trailer INTEGER NOT NULL REFERENCES trailers(trailer_id)
);

CREATE TABLE user_logs (
    trailer_log_id SERIAL PRIMARY KEY,
    client INTEGER NOT NULL REFERENCES users(user_id),
    bike INTEGER NOT NULL REFERENCES bikes(bike_id),
    time_start TIMESTAMP NOT NULL,
    time_end TIMESTAMP NOT NULL
);

CREATE TABLE trailer_data (
    trailer_data_id SERIAL,
    trailer INTEGER NOT NULL REFERENCES trailers(trailer_id),
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    temperature INTEGER,
    PRIMARY KEY (trailer_data_id, created_at)
);

CREATE TABLE bike_data (
  bike_data_id SERIAL,
  bike INTEGER NOT NULL REFERENCES bikes(bike_id),
  created_at TIMESTAMP NOT NULL DEFAULT now(),
  voltage INTEGER,
  rpm INTEGER,
  current INTEGER,
  PRIMARY KEY (bike_data_id, created_at)
);


INSERT INTO orgs (name) VALUES ('Kai');
INSERT INTO orgs (name) VALUES ('energilab');

INSERT INTO trailers (name, location, org)
VALUES (
    'Kais House',
    'Tokyo, Japan',
    (SELECT org_id FROM orgs WHERE name='Kai')
);
INSERT INTO trailers (name, location, org)
VALUES (
    'Joes fan',
    'Denver, CO',
    (SELECT org_id FROM orgs WHERE name='energilab')
);

INSERT INTO bikes (trailer)
VALUES (
  (SELECT trailer_id FROM trailers WHERE name='Kais House')
);

INSERT INTO users (first_name, last_name, org)
VALUES (
    'Kai',
    'Dewey',
    (SELECT org_id FROM orgs WHERE name='Kai')
);
INSERT INTO users (first_name, org)
VALUES (
    'Joe',
    (SELECT org_id FROM orgs WHERE name='energilab')
);
