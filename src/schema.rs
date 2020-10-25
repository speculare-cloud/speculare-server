table! {
    cpu_info (id) {
        id -> Int4,
        cpu_freq -> Int8,
        data_uuid -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    data (uuid) {
        os -> Varchar,
        hostname -> Varchar,
        uptime -> Int8,
        uuid -> Varchar,
        active_user -> Varchar,
        mac_address -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    disks (id) {
        id -> Int4,
        disk_name -> Varchar,
        mount_point -> Varchar,
        total_space -> Int8,
        avail_space -> Int8,
        data_uuid -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    load_avg (id) {
        id -> Int4,
        one -> Float8,
        five -> Float8,
        fifteen -> Float8,
        data_uuid -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    sensors (id) {
        id -> Int4,
        label -> Varchar,
        temp -> Float8,
        data_uuid -> Varchar,
        created_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    cpu_info,
    data,
    disks,
    load_avg,
    sensors,
);
