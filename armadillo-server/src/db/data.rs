use serde::Deserialize;

use super::models::{BikeData, OvenData, SolarData};
use crate::db::DbAccess;
use crate::db::{Id, Timestamp};

pub trait TrailerData: Sized {
    type NewData;
    fn find<A: DbAccess>(db: &A, id: Id, from: Timestamp, until: Timestamp) -> Result<Vec<Self>, A::E>;
    fn insert<A: DbAccess>(db: &A, id: Id, epoch_time: u64, data: Self::NewData) -> Result<Self, A::E>;
}

// structs for creating new data
#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct NewOvenData {
    pub temperature: Option<f32>,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct NewBikeData {
    pub voltage: Option<i32>,
    pub rpm: Option<i32>,
    pub current: Option<i32>,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct NewSolarData {
    pub temperature: Option<f32>,
    pub power: Option<f32>,
}

impl TrailerData for BikeData {
    type NewData = NewBikeData;
    fn find<A: DbAccess>(db: &A, id: Id, from: Timestamp, until: Timestamp) -> Result<Vec<Self>, A::E> {
        db.find_bike_data(id, from, until)
    }
    fn insert<A: DbAccess>(db: &A, id: Id, epoch_time: u64, data: Self::NewData) -> Result<Self, A::E> {
        db.insert_bike_data(BikeData {
            id: 0,
            created_at: epoch_time as i64,
            bike: id as i32,
            voltage: data.voltage,
            current: data.current,
            rpm: data.rpm,
        })
    }
}
impl TrailerData for OvenData {
    type NewData = NewOvenData;
    fn find<A: DbAccess>(db: &A, id: Id, from: Timestamp, until: Timestamp) -> Result<Vec<Self>, A::E> {
        db.find_oven_data(id, from, until)
    }
    fn insert<A: DbAccess>(db: &A, id: Id, epoch_time: u64, data: Self::NewData) -> Result<Self, A::E> {
        db.insert_oven_data(OvenData {
            id: 0,
            created_at: epoch_time as i64,
            oven: id as i32,
            temperature: data.temperature,
        })
    }
}
impl TrailerData for SolarData {
    type NewData = NewSolarData;
    fn find<A: DbAccess>(db: &A, id: Id, from: Timestamp, until: Timestamp) -> Result<Vec<Self>, A::E> {
        db.find_solar_data(id, from, until)
    }
    fn insert<A: DbAccess>(db: &A, id: Id, epoch_time: u64, data: Self::NewData) -> Result<Self, A::E> {
        db.insert_solar_data(SolarData {
            id: 0,
            created_at: epoch_time as i64,
            solar: id as i32,
            temperature: data.temperature,
            power: data.power,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::test_query::TestDb;

    #[test]
    fn insert_bike_data() {
        use crate::time;

        let test_db = TestDb::new(5);

        let test_bike_data = NewBikeData {
            voltage: Some(5),
            current: None,
            rpm: Some(3),
        };

        let now = time::now();

        let insert_data = BikeData::insert(&test_db, 1, now, test_bike_data.clone()).unwrap();

        let expected_data = BikeData {
            bike: 1,
            created_at: now as i64,
            voltage: test_bike_data.voltage,
            current: test_bike_data.current,
            rpm: test_bike_data.rpm,
            ..BikeData::default()
        };

        assert_eq!(insert_data, expected_data);
    }

    #[test]
    fn find_bike_data() {
        let test_db = TestDb::new(5);

        assert_eq!(BikeData::find(&test_db, 3, 0, 10).unwrap().len(), 10);
        assert_eq!(BikeData::find(&test_db, 1, 0, 10).unwrap().len(), 10);
        assert_eq!(BikeData::find(&test_db, 5, 5, 10).unwrap().len(), 5);
        assert_eq!(BikeData::find(&test_db, 3, 0, 10).unwrap().len(), 10);
    }
}
