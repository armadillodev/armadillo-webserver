use diesel::dsl::Eq;
use diesel::dsl::Filter;
use diesel::dsl::Select;
use diesel::prelude::*;

use super::schema::trailers;

#[rustfmt_skip]
type AllColumns = (
    trailers::trailer_id,
    trailers::name,
    trailers::location,
    trailer::org
);
#[rustfmt_skip]
const ALL_COLUMNS: AllColumns = (
    trailers::trailer_id,
    trailers::name,
    trailers::location,
    trailer::org
);

type All = Select<bikes::table, AllColumns>;
type WithId = Eq<bikes::bike_id, i32>;
type ById = Filter<All, WithId>;

fn with_id(id: i32) -> WithId {
    bikes::bike_id.eq(id)
}

pub struct Trailer;

impl Trailer {
    pub fn all() -> All {
        trailers::table.select(ALL_COLUMNS)
    }

    pub fn by_id(id: i32) -> ById {
        Self::all().filter(with_id(id))
    }
}
