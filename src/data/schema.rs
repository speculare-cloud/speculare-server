table! {
    cpu_info (id) {
        id -> Int4,
        cpu_freq -> Int8,
        host_uuid -> Varchar,
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
    load_avg (id) {
        id -> Int4,
        one -> Float8,
        five -> Float8,
        fifteen -> Float8,
        host_uuid -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    memory (id) {
        id -> Int4,
        total_virt -> Int8,
        avail_virt -> Int8,
        total_swap -> Int8,
        avail_swap -> Int8,
        host_uuid -> Varchar,
        created_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(cpu_info, disks, hosts, load_avg, memory,);
