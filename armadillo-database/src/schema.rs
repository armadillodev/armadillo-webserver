table! {
    bike_data (bike, created_at) {
        bike -> Int4,
        created_at -> Int8,
        voltage -> Nullable<Int4>,
        rpm -> Nullable<Int4>,
        current -> Nullable<Int4>,
    }
}

table! {
    bikes (bike_id) {
        bike_id -> Int4,
        trailer -> Int4,
    }
}

table! {
    oven_data (oven, created_at) {
        oven -> Int4,
        created_at -> Int8,
        temperature -> Nullable<Float4>,
    }
}

table! {
    ovens (oven_id) {
        oven_id -> Int4,
        trailer -> Int4,
    }
}

table! {
    solar_microgrid_data (solar_microgrid, created_at) {
        solar_microgrid -> Int4,
        created_at -> Int8,
        temperature -> Nullable<Float4>,
        power -> Nullable<Float4>,
    }
}

table! {
    solar_microgrids (solar_microgrid_id) {
        solar_microgrid_id -> Int4,
        trailer -> Int4,
        capacity -> Nullable<Float4>,
    }
}

table! {
    trailers (trailer_id) {
        trailer_id -> Int4,
        name -> Text,
        location -> Text,
    }
}

joinable!(bike_data -> bikes (bike));
joinable!(bikes -> trailers (trailer));
joinable!(oven_data -> ovens (oven));
joinable!(ovens -> trailers (trailer));
joinable!(solar_microgrid_data -> solar_microgrids (solar_microgrid));
joinable!(solar_microgrids -> trailers (trailer));

allow_tables_to_appear_in_same_query!(
    bike_data,
    bikes,
    oven_data,
    ovens,
    solar_microgrid_data,
    solar_microgrids,
    trailers,
);
