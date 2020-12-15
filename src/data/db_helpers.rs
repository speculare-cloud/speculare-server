use crate::data::models_db::*;
use crate::data::models_http::*;
use crate::data::schema::cpu_info::dsl::created_at as cpuinfo_created_at;
use crate::data::schema::disks::dsl::created_at as disk_created_at;
use crate::data::schema::hosts::dsl::{hosts, uuid};
use crate::data::schema::load_avg::dsl::created_at as loadavg_created_at;
use crate::data::schema::memory::dsl::created_at as memory_created_at;
use crate::errors::AppError;
use crate::ConnType;

use diesel::prelude::*;

/// Return a Vector of Host
/// # Params
/// * `size` - The number of elements to fetch
/// * `page` - How many items you want to skip (page * size)
/// * `conn` - The r2d2 connection needed to fetch the data from the db
pub fn get_data_vec(size: i64, page: i64, conn: ConnType) -> Result<Vec<Host>, AppError> {
    Ok(hosts.limit(size).offset(page * size).load(&conn)?)
}

/// Return a Vector of HttpGetData
/// # Params
/// * `mmuid` - Which object you want to get info from
/// * `conn` - The r2d2 connection needed to fetch the data from the db
pub fn get_data_from(muuid: String, conn: ConnType) -> Result<HttpGetData, AppError> {
    // Retrieve the main host from the uuid
    let data_f = hosts.filter(uuid.eq(muuid)).first::<Host>(&conn)?;
    // Retrieve the last Many to Many relation foreach to construct the HttpGetData
    let disks_f: Disks = Disks::belonging_to(&data_f)
        .order(disk_created_at.desc())
        .first(&conn)?;
    let loadavg_f: LoadAvg = LoadAvg::belonging_to(&data_f)
        .order(loadavg_created_at.desc())
        .first(&conn)?;
    let cpuinfo_f: CpuInfo = CpuInfo::belonging_to(&data_f)
        .order(cpuinfo_created_at.desc())
        .first(&conn)?;
    let memory_f: Memory = Memory::belonging_to(&data_f)
        .order(memory_created_at.desc())
        .first(&conn)?;
    // Retreive the HttpGetData
    Ok(HttpGetData {
        os: data_f.os,
        hostname: data_f.hostname,
        uptime: data_f.uptime,
        uuid: data_f.uuid,
        cpu_freq: cpuinfo_f,
        load_avg: loadavg_f,
        disks: disks_f,
        memory: memory_f,
    })
}
