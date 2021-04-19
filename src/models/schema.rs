table! {
    cpustats (id) {
        id -> Int8,
        cuser -> Int8,
        nice -> Int8,
        system -> Int8,
        idle -> Int8,
        iowait -> Int8,
        irq -> Int8,
        softirq -> Int8,
        steal -> Int8,
        guest -> Int8,
        guest_nice -> Int8,
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
        system -> Varchar,
        os_version -> Varchar,
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

allow_tables_to_appear_in_same_query!(cpustats, disks, hosts, iostats, load_avg, memory,);
