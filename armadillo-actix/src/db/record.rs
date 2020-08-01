pub trait Record {
    type All;
    type ById;

    fn all() -> Self::All;
    fn by_id(id: i32) -> Self::ById;
}

pub trait DataRecord: Record {
    type ByKeyId;

    fn by_key_id(id: i32) -> Self::ByKeyId;
}

macro_rules! make_record(
    (data_record $record_name:ident, $schema:ident, $key_id:ident) => {
        type WithKeyId = Eq<$schema::$key_id, i32>;
        type ByKeyId = Filter<All, WithKeyId>;

        fn with_key_id(id: i32) -> WithKeyId {
            $schema::$key_id.eq(id)
        }

        impl DataRecord for $record_name {
            type ByKeyId = ByKeyId;

            fn by_key_id(id: i32) -> ByKeyId {
                Self::all().filter(with_key_id(id))
            }
        }
    };
    (record $record_name:ident, $schema:ident, $id:ident, $columns:tt) => {
        use diesel::dsl::Eq;
        use diesel::dsl::Filter;
        use diesel::dsl::Select;
        use diesel::prelude::*;

        use crate::db::schema::$schema;

        type AllColumns = $columns;
        const ALL_COLUMNS: AllColumns = $columns;

        type All = Select<$schema::table, AllColumns>;
        type WithId = Eq<$schema::$id, i32>;
        type ById = Filter<All, WithId>;

        fn with_id(id: i32) -> WithId {
            $schema::$id.eq(id)
        }

        pub struct $record_name;

        impl Record for $record_name {
            type All = All;
            type ById = ById;

            fn all() -> All {
                $schema::table.select(ALL_COLUMNS)
            }

            fn by_id(id: i32) -> ById {
                Self::all().filter(with_id(id))
            }
        }
    };
    ($mod_name:ident, $record_name:ident, $schema:ident, $id:ident, $columns:tt) => {
        mod $mod_name {
            use super::Record;

            make_record!(record $record_name, $schema, $id, $columns);
        }
        pub use $mod_name::$record_name;
    };
    (data $mod_name:ident, $record_name:ident, $schema:ident, $id:ident, $key_id:ident, $columns:tt) => {
        mod $mod_name {
            use super::Record;
            use super::DataRecord;

            make_record!(record $record_name, $schema, $id, $columns);
            make_record!(data_record $record_name, $schema, $key_id);
        }
        pub use $mod_name::$record_name;
    };
);

// bike
#[rustfmt::skip]
make_record!(bike, BikeRecord, bikes, bike_id, (
    bikes::bike_id,
    bikes::trailer,
));

// bike data
#[rustfmt::skip]
make_record!(data bike_data, BikeDataRecord, bike_data, bike_data_id, bike, (
    bike_data::bike_data_id,
    bike_data::bike,
    bike_data::created_at,
    bike_data::voltage,
    bike_data::rpm,
    bike_data::current,
));

// microgrid
#[rustfmt::skip]
make_record!(microgrid, MicrogridRecord, solar_microgrids, solar_microgrid_id, (
    solar_microgrids::solar_microgrid_id,
    solar_microgrids::trailer,
));

// microgrid data
#[rustfmt::skip]
make_record!(data microgrid_data, MicrogridDataRecord, solar_microgrid_data, solar_microgrid_data_id, solar_microgrid, (
    solar_microgrid_data::solar_microgrid_data_id,
    solar_microgrid_data::solar_microgrid,
    solar_microgrid_data::created_at,
    solar_microgrid_data::temperature,
    solar_microgrid_data::power,
));

// oven
#[rustfmt::skip]
make_record!(oven, OvenRecord, ovens, oven_id, (
    ovens::oven_id,
    ovens::trailer,
));

// oven data
#[rustfmt::skip]
make_record!(data oven_data, OvenDataRecord, oven_data, oven_data_id, oven, (
    oven_data::oven_data_id,
    oven_data::oven,
    oven_data::created_at,
    oven_data::temperature,
));

// trailer
#[rustfmt::skip]
make_record!(trailer, TrailerRecord, trailers, trailer_id, (
    trailers::trailer_id,
    trailers::name,
    trailers::location,
    trailers::org
));

// org
#[rustfmt::skip]
make_record!(org, OrgRecord, orgs, org_id, (
    orgs::org_id,
    orgs::name,
));
