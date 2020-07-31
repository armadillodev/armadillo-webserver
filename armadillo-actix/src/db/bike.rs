use diesel::dsl::Eq;
use diesel::dsl::Filter;
use diesel::dsl::Select;
use diesel::prelude::*;

use super::schema::bikes;
use super::Record;

type AllColumns = (bikes::bike_id, bikes::trailer);
const ALL_COLUMNS: AllColumns = (bikes::bike_id, bikes::trailer);

type All = Select<bikes::table, AllColumns>;
type WithId = Eq<bikes::bike_id, i32>;
type ById = Filter<All, WithId>;

fn with_id(id: i32) -> WithId {
    bikes::bike_id.eq(id)
}

pub struct Bike;

impl Record for Bike {
    type All = All;
    type ById = ById;

    fn all() -> All {
        bikes::table.select(ALL_COLUMNS)
    }

    fn by_id(id: i32) -> ById {
        Self::all().filter(with_id(id))
    }
}
