-- Your SQL goes here
-- remove duplicates
DELETE FROM bike_data a USING (
      SELECT MIN(ctid) as ctid, created_at
        FROM bike_data 
        GROUP BY created_at HAVING COUNT(*) > 1
      ) b
      WHERE a.created_at = b.created_at 
      AND a.ctid <> b.ctid;

DELETE FROM oven_data a USING (
      SELECT MIN(ctid) as ctid, created_at
        FROM oven_data 
        GROUP BY created_at HAVING COUNT(*) > 1
      ) b
      WHERE a.created_at = b.created_at 
      AND a.ctid <> b.ctid;

DELETE FROM solar_microgrid_data a USING (
      SELECT MIN(ctid) as ctid, created_at
        FROM solar_microgrid_data 
        GROUP BY created_at HAVING COUNT(*) > 1
      ) b
      WHERE a.created_at = b.created_at 
      AND a.ctid <> b.ctid;

ALTER TABLE "bike_data"
ADD CONSTRAINT "bike_data_pk" PRIMARY KEY ("bike", "created_at"),
DROP CONSTRAINT "bike_data_pk";

ALTER TABLE "bike_data"
DROP "bike_data_id";

ALTER TABLE "oven_data"
ADD CONSTRAINT "oven_data_pk" PRIMARY KEY ("oven", "created_at"),
DROP CONSTRAINT "oven_data_pk";

ALTER TABLE "oven_data"
DROP "oven_data_id";

ALTER TABLE "solar_microgrid_data"
ADD CONSTRAINT "solar_microgrid_data_pk" PRIMARY KEY ("solar_microgrid", "created_at"),
DROP CONSTRAINT "solar_microgrid_data_pk";

ALTER TABLE "solar_microgrid_data"
DROP "solar_microgrid_data_id";
