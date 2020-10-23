table! {
    data (uuid) {
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
    disks (id) {
        id -> Int4,
        disk_name -> Varchar,
        mount_point -> Varchar,
        total_space -> Int8,
        avail_space -> Int8,
        data_uuid -> Varchar,
    }
}

table! {
    sensors (id) {
        id -> Int4,
        label -> Varchar,
        temp -> Float8,
        data_uuid -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(data, disks, sensors,);
