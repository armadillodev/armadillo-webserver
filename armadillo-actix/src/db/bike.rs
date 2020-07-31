use diesel::dsl::Eq;
use diesel::dsl::Filter;
use diesel::dsl::Select;
use diesel::prelude::*;

use super::schema::bikes;

type AllColumns = (bikes::bike_id, bikes::trailer);
const ALL_COLUMNS: AllColumns = (bikes::bike_id, bikes::trailer);

type All = Select<bikes::table, AllColumns>;
type WithId = Eq<bikes::bike_id, i32>;
type ById = Filter<All, WithId>;

pub struct Bike;

fn with_id(id: i32) -> WithId {
    bikes::bike_id.eq(id)
}

impl Bike {
    pub fn all() -> All {
        bikes::table.select(ALL_COLUMNS)
    }

    pub fn by_id(id: i32) -> ById {
        Self::all().filter(with_id(id))
    }
}
