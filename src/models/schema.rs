table! {
    cpu_info (id) {
        id -> Int8,
        cpu_freq -> Int8,
        host_uuid -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    disks (id) {
        id -> Int8,
        disk_name -> Varchar,
        mount_point -> Varchar,
        total_space -> Int8,
        avail_space -> Int8,
        host_uuid -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    hosts (uuid) {
        os -> Varchar,
        hostname -> Varchar,
        uptime -> Int8,
        uuid -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    iostats (id) {
        id -> Int8,
        device_name -> Varchar,
        bytes_read -> Int8,
        bytes_wrtn -> Int8,
        host_uuid -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    load_avg (id) {
        id -> Int8,
        one -> Float8,
        five -> Float8,
        fifteen -> Float8,
        host_uuid -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    memory (id) {
        id -> Int8,
        total_virt -> Int8,
        avail_virt -> Int8,
        total_swap -> Int8,
        avail_swap -> Int8,
        host_uuid -> Varchar,
        created_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(cpu_info, disks, hosts, iostats, load_avg, memory,);
