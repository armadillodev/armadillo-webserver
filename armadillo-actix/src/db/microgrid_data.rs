use diesel::dsl::Eq;
use diesel::dsl::Filter;
use diesel::dsl::Select;
use diesel::prelude::*;

use super::schema::solar_microgrid_data;
use super::Record;

#[rustfmt_skip]
type AllColumns = (
    solar_microgrid_data::solar_microgrid_data_id,
    solar_microgrid_data::solar_microgrid,
    solar_microgrid_data::created_at,
    solar_microgrid_data::temperature,
    solar_microgrid_data::power,
);
#[rustfmt_skip]
const ALL_COLUMNS: AllColumns = (
    solar_microgrid_data::solar_microgrid_data_id,
    solar_microgrid_data::solar_microgrid,
    solar_microgrid_data::created_at,
    solar_microgrid_data::temperature,
    solar_microgrid_data::power,
);

type All = Select<solar_microgrid_data::table, AllColumns>;
type WithId = Eq<solar_microgrid_data::solar_microgrid_data_id, i32>;
type ById = Filter<All, WithId>;

fn with_id(id: i32) -> WithId {
    solar_microgrid_data::solar_microgrid_data_id.eq(id)
}

pub struct MicrogridDataRecord;

impl Record for MicrogridDataRecord {
    type All = All;
    type ById = ById;

    fn all() -> All {
        solar_microgrid_data::table.select(ALL_COLUMNS)
    }

    fn by_id(id: i32) -> ById {
        Self::all().filter(with_id(id))
    }
}
