use diesel::prelude::*;
use diesel::PgConnection;
use serde::Deserialize;
use super::models::BikeData;
use super::models::OvenData;
use super::Address;

pub trait DbData: Send + Sync {
    fn id(&self) -> Address;
    fn to_packet(&self) -> String;
}

pub trait DataQuery: DbData + Sized{
    type NewData: Send;

    fn find(
        conn: &PgConnection,
        id: i32,
        count: i32,
    ) -> Result<Vec<Self>, diesel::result::Error>;

    fn insert(
        _conn: &PgConnection,
        _id: i32,
        _data: Self::NewData,
    ) -> Result<(), diesel::result::Error> {
        Ok(())
    }
}

#[derive(Deserialize)]
pub struct NewOvenData {
    pub oven: i32,
    pub temperature: Option<f32>,
}

// fields for creating new data row
#[derive(Deserialize)]
pub struct CreateBikeData {
    pub voltage: Option<i32>,
    pub rpm: Option<i32>,
    pub current: Option<i32>,
}

impl DbData for OvenData {
    fn id(&self) -> Address {
        Address::Oven(self.id)
    }

    fn to_packet(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl DataQuery for OvenData {
    type NewData = NewOvenData;

    fn find(
        conn: & PgConnection,
        oven_id: i32,
        count: i32,
    ) -> Result<Vec<OvenData>, diesel::result::Error> {
        use super::schema::oven_data::dsl::*;

        let data = oven_data
            .filter(oven.eq(oven_id))
            .order(created_at.desc())
            .limit(count as i64)
            .load::<OvenData>(conn)?;
    
        Ok(data)    
    }
}

impl DbData for BikeData {
    fn id(&self) -> Address {
        Address::Bike(self.id)
    }

    fn to_packet(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl DataQuery for BikeData {
    type NewData = CreateBikeData;
    fn find(
        conn: &PgConnection,
        bike_id: i32,
        count: i32,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use super::schema::bike_data::dsl::*;

        let data = bike_data
            .filter(bike.eq(bike_id))
            .order(created_at.desc())
            .limit(count as i64)
            .load::<BikeData>(conn)?;
    
        Ok(data)    
    }

    fn insert(
        conn: &PgConnection,
        bike_id: i32,
        data: Self::NewData,
    ) -> Result<(), diesel::result::Error> {
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