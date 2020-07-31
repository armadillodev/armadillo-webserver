use diesel::dsl::Eq;
use diesel::dsl::Filter;
use diesel::dsl::Select;
use diesel::prelude::*;

use super::schema::oven_data;
use super::Record;

#[rustfmt_skip]
type AllColumns = (
    oven_data::oven_data_id,
    oven_data::oven,
    oven_data::created_at,
    oven_data::temperature,
);

#[rustfmt_skip]
const ALL_COLUMNS: AllColumns = (
    oven_data::oven_data_id,
    oven_data::oven,
    oven_data::created_at,
    oven_data::temperature,
);

type All = Select<oven_data::table, AllColumns>;
type WithId = Eq<oven_data::oven_data_id, i32>;
type ById = Filter<All, WithId>;

fn with_id(id: i32) -> WithId {
    oven_data::oven_data_id.eq(id)
}

pub struct OvenDataRecord;

impl Record for OvenDataRecord {
    type All = All;
    type ById = ById;

    fn all() -> All {
        oven_data::table.select(ALL_COLUMNS)
    }

    fn by_id(id: i32) -> ById {
        Self::all().filter(with_id(id))
    }
}
