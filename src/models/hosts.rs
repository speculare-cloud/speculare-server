use crate::errors::AppError;
use crate::types::ConnType;

use super::schema::{
    cpu_info::dsl::created_at as co_created,
    cpu_info::dsl::*,
    disks::dsl::created_at as ds_created,
    disks::dsl::*,
    hosts,
    hosts::dsl::*,
    hosts::dsl::{hosts as dsl_host, uuid},
    iostats::dsl::created_at as is_created,
    iostats::dsl::*,
    load_avg::dsl::created_at as lg_created,
    load_avg::dsl::*,
    memory::dsl::created_at as my_created,
    memory::dsl::*,
};
use super::{
    CpuInfo, Disks, HttpGetHost, HttpPostHost, IoStats, LoadAvg, Memory, NewCpuInfo, NewDisksList,
    NewIostatsList, NewLoadAvg, NewMemory,
};

use diesel::*;
use serde::{Deserialize, Serialize};

// ========================
// DATABASE Specific struct
// ========================
#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize, Insertable, AsChangeset)]
#[table_name = "hosts"]
#[primary_key(uuid)]
pub struct Host {
    pub os: String,
    pub hostname: String,
    pub uptime: i64,
    pub uuid: String,
    pub created_at: chrono::NaiveDateTime,
}

impl Host {
    /// Return a Vector of HttpGetHost
    /// # Params
    /// * `mmuid` - Which object you want to get info from
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    pub fn get(conn: &ConnType, muuid: &str) -> Result<HttpGetHost, AppError> {
        // Retrieve the main host from the uuid
        let data_f = dsl_host.filter(uuid.eq(muuid)).first::<Host>(conn)?;
        // Retrieve the last Many to Many relation foreach to construct the HttpGetHost
        let cpuinfo_f = CpuInfo::belonging_to(&data_f)
            .order(co_created.desc())
            .first::<CpuInfo>(conn)?;
        let loadavg_f = LoadAvg::belonging_to(&data_f)
            .order(lg_created.desc())
            .first::<LoadAvg>(conn)
            .optional()?;
        let memory_f = Memory::belonging_to(&data_f)
            .order(my_created.desc())
            .first::<Memory>(conn)
            .optional()?;
        // Might change to not only get the last one, but rather all previous one
        // with the same time
        let disks_f = Disks::belonging_to(&data_f)
            .order(ds_created.desc())
            .first::<Disks>(conn)
            .optional()?;
        let iostats_f = IoStats::belonging_to(&data_f)
            .order(is_created.desc())
            .first::<IoStats>(conn)
            .optional()?;
        // Return the HttpGetHost struct
        Ok(HttpGetHost {
            os: data_f.os,
            hostname: data_f.hostname,
            uptime: data_f.uptime,
            uuid: data_f.uuid,
            cpu_freq: cpuinfo_f,
            load_avg: loadavg_f,
            disks: disks_f,
            iostats: iostats_f,
            memory: memory_f,
        })
    }

    /// Insert the host data (update or create)
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `item` - The HttpPostHost we just got from the Post request (contains all our info)
    pub fn insert(conn: &ConnType, item: &HttpPostHost) -> Result<(), AppError> {
        // Construct the new Struct from item
        let new_data = Host::from(item);
        let new_cpuinfo = NewCpuInfo::from(item);
        let new_loadavg = Option::<NewLoadAvg>::from(item);
        let new_disks = Option::<NewDisksList>::from(item);
        let new_memory = Option::<NewMemory>::from(item);
        let new_iostats = Option::<NewIostatsList>::from(item);

        // Insert all the Host data
        // for Host, if conflict, only update uptime
        insert_into(hosts)
            .values(&new_data)
            .on_conflict(uuid)
            .do_update()
            .set(uptime.eq(item.uptime))
            .execute(conn)?;
        insert_into(cpu_info).values(&new_cpuinfo).execute(conn)?;
        insert_into(load_avg).values(&new_loadavg).execute(conn)?;
        insert_into(memory).values(&new_memory).execute(conn)?;
        // Need this check as Diesel use a BatchInsert for vec which does not handle
        // None for option as it does not implement the Default constructor
        if new_disks.is_some() {
            insert_into(disks)
                .values(&new_disks.unwrap())
                .execute(conn)?;
        }
        if new_iostats.is_none() {
            insert_into(iostats)
                .values(&new_iostats.unwrap())
                .execute(conn)?;
        }
        // If we reached this point, everything went well
        // So return Ok(())
        Ok(())
    }
}

impl From<&HttpPostHost> for Host {
    fn from(item: &HttpPostHost) -> Host {
        Host {
            os: item.os.to_string(),
            hostname: item.hostname.to_string(),
            uptime: item.uptime,
            uuid: item.uuid.to_string(),
            created_at: item.created_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HostList(pub Vec<Host>);

impl HostList {
    /// Return a Vector of Host
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `size` - The number of elements to fetch
    /// * `page` - How many items you want to skip (page * size)
    pub fn list(conn: &ConnType, size: i64, page: i64) -> Result<Self, AppError> {
        Ok(Self {
            0: dsl_host.limit(size).offset(page * size).load(conn)?,
        })
    }
}
