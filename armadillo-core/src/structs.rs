#[derive(Eq, PartialEq, Hash, Debug)]
pub struct Org {
    pub id: i32,
    pub name: String,
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Trailer {
    pub id: i32,
    pub org: i32,
    pub name: String,
    pub location: String,
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Bike {
    pub id: i32,
    pub trailer: i32,
}
