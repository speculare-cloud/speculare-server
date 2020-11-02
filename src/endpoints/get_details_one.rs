use crate::errors::{AppError, AppErrorType};
use crate::models_db::*;
use crate::models_http::*;
use crate::schema::data::dsl::*;
use crate::Pool;

use actix_identity::Identity;
use actix_web::{get, web, HttpRequest, HttpResponse};
use diesel::prelude::*;

#[get("/speculare/{uuid}")]
pub async fn index(
    id: Identity,
    req: HttpRequest,
    db: web::Data<Pool>,
) -> Result<HttpResponse, AppError> {
    // If the user is not identified, restrict access
    if !id.identity().is_some() {
        return Err(AppError {
            cause: None,
            message: Some("You're not allowed to access this resource".to_string()),
            error_type: AppErrorType::InvalidRequest,
        });
    }

    let data_uuid_i = sanitize_filename::sanitize(req.match_info().query("uuid"));
    if log_enabled!(log::Level::Info) {
        info!("Route GET /speculare/{}", data_uuid_i);
    }

    // Get a connection from the pool
    let conn = db.get()?;

    // Get all HasMany (Many to Many)
    let data_f = data.filter(uuid.eq(&data_uuid_i)).first::<Data>(&conn)?;
    let sensors_f: Vec<Sensors> = Sensors::belonging_to(&data_f).limit(500).load(&conn)?;
    let disks_f: Vec<Disks> = Disks::belonging_to(&data_f).limit(500).load(&conn)?;
    let loadavg_f: Vec<LoadAvg> = LoadAvg::belonging_to(&data_f).limit(500).load(&conn)?;
    let cpuinfo_f: Vec<CpuInfo> = CpuInfo::belonging_to(&data_f).limit(500).load(&conn)?;

    // Retreive the RData
    let rdata = RData {
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
    };

    // Return the data as form of JSON
    Ok(HttpResponse::Ok().json(&rdata))
}
