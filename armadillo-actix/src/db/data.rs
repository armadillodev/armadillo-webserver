use diesel::prelude::*;
use diesel::PgConnection;
use serde::Deserialize;

use super::models::{BikeData, OvenData, SolarMicrogridData};
use super::record::{BikeDataRecord, MicrogridDataRecord, OvenDataRecord};

pub trait DataQuery: Sized + Send + Sync {
    type NewData: Send;

    fn find(conn: &PgConnection, id: i32, count: i32) -> Result<Vec<Self>, diesel::result::Error>;
    fn insert(_conn: &PgConnection, _id: i32, _data: Self::NewData) -> Result<(), diesel::result::Error> {
        Ok(())
    }
}

// structs for creating new data
#[derive(Deserialize)]
pub struct NewOvenData {
    pub temperature: Option<f32>,
}

#[derive(Deserialize)]
pub struct NewBikeData {
    pub voltage: Option<i32>,
    pub rpm: Option<i32>,
    pub current: Option<i32>,
}

#[derive(Deserialize)]
pub struct NewMicrogridData {
    pub temperature: Option<f32>,
    pub power: Option<f32>,
}

// impl DbData and DataQuery
// Oven Data
impl DataQuery for OvenData {
    type NewData = NewOvenData;

    fn find(conn: &PgConnection, oven_id: i32, count: i32) -> Result<Vec<OvenData>, diesel::result::Error> {
        use super::schema::oven_data::dsl::*;

        let data = oven_data
            .filter(oven.eq(oven_id))
            .order(created_at.desc())
            .limit(count as i64)
            .load::<OvenData>(conn)?;

        Ok(data)
    }

    fn insert(conn: &PgConnection, oven_id: i32, data: Self::NewData) -> Result<(), diesel::result::Error> {
        use super::schema::oven_data::dsl::*;

        let _data = diesel::insert_into(oven_data)
            .values((oven.eq(oven_id), temperature.eq(data.temperature)))
            .execute(conn)?;

        Ok(())
    }
}

// BikeData
impl DataQuery for BikeData {
    type NewData = NewBikeData;

    fn find(conn: &PgConnection, bike_id: i32, count: i32) -> Result<Vec<Self>, diesel::result::Error> {
        use super::schema::bike_data::dsl::*;

        let data = bike_data
            .filter(bike.eq(bike_id))
            .order(created_at.desc())
            .limit(count as i64)
            .load::<BikeData>(conn)?;

        Ok(data)
    }

    fn insert(conn: &PgConnection, bike_id: i32, data: Self::NewData) -> Result<(), diesel::result::Error> {
        use super::schema::bike_data::dsl::*;

        let _data = diesel::insert_into(bike_data)
            .values((
                bike.eq(bike_id),
                voltage.eq(data.voltage),
                current.eq(data.current),
                rpm.eq(data.rpm),
            ))
            .execute(conn)?;

        Ok(())
    }
}

// SolarMicrogridData
impl DataQuery for SolarMicrogridData {
    type NewData = NewMicrogridData;

    fn find(conn: &PgConnection, solar_microgrid_id: i32, count: i32) -> Result<Vec<Self>, diesel::result::Error> {
        use super::schema::solar_microgrid_data::dsl::*;

        let data = solar_microgrid_data
            .filter(solar_microgrid.eq(solar_microgrid_id))
            .order(created_at.desc())
            .limit(count as i64)
            .load::<SolarMicrogridData>(conn)?;

        Ok(data)
    }

    fn insert(conn: &PgConnection, solar_microgrid_id: i32, data: Self::NewData) -> Result<(), diesel::result::Error> {
        use super::schema::solar_microgrid_data::dsl::*;

        let _data = diesel::insert_into(solar_microgrid_data)
            .values((
                solar_microgrid.eq(solar_microgrid_id),
                power.eq(data.power),
                temperature.eq(data.temperature),
            ))
            .execute(conn)?;

        Ok(())
    }
}
