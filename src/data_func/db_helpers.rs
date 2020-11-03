use crate::errors::AppError;
use crate::models_db::*;
use crate::models_http::*;
use crate::schema::data::dsl::*;
use crate::ConnType;

use diesel::prelude::*;

/// Return a Vector of Data
/// # Params
/// * `size` - The number of elements to fetch
/// * `page` - How many items you want to skip (page * size)
/// * `conn` - The r2d2 connection needed to fetch the data from the db
pub fn get_data_vec(size: i64, page: i64, conn: ConnType) -> Result<Vec<Data>, AppError> {
    Ok(data.limit(size).offset(page * size).load(&conn)?)
}

/// Return a Vector of RData
/// # Params
/// * `mmuid` - Which object you want to get info from
/// * `conn` - The r2d2 connection needed to fetch the data from the db
pub fn get_rdata(muuid: String, conn: ConnType) -> Result<RData, AppError> {
    // Retrieve all the Many to Many relation to construct the RData
    let data_f = data.filter(uuid.eq(muuid)).first::<Data>(&conn)?;
    let sensors_f: Vec<Sensors> = Sensors::belonging_to(&data_f).limit(500).load(&conn)?;
    let disks_f: Vec<Disks> = Disks::belonging_to(&data_f).limit(500).load(&conn)?;
    let loadavg_f: Vec<LoadAvg> = LoadAvg::belonging_to(&data_f).limit(500).load(&conn)?;
    let cpuinfo_f: Vec<CpuInfo> = CpuInfo::belonging_to(&data_f).limit(500).load(&conn)?;

    // Retreive the RData
    Ok(RData {
        os: data_f.os,
        hostname: data_f.hostname,
        uptime: data_f.uptime,
        uuid: data_f.uuid,
        cpu_freq: cpuinfo_f,
        load_avg: loadavg_f,
        sensors: sensors_f,
        disks: disks_f,
        user: data_f.active_user,
        mac_address: data_f.mac_address,
    })
}
