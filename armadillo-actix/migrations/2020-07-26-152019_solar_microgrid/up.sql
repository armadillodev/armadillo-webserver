-- Your SQL goes here
CREATE TABLE solar_microgrids (
    solar_microgrid_id SERIAL PRIMARY KEY,
    trailer INTEGER NOT NULL REFERENCES trailers(trailer_id) ON DELETE CASCADE,
    capacity REAL
);

CREATE TABLE solar_microgrid_data (
    solar_microgrid_data_id SERIAL,
    solar_microgrid INTEGER NOT NULL REFERENCES solar_microgrids(solar_microgrid_id) ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    temperature REAL,
    power REAL,
    PRIMARY KEY(solar_microgrid_data_id, created_at)
);

INSERT INTO solar_microgrids (trailer) VALUES (1);
INSERT INTO solar_microgrids (trailer) VALUES (2);