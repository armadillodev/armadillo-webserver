table! {
    bike_data (bike_data_id, created_at) {
        bike_data_id -> Int4,
        bike -> Int4,
        created_at -> Timestamp,
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
    oven_data (oven_data_id, created_at) {
        oven_data_id -> Int4,
        oven -> Int4,
        created_at -> Timestamp,
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
    solar_microgrid_data (solar_microgrid_data_id, created_at) {
        solar_microgrid_data_id -> Int4,
        solar_microgrid -> Int4,
        created_at -> Timestamp,
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
    trailer_data (trailer_data_id, created_at) {
        trailer_data_id -> Int4,
        trailer -> Int4,
        created_at -> Timestamp,
        temperature -> Nullable<Int4>,
    }
}

table! {
    trailers (trailer_id) {
        trailer_id -> Int4,
        name -> Text,
        location -> Text,
    }
}

table! {
    user_logs (trailer_log_id) {
        trailer_log_id -> Int4,
        client -> Int4,
        bike -> Int4,
        time_start -> Timestamp,
        time_end -> Timestamp,
    }
}

table! {
    users (user_id) {
        user_id -> Int4,
        first_name -> Text,
        last_name -> Nullable<Text>,
        trailer -> Int4,
    }
}

joinable!(bike_data -> bikes (bike));
joinable!(bikes -> trailers (trailer));
joinable!(oven_data -> ovens (oven));
joinable!(ovens -> trailers (trailer));
joinable!(solar_microgrid_data -> solar_microgrids (solar_microgrid));
joinable!(solar_microgrids -> trailers (trailer));
joinable!(trailer_data -> trailers (trailer));
joinable!(user_logs -> bikes (bike));
joinable!(user_logs -> users (client));
joinable!(users -> trailers (trailer));

allow_tables_to_appear_in_same_query!(
    bike_data,
    bikes,
    oven_data,
    ovens,
    solar_microgrid_data,
    solar_microgrids,
    trailer_data,
    trailers,
    user_logs,
    users,
);
