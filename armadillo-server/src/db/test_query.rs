use crate::db::query::EntityQuery;
use crate::db::{Bike, Oven, Solar, Trailer};
use crate::db::{Id, Timestamp};

struct TestDb {
    trailer_count: u32,
}

impl TestDb {
    fn new(trailer_count: u32) -> Self {
        TestDb { trailer_count }
    }
}

fn test_find_method<T: Default>(max: u32, id: Id) -> Option<T> {
    if id as u32 >= max {
        None
    } else if id < 0 {
        panic!("Trailed to access negative id");
    } else {
        Some(T::default())
    }
}

// each trailer will have an equal number of points as its id number
// Example: trailer 3 will have 3 bikes
fn test_find_trailer_method<T: Default>(max: u32, id: Id) -> Vec<T> {
    if id as u32 >= max {
        return Vec::new();
    }
    if id < 0 {
        panic!("Trailed to access negative id");
    }
    (0..id).map(|_| T::default()).collect::<Vec<_>>()
}

impl EntityQuery for TestDb {
    // trailer methods
    fn find_trailer(&self, id: Id) -> Option<Trailer> {
        test_find_method(self.trailer_count, id)
    }
    fn list_all_trailers(&self) -> Vec<Trailer> {
        (0..self.trailer_count).map(|_| Trailer::default()).collect::<Vec<_>>()
    }

    // bike methods
    fn find_bike(&self, id: Id) -> Option<Bike> {
        test_find_method(self.trailer_count, id)
    }
    fn find_trailer_bikes(&self, trailer_id: Id) -> Vec<Bike> {
        test_find_trailer_method(self.trailer_count, trailer_id)
    }

    // oven methods
    fn find_oven(&self, id: Id) -> Option<Oven> {
        test_find_method(self.trailer_count, id)
    }
    fn find_trailer_ovens(&self, trailer_id: Id) -> Vec<Oven> {
        test_find_trailer_method(self.trailer_count, trailer_id)
    }

    // solar methods
    fn find_solar(&self, id: Id) -> Option<Solar> {
        test_find_method(self.trailer_count, id)
    }
    fn find_trailer_solars(&self, trailer_id: Id) -> Vec<Solar> {
        test_find_trailer_method(self.trailer_count, trailer_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_test_trailers() {
        let test_db = TestDb::new(5);

        assert_eq!(test_db.list_all_trailers().len(), 5);
    }

    #[test]
    fn find_test_trailer_entities() {
        let test_db = TestDb::new(5);

        assert_eq!(test_db.find_trailer_bikes(3).len(), 3);
        assert_eq!(test_db.find_trailer_ovens(3).len(), 3);
        assert_eq!(test_db.find_trailer_solars(3).len(), 3);
    }

    #[test]
    fn find_test_entities() {
        let test_db = TestDb::new(5);

        assert!(test_db.find_bike(3).is_some());
        assert!(test_db.find_bike(6).is_none());

        assert!(test_db.find_oven(3).is_some());
        assert!(test_db.find_oven(6).is_none());

        assert!(test_db.find_solar(3).is_some());
        assert!(test_db.find_solar(6).is_none());
    }
}