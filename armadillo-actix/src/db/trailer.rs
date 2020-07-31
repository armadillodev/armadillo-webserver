use diesel::dsl::Eq;
use diesel::dsl::Filter;
use diesel::dsl::Select;
use diesel::prelude::*;

use super::schema::trailers;
use super::Record;

#[rustfmt_skip]
type AllColumns = (
    trailers::trailer_id,
    trailers::name,
    trailers::location,
    trailers::org
);
#[rustfmt_skip]
const ALL_COLUMNS: AllColumns = (
    trailers::trailer_id,
    trailers::name,
    trailers::location,
    trailers::org
);

type All = Select<trailers::table, AllColumns>;
type WithId = Eq<trailers::trailer_id, i32>;
type ById = Filter<All, WithId>;

fn with_id(id: i32) -> WithId {
    trailers::trailer_id.eq(id)
}

pub struct TrailerRecord;

impl Record for TrailerRecord {
    type All = All;
    type ById = ById;

    fn all() -> All {
        trailers::table.select(ALL_COLUMNS)
    }

    fn by_id(id: i32) -> ById {
        Self::all().filter(with_id(id))
    }
}
