use crate::errors::AppError;
use crate::models_db::*;
use crate::models_http::*;
use crate::schema::cpu_info::dsl::*;
use crate::schema::data::dsl::*;
use crate::schema::disks::dsl::*;
use crate::schema::load_avg::dsl::*;
use crate::schema::sensors::dsl::*;
use crate::Pool;

use actix_web::{post, web, HttpResponse};
use chrono::prelude::*;
use diesel::dsl::insert_into;
use diesel::prelude::*;
use futures::join;

async fn construct_data<'a>(item: &'a SData, now: &'a chrono::NaiveDateTime) -> Data {
    Data {
        os: item.os.to_string(),
        hostname: item.hostname.to_string(),
        uptime: item.uptime,
        uuid: item.uuid.to_string(),
        active_user: item.user.to_string(),
        mac_address: item.mac_address.to_string(),
        created_at: *now,
    }
}

async fn construct_cpuinfo<'a>(item: &'a SData, now: &'a chrono::NaiveDateTime) -> InsCpuInfo<'a> {
    InsCpuInfo {
        cpu_freq: item.cpu_freq,
        data_uuid: &item.uuid,
        created_at: *now,
    }
}

async fn construct_loadavg<'a>(item: &'a SData, now: &'a chrono::NaiveDateTime) -> InsLoadAvg<'a> {
    InsLoadAvg {
        one: item.load_avg.one,
        five: item.load_avg.five,
        fifteen: item.load_avg.fifteen,
        data_uuid: &item.uuid,
        created_at: *now,
    }
}

async fn construct_sensors<'a>(
    item: &'a SData,
    now: &'a chrono::NaiveDateTime,
) -> Vec<InsSensors<'a>> {
    let mut new_sensors: Vec<InsSensors> = Vec::with_capacity(item.sensors.len());
    for s in &item.sensors {
        new_sensors.push(InsSensors {
            label: &s.label,
            temp: s.temp,
            data_uuid: &item.uuid,
            created_at: *now,
        });
    }
    new_sensors
}

async fn construct_disks<'a>(item: &'a SData, now: &'a chrono::NaiveDateTime) -> Vec<InsDisks<'a>> {
    let mut new_disks: Vec<InsDisks> = Vec::with_capacity(item.disks.len());
    for s in &item.disks {
        new_disks.push(InsDisks {
            disk_name: &s.name,
            mount_point: &s.mount_point,
            total_space: s.total_space,
            avail_space: s.avail_space,
            data_uuid: &item.uuid,
            created_at: *now,
        });
    }
    new_disks
}

#[post("/speculare")]
pub async fn index(db: web::Data<Pool>, item: web::Json<SData>) -> Result<HttpResponse, AppError> {
    if log_enabled!(log::Level::Info) {
        info!("Route POST /speculare : {:?}", item);
    }

    // Received time is so the created time
    let mcreated_at = Utc::now().naive_local();

    // Get the Insertable * struct
    let new_data = construct_data(&item, &mcreated_at);
    let new_cpuinfo = construct_cpuinfo(&item, &mcreated_at);
    let new_loadavg = construct_loadavg(&item, &mcreated_at);
    let new_sensors = construct_sensors(&item, &mcreated_at);
    let new_disks = construct_disks(&item, &mcreated_at);

    // Join the previous async block
    let ret = join!(new_data, new_cpuinfo, new_loadavg, new_sensors, new_disks);

    // Get a connection from the pool
    let conn = db.get()?;
    // We use a transaction so that if one of the below fail, the previous will be reverted
    conn.transaction::<_, AppError, _>(|| {
        // Insert or update if conflict
        insert_into(data)
            .values(&ret.0)
            .on_conflict(uuid)
            .do_update()
            .set(&ret.0)
            .execute(&conn)?;
        // Insert cpu_info
        insert_into(cpu_info).values(&ret.1).execute(&conn)?;
        // Insert load_avg
        insert_into(load_avg).values(&ret.2).execute(&conn)?;
        // Insert the sensors
        insert_into(sensors).values(&ret.3).execute(&conn)?;
        // Insert the disks
        insert_into(disks).values(&ret.4).execute(&conn)?;
        Ok(())
    })?;
    // Return a 200 status code as everything went well
    Ok(HttpResponse::Ok().finish())
}
