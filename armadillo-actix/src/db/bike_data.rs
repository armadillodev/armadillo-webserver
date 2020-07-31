use diesel::dsl::Eq;
use diesel::dsl::Filter;
use diesel::dsl::Select;
use diesel::prelude::*;

use super::schema::bike_data;
use super::Record;

#[rustfmt_skip]
type AllColumns = (
    bike_data::bike_data_id,
    bike_data::bike,
    bike_data::created_at,
    bike_data::voltage,
    bike_data::rpm,
    bike_data::current,
);
#[rustfmt_skip]
const ALL_COLUMNS: AllColumns = (
    bike_data::bike_data_id,
    bike_data::bike,
    bike_data::created_at,
    bike_data::voltage,
    bike_data::rpm,
    bike_data::current,
);

type All = Select<bike_data::table, AllColumns>;
type WithId = Eq<bike_data::bike_data_id, i32>;
type ById = Filter<All, WithId>;

fn with_id(id: i32) -> WithId {
    bike_data::bike_data_id.eq(id)
}

pub struct BikeDataRecord;

impl Record for BikeDataRecord {
    type All = All;
    type ById = ById;
    fn all() -> All {
        bike_data::table.select(ALL_COLUMNS)
    }

    fn by_id(id: i32) -> ById {
        Self::all().filter(with_id(id))
    }
}
