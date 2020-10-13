table! {
    data (id) {
        id -> Int4,
        os -> Varchar,
        hostname -> Varchar,
        uptime -> Int8,
        uuid -> Varchar,
        cpu_freq -> Int8,
        active_user -> Varchar,
        mac_address -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    datasensors (id) {
        id -> Int4,
        data_id -> Int4,
        sensors_id -> Int4,
    }
}

table! {
    sensors (id) {
        id -> Int4,
        label -> Varchar,
        temp -> Float8,
    }
}

allow_tables_to_appear_in_same_query!(data, datasensors, sensors,);
