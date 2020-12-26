use crate::models::{HttpPostHost, NewDisks};

use diesel::{prelude::PgConnection, r2d2::ConnectionManager};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub type ConnType = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

pub type NewDisksList<'a> = Vec<NewDisks<'a>>;

impl<'a> From<&'a HttpPostHost> for NewDisksList<'a> {
    fn from(item: &'a HttpPostHost) -> NewDisksList<'a> {
        let mut list: NewDisksList = Vec::with_capacity(item.disks.len());
        for disk in &item.disks {
            list.push(NewDisks {
                disk_name: &disk.name,
                mount_point: &disk.mount_point,
                total_space: disk.total_space,
                avail_space: disk.avail_space,
                host_uuid: &item.uuid,
                created_at: item.created_at,
            })
        }
        list
    }
}
