use super::models::{Bike, Oven, Solar, Trailer};

use super::DbAccess;
use crate::db::Id;

pub trait TrailerEntity: Sized {
    fn id<A: DbAccess>(db: &A, id: Id) -> Result<Option<Self>, A::E>;
    fn trailer_id<A: DbAccess>(db: &A, trailer_id: Id) -> Result<Vec<Self>, A::E>;
    fn all<A: DbAccess>(db: &A) -> Result<Vec<Self>, A::E>;
}

impl TrailerEntity for Trailer {
    fn id<A: DbAccess>(db: &A, id: Id) -> Result<Option<Self>, A::E> {
        db.find_trailer(id)
    }
    fn trailer_id<A: DbAccess>(_db: &A, _trailer_id: Id) -> Result<Vec<Self>, A::E> {
        panic!("Trailer doesn't belong to a trailer");
    }
    fn all<A: DbAccess>(db: &A) -> Result<Vec<Self>, A::E> {
        db.list_all_trailers()
    }
}

impl TrailerEntity for Bike {
    fn id<A: DbAccess>(db: &A, id: Id) -> Result<Option<Self>, A::E> {
        db.find_bike(id)
    }
    fn trailer_id<A: DbAccess>(db: &A, trailer_id: Id) -> Result<Vec<Self>, A::E> {
        db.find_trailer_bikes(trailer_id)
    }
    fn all<A: DbAccess>(_db: &A) -> Result<Vec<Self>, A::E> {
        panic!("bike lists are unsupported");
    }
}

impl TrailerEntity for Oven {
    fn id<A: DbAccess>(db: &A, id: Id) -> Result<Option<Self>, A::E> {
        db.find_oven(id)
    }
    fn trailer_id<A: DbAccess>(db: &A, trailer_id: Id) -> Result<Vec<Self>, A::E> {
        db.find_trailer_ovens(trailer_id)
    }
    fn all<A: DbAccess>(_db: &A) -> Result<Vec<Self>, A::E> {
        panic!("oven lists are unsupported");
    }
}

impl TrailerEntity for Solar {
    fn id<A: DbAccess>(db: &A, id: Id) -> Result<Option<Self>, A::E> {
        db.find_solar(id)
    }
    fn trailer_id<A: DbAccess>(db: &A, trailer_id: Id) -> Result<Vec<Self>, A::E> {
        db.find_trailer_solars(trailer_id)
    }
    fn all<A: DbAccess>(_db: &A) -> Result<Vec<Self>, A::E> {
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
            match Bike::id(&test_db, id).unwrap() {
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

        assert!(Bike::id(&test_db, 5).unwrap().is_some());
        assert!(Bike::id(&test_db, 6).unwrap().is_none());

        assert_eq!(Bike::trailer_id(&test_db, 5).unwrap().len(), 5);
        assert_eq!(Bike::trailer_id(&test_db, 6).unwrap().len(), 0);
    }

    #[test]
    fn trailer_query_bounds() {
        let test_db = TestDb::new(5);

        assert!(Bike::id(&test_db, 5).unwrap().is_some());
        assert!(Bike::id(&test_db, 6).unwrap().is_none());

        assert_eq!(Bike::trailer_id(&test_db, 5).unwrap().len(), 5);
        assert_eq!(Bike::trailer_id(&test_db, 6).unwrap().len(), 0);
    }
}
