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
    orgs (org_id) {
        org_id -> Int4,
        name -> Text,
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
        org -> Int4,
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
        org -> Int4,
        first_name -> Text,
        last_name -> Nullable<Text>,
    }
}

joinable!(bike_data -> bikes (bike));
joinable!(bikes -> trailers (trailer));
joinable!(trailer_data -> trailers (trailer));
joinable!(trailers -> orgs (org));
joinable!(user_logs -> bikes (bike));
joinable!(user_logs -> users (client));
joinable!(users -> orgs (org));

allow_tables_to_appear_in_same_query!(
    bike_data,
    bikes,
    orgs,
    trailer_data,
    trailers,
    user_logs,
    users,
);
