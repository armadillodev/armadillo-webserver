use crate::db::query::DbAccess;
use crate::db::{Bike, Oven, Solar, Trailer};
use crate::db::{BikeData, OvenData, SolarData};
use crate::db::{Id, Timestamp};

pub struct TestDb {
    trailer_count: u32,
}

impl TestDb {
    pub fn new(trailer_count: u32) -> Self {
        TestDb { trailer_count }
    }
}

fn test_find_method<T: Default>(max: u32, id: Id) -> Option<T> {
    if id > max {
        None
    } else if id <= 0 {
        panic!("Trailed to access invalid id");
    } else {
        Some(T::default())
    }
}

// each trailer will have an equal number of points as its id number
// Example: trailer 3 will have 3 bikes
fn test_find_trailer_method<T: Default>(max: u32, id: Id) -> Vec<T> {
    if id > max {
        return Vec::new();
    }
    if id <= 0 {
        panic!("Trailed to access invalid id");
    }
    (1..=id).map(|_| T::default()).collect::<Vec<_>>()
}

fn test_find_data<T: Default>(id: Id, max: u32, from: Timestamp, until: Timestamp) -> Vec<T> {
    if until < from {
        panic!("time stamp issues. from: {}, until: {}", from, until);
    }

    if id == 0 {
        panic!("invalid id");
    }

    if id > max {
        return Vec::new();
    }

    (from..until).map(|_| T::default()).collect::<Vec<_>>()
}

fn test_insert_data<T: Default>(data: T) -> T {
    data
}

impl DbAccess for TestDb {
    type E = std::convert::Infallible;
    // trailer methods
    fn find_trailer(&self, id: Id) -> Result<Option<Trailer>, Self::E> {
        Ok(test_find_method(self.trailer_count, id))
    }
    fn list_all_trailers(&self) -> Result<Vec<Trailer>, Self::E> {
        Ok((0..self.trailer_count).map(|_| Trailer::default()).collect::<Vec<_>>())
    }

    // bike methods
    fn find_bike(&self, id: Id) -> Result<Option<Bike>, Self::E> {
        Ok(test_find_method(self.trailer_count, id))
    }
    fn find_trailer_bikes(&self, trailer_id: Id) -> Result<Vec<Bike>, Self::E> {
        Ok(test_find_trailer_method(self.trailer_count, trailer_id))
    }

    // bike data methods
    fn find_bike_data(&self, bike_id: Id, from: Timestamp, until: Timestamp) -> Result<Vec<BikeData>, Self::E> {
        Ok(test_find_data(bike_id, self.trailer_count, from, until))
    }
    fn insert_bike_data(&self, bike_data: BikeData) -> Result<BikeData, Self::E> {
        Ok(test_insert_data(bike_data))
    }

    // oven methods
    fn find_oven(&self, id: Id) -> Result<Option<Oven>, Self::E> {
        Ok(test_find_method(self.trailer_count, id))
    }
    fn find_trailer_ovens(&self, trailer_id: Id) -> Result<Vec<Oven>, Self::E> {
        Ok(test_find_trailer_method(self.trailer_count, trailer_id))
    }

    // oven data methods
    fn find_oven_data(&self, oven_id: Id, from: Timestamp, until: Timestamp) -> Result<Vec<OvenData>, Self::E> {
        Ok(test_find_data(oven_id, self.trailer_count, from, until))
    }
    fn insert_oven_data(&self, oven_data: OvenData) -> Result<OvenData, Self::E> {
        Ok(test_insert_data(oven_data))
    }

    // solar methods
    fn find_solar(&self, id: Id) -> Result<Option<Solar>, Self::E> {
        Ok(test_find_method(self.trailer_count, id))
    }
    fn find_trailer_solars(&self, trailer_id: Id) -> Result<Vec<Solar>, Self::E> {
        Ok(test_find_trailer_method(self.trailer_count, trailer_id))
    }

    // solar data methods
    fn find_solar_data(&self, solar_id: Id, from: Timestamp, until: Timestamp) -> Result<Vec<SolarData>, Self::E> {
        Ok(test_find_data(solar_id, self.trailer_count, from, until))
    }
    fn insert_solar_data(&self, solar_data: SolarData) -> Result<SolarData, Self::E> {
        Ok(test_insert_data(solar_data))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_test_trailers() {
        let test_db = TestDb::new(5);

        assert_eq!(test_db.list_all_trailers().unwrap().len(), 5);
    }

    #[test]
    fn find_test_trailer_entities() {
        let test_db = TestDb::new(5);

        assert_eq!(test_db.find_trailer_bikes(3).unwrap().len(), 3);
        assert_eq!(test_db.find_trailer_bikes(5).unwrap().len(), 5);
        assert_eq!(test_db.find_trailer_bikes(6).unwrap().len(), 0);
    }

    #[test]
    fn find_test_entities() {
        let test_db = TestDb::new(5);

        assert!(test_db.find_bike(3).unwrap().is_some());
        assert!(test_db.find_bike(5).unwrap().is_some());
        assert!(test_db.find_bike(6).unwrap().is_none());
    }

    #[test]
    fn find_data() {
        let test_db = TestDb::new(5);

        assert_eq!(test_db.find_bike_data(1, 0, 10).unwrap().len(), 10);
        assert_eq!(test_db.find_bike_data(5, 0, 10).unwrap().len(), 10);
        assert_eq!(test_db.find_bike_data(1, 5, 10).unwrap().len(), 5);
        assert_eq!(test_db.find_bike_data(6, 0, 10).unwrap().len(), 0);
    }

    #[test]
    fn insert_data() {
        let test_db = TestDb::new(5);

        let data = BikeData::default();
        assert_eq!(test_db.insert_bike_data(data.clone()).unwrap(), data);
    }
}
