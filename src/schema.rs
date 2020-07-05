table! {
    orgs (org_id) {
        org_id -> Integer,
        name -> Text,
    }
}

table! {
    trailor_data (trailor_data_id) {
        trailor_data_id -> Integer,
        trailor -> Integer,
        timestamp -> Integer,
        temperature -> Nullable<Integer>,
    }
}

table! {
    trailor_logs (trailor_log_id) {
        trailor_log_id -> Integer,
        user -> Integer,
        trailor -> Integer,
        time_start -> Integer,
        time_end -> Integer,
    }
}

table! {
    trailors (trailor_id) {
        trailor_id -> Integer,
        name -> Text,
        location -> Text,
        org -> Integer,
    }
}

table! {
    users (user_id) {
        user_id -> Integer,
        first_name -> Text,
        last_name -> Nullable<Text>,
        org -> Integer,
    }
}

joinable!(trailor_data -> trailors (trailor));
joinable!(trailor_logs -> orgs (user));
joinable!(trailor_logs -> trailors (trailor));
joinable!(trailors -> orgs (org));
joinable!(users -> orgs (org));

allow_tables_to_appear_in_same_query!(
    orgs,
    trailor_data,
    trailor_logs,
    trailors,
    users,
);
