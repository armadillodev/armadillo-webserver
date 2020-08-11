use crate::db::query::DbAccess;
use crate::db::{Bike, Oven, Solar, Trailer};
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

    // oven methods
    fn find_oven(&self, id: Id) -> Result<Option<Oven>, Self::E> {
        Ok(test_find_method(self.trailer_count, id))
    }
    fn find_trailer_ovens(&self, trailer_id: Id) -> Result<Vec<Oven>, Self::E> {
        Ok(test_find_trailer_method(self.trailer_count, trailer_id))
    }

    // solar methods
    fn find_solar(&self, id: Id) -> Result<Option<Solar>, Self::E> {
        Ok(test_find_method(self.trailer_count, id))
    }
    fn find_trailer_solars(&self, trailer_id: Id) -> Result<Vec<Solar>, Self::E> {
        Ok(test_find_trailer_method(self.trailer_count, trailer_id))
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
}
