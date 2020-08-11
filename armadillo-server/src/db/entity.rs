use super::models::{Bike, Oven, Solar, Trailer};

use crate::db::query::DbAccess;
use crate::db::Id;

pub trait TrailerEntity: Sized {
    fn id(db: &impl DbAccess, id: Id) -> Option<Self>;
    fn trailer_id(db: &impl DbAccess, trailer_id: Id) -> Vec<Self>;
    fn all(db: &impl DbAccess) -> Vec<Self>;
}

impl TrailerEntity for Trailer {
    fn id(db: &impl DbAccess, id: Id) -> Option<Self> {
        db.find_trailer(id)
    }
    fn trailer_id(_db: &impl DbAccess, _trailer_id: Id) -> Vec<Self> {
        panic!("Trailer doesn't belong to a trailer");
    }
    fn all(db: &impl DbAccess) -> Vec<Self> {
        db.list_all_trailers()
    }
}

impl TrailerEntity for Bike {
    fn id(db: &impl DbAccess, id: Id) -> Option<Self> {
        db.find_bike(id)
    }
    fn trailer_id(db: &impl DbAccess, trailer_id: Id) -> Vec<Self> {
        db.find_trailer_bikes(trailer_id)
    }
    fn all(_db: &impl DbAccess) -> Vec<Self> {
        panic!("bike lists are unsupported");
    }
}

impl TrailerEntity for Oven {
    fn id(db: &impl DbAccess, id: Id) -> Option<Self> {
        db.find_oven(id)
    }
    fn trailer_id(db: &impl DbAccess, trailer_id: Id) -> Vec<Self> {
        db.find_trailer_ovens(trailer_id)
    }
    fn all(_db: &impl DbAccess) -> Vec<Self> {
        panic!("oven lists are unsupported");
    }
}

impl TrailerEntity for Solar {
    fn id(db: &impl DbAccess, id: Id) -> Option<Self> {
        db.find_solar(id)
    }
    fn trailer_id(db: &impl DbAccess, trailer_id: Id) -> Vec<Self> {
        db.find_trailer_solars(trailer_id)
    }
    fn all(_db: &impl DbAccess) -> Vec<Self> {
        panic!("solar lists are unsupported");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::test_query::TestDb;

    use quickcheck::TestResult;
    use quickcheck_macros::quickcheck as qc;

    #[qc]
    fn bike_query(id: u32) -> TestResult {
        let db_length = 5;
        let test_db = TestDb::new(db_length);

        if id == 0 {
            TestResult::must_fail(move || Bike::id(&test_db, id))
        } else {
            match Bike::id(&test_db, id) {
                Some(_) => {
                    if id <= db_length {
                        TestResult::passed()
                    } else {
                        TestResult::failed()
                    }
                }
                None => {
                    if id > db_length {
                        TestResult::passed()
                    } else {
                        TestResult::failed()
                    }
                }
            }
        }
    }

    #[test]
    fn bike_query_bounds() {
        let test_db = TestDb::new(5);

        assert!(Bike::id(&test_db, 5).is_some());
        assert!(Bike::id(&test_db, 6).is_none());

        assert_eq!(Bike::trailer_id(&test_db, 5).len(), 5);
        assert_eq!(Bike::trailer_id(&test_db, 6).len(), 0);
    }

    #[test]
    fn trailer_query_bounds() {
        let test_db = TestDb::new(5);

        assert!(Bike::id(&test_db, 5).is_some());
        assert!(Bike::id(&test_db, 6).is_none());

        assert_eq!(Bike::trailer_id(&test_db, 5).len(), 5);
        assert_eq!(Bike::trailer_id(&test_db, 6).len(), 0);
    }
}
