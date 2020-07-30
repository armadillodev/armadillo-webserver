#![allow(dead_code)]
use structs::{Bike, Org, Trailer};

mod structs;

pub trait Provider {
    type OrgList: Iterator<Item = i32>;
    type TrailerList: Iterator<Item = i32>;
    type BikeList: Iterator<Item = i32>;

    fn new() -> Self;
    fn get_org(&self, id: i32) -> Option<Org>;
    fn get_org_list(&self) -> Self::OrgList;
    fn get_trailer(&self, id: i32) -> Option<Trailer>;
    fn get_trailer_list(&self) -> Self::TrailerList;
    fn get_bike(&self, id: i32) -> Option<Bike>;
    fn get_bike_list(&self) -> Self::BikeList;
}

#[derive(Debug)]
pub struct Store<T: Provider> {
    provider: T,
}

impl<T: Provider> Store<T> {
    fn new() -> Self {
        Store { provider: T::new() }
    }
}

macro_rules! impl_handle {
    (create $handle_name:ident for $data:ident with $new_handle:ident) => {
        #[derive(Copy, Clone, Debug)]
        struct $handle_name<'a, P: Provider> {
            id: i32,
            store: &'a Store<P>,
        }

        impl<'a, P: Provider> $handle_name<'a, P> {
            fn data(&self) -> Option<$data> {
                self.store.provider.$new_handle(self.id)
            }
        }

        impl<P: Provider> Store<P> {
            fn $new_handle(&self, id: i32) -> $handle_name<P> {
                $handle_name { id, store: self }
            }
        }
    };
}

macro_rules! impl_handle_list {
    ($handle_name:ident with $handle_list:ident) => {
        impl<P: Provider> Store<P> {
            fn $handle_list(&self) -> impl Iterator<Item = $handle_name<P>> {
                self.provider
                    .$handle_list()
                    .map(move |id| $handle_name { id, store: self })
            }
        }
    };
    ($from_handle:ident.$list_method:ident = $to_handle:ident.$full_list:ident map $to_key:ident) => {
        impl<'a, P: Provider> $from_handle<'a, P> {
            fn $list_method(&self) -> impl Iterator<Item = $to_handle<P>> {
                self.store
                    .$full_list()
                    .filter(move |handle| handle.data().unwrap().$to_key == self.id)
            }
        }
    };
}

impl_handle!(create OrgHandle for Org with get_org);
impl_handle_list!(OrgHandle with get_org_list);

impl_handle!(create TrailerHandle for Trailer with get_trailer);
impl_handle_list!(TrailerHandle with get_trailer_list);
impl_handle_list!(OrgHandle.trailers = TrailerHandle.get_trailer_list map org);

impl_handle!(create BikeHandle for Bike with get_bike);
impl_handle_list!(BikeHandle with get_bike_list);
impl_handle_list!(TrailerHandle.bikes = BikeHandle.get_bike_list map trailer);

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct TestProvider;
    impl Provider for TestProvider {
        type OrgList = std::ops::Range<i32>;
        type TrailerList = std::ops::Range<i32>;
        type BikeList = std::ops::Range<i32>;
        fn new() -> Self {
            TestProvider
        }
        fn get_org(&self, id: i32) -> Option<Org> {
            test_org(id)
        }
        fn get_org_list(&self) -> Self::OrgList {
            0..10
        }
        fn get_trailer(&self, id: i32) -> Option<Trailer> {
            test_trailer(id)
        }
        fn get_trailer_list(&self) -> Self::TrailerList {
            0..10
        }
        fn get_bike(&self, id: i32) -> Option<Bike> {
            test_bike(id)
        }
        fn get_bike_list(&self) -> Self::BikeList {
            0..10
        }
    }

    fn test_trailer(id: i32) -> Option<Trailer> {
        Some(Trailer {
            id,
            org: id,
            name: "Hi".into(),
            location: "lost".into(),
        })
    }

    fn test_org(id: i32) -> Option<Org> {
        Some(Org {
            id,
            name: format!("test {}", id),
        })
    }

    fn test_bike(id: i32) -> Option<Bike> {
        Some(Bike { id, trailer: id })
    }

    fn make_test_store() -> Store<TestProvider> {
        Store::new()
    }

    #[test]
    fn get_org_test() {
        let store = make_test_store();
        let org = store.get_org(1);
        assert_eq!(org.data(), test_org(1),);
        let org2 = store.get_org(2);
        assert_eq!(org2.data(), test_org(2),);
        let org3 = store.get_org(3);
        assert_eq!(org3.data(), test_org(3),);
    }

    #[test]
    fn get_trailer_test() {
        let store = make_test_store();
        let trailer = store.get_trailer(1);
        assert_eq!(trailer.data(), test_trailer(1));
        let trailer = store.get_trailer(2);
        assert_eq!(trailer.data(), test_trailer(2));
        let trailer = store.get_trailer(3);
        assert_eq!(trailer.data(), test_trailer(3));
    }

    #[test]
    fn get_bike_test() {
        let store = make_test_store();
        let bike = store.get_bike(1);
        assert_eq!(bike.data(), test_bike(1));
        let bike = store.get_bike(2);
        assert_eq!(bike.data(), test_bike(2));
        let bike = store.get_bike(3);
        assert_eq!(bike.data(), test_bike(3));
    }

    #[test]
    fn get_org_trailer_list() {
        let store = make_test_store();
        let org = store.get_org(1);
        let trailers: Vec<_> = org.trailers().collect();
        // there should only be one trailer with org_id 1
        assert_eq!(trailers.len(), 1);

        let trailer = trailers.get(0).unwrap();
        assert_eq!(trailer.data(), test_trailer(1));
    }

    #[test]
    fn get_trailer_bike_list() {
        let store = make_test_store();
        let trailer = store.get_trailer(1);
        let bikes: Vec<_> = trailer.bikes().collect();
        // there should only be one bike with trailer_id 1
        assert_eq!(bikes.len(), 1);

        let bike = bikes.get(0).unwrap();
        assert_eq!(bike.data(), test_bike(1));
    }
}
