use diesel::prelude::*;
use diesel::PgConnection;
use serde::{Deserialize};
use super::models::BikeData;

// fields for creating new data row
#[derive(Deserialize)]
pub struct CreateBikeData {
    pub voltage: Option<i32>,
    pub rpm: Option<i32>,
    pub current: Option<i32>,
}

// get some number of data rows for a bike
pub fn find_bike_data(
    conn: &PgConnection,
    bike_id: i32,
    count: i32
) -> Result<Vec<BikeData>, diesel::result::Error> {
    use super::schema::bike_data::dsl::*;

    let data = bike_data
        .filter(bike.eq(bike_id))
        .order(created_at.desc())
        .limit(count as i64)
        .load::<BikeData>(conn)?;

    Ok(data)
}

// create new data row for a bike
pub fn insert_new_bike_data(
    conn: &PgConnection,
    bike_id: i32,
    data: CreateBikeData,
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