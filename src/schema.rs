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
    datadisks (id) {
        id -> Int4,
        data_id -> Int4,
        disks_id -> Int4,
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
    disks (id) {
        id -> Int4,
        disk_name -> Varchar,
        mount_point -> Varchar,
        total_space -> Int8,
        avail_space -> Int8,
    }
}

table! {
    sensors (id) {
        id -> Int4,
        label -> Varchar,
        temp -> Float8,
    }
}

allow_tables_to_appear_in_same_query!(
    data,
    datadisks,
    datasensors,
    disks,
    sensors,
);
