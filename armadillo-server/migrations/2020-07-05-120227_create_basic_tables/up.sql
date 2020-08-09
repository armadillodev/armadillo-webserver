CREATE TABLE orgs (
    org_id SERIAL PRIMARY KEY,
    name TEXT NOT NULL unique
);

CREATE TABLE trailers (
    trailer_id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    location TEXT NOT NULL
);

CREATE TABLE users (
    user_id SERIAL PRIMARY KEY,
    trailer INTEGER NOT NULL REFERENCES trailers(trailer_id) ON DELETE CASCADE,
    first_name TEXT NOT NULL,
    last_name TEXT
);

CREATE TABLE bikes (
  bike_id SERIAL PRIMARY KEY,
  trailer INTEGER NOT NULL REFERENCES trailers(trailer_id) ON DELETE CASCADE
);

CREATE TABLE user_logs (
    trailer_log_id SERIAL PRIMARY KEY,
    client INTEGER NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    bike INTEGER NOT NULL REFERENCES bikes(bike_id) ON DELETE CASCADE ,
    time_start TIMESTAMP NOT NULL,
    time_end TIMESTAMP NOT NULL
);

CREATE TABLE trailer_data (
    trailer_data_id SERIAL,
    trailer INTEGER NOT NULL REFERENCES trailers(trailer_id) ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    temperature INTEGER,
    PRIMARY KEY (trailer_data_id, created_at)
);

CREATE TABLE bike_data (
  bike_data_id SERIAL,
  bike INTEGER NOT NULL REFERENCES bikes(bike_id) ON DELETE CASCADE,
  created_at TIMESTAMP NOT NULL DEFAULT now(),
  voltage INTEGER,
  rpm INTEGER,
  current INTEGER,
  PRIMARY KEY (bike_data_id, created_at)
);


INSERT INTO orgs (name) VALUES ('Kai');
INSERT INTO orgs (name) VALUES ('energilab');

INSERT INTO trailers (name, location)
VALUES (
    'Kais House',
    'Tokyo, Japan'
);
INSERT INTO trailers (name, location)
VALUES (
    'Joes fan',
    'Denver, CO'
);

INSERT INTO bikes (trailer)
VALUES (
  (SELECT trailer_id FROM trailers WHERE name='Kais House')
);
