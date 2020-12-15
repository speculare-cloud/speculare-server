use crate::data::models_db::*;
use crate::data::models_http::*;
use crate::data::schema::cpu_info::dsl::*;
use crate::data::schema::disks::dsl::*;
use crate::data::schema::hosts::dsl::*;
use crate::data::schema::load_avg::dsl::*;
use crate::errors::AppError;
use crate::{ConnType, Pool};

use actix_web::{web, HttpResponse};
use chrono::prelude::*;
use diesel::dsl::insert_into;
use diesel::prelude::*;

/// POST /speculare
/// Save data from a host into the db under his uuid
pub async fn index(
    db: web::Data<Pool>,
    item: web::Json<HttpPostData>,
) -> Result<HttpResponse, AppError> {
    if log_enabled!(log::Level::Info) {
        info!("Route POST /speculare : {:?}", item);
    }
    // make all insert taking advantage of web::block to offload the request thread
    web::block(move || insert_all_block(item, db.get()?)).await?;
    // Return a 200 status code as everything went well
    Ok(HttpResponse::Ok().finish())
}

fn insert_all_block(item: web::Json<HttpPostData>, conn: ConnType) -> Result<(), AppError> {
    // Received time is so the created time
    let mcreated_at = Utc::now().naive_local();
    // Construct insertable structure
    let new_data = construct_data(&item, &mcreated_at);
    let new_cpuinfo = construct_cpuinfo(&item, &mcreated_at);
    let new_loadavg = construct_loadavg(&item, &mcreated_at);
    let new_disks = construct_disks(&item, &mcreated_at);
    // Insert or update if conflict
    insert_into(hosts)
        .values(&new_data)
        .on_conflict(uuid)
        .do_update()
        .set(&new_data)
        .execute(&conn)?;
    // Insert cpu_info
    insert_into(cpu_info).values(&new_cpuinfo).execute(&conn)?;
    // Insert load_avg
    insert_into(load_avg).values(&new_loadavg).execute(&conn)?;
    // Insert the disks
    insert_into(disks).values(&new_disks).execute(&conn)?;
    // Return Ok(()) as everything went fine
    Ok(())
}

fn construct_data<'a>(item: &'a HttpPostData, now: &'a chrono::NaiveDateTime) -> Host {
    Host {
        os: item.os.to_string(),
        hostname: item.hostname.to_string(),
        uptime: item.uptime,
        uuid: item.uuid.to_string(),
        created_at: *now,
    }
}

fn construct_cpuinfo<'a>(item: &'a HttpPostData, now: &'a chrono::NaiveDateTime) -> NewCpuInfo<'a> {
    NewCpuInfo {
        cpu_freq: item.cpu_freq,
        host_uuid: &item.uuid,
        created_at: *now,
    }
}

fn construct_loadavg<'a>(item: &'a HttpPostData, now: &'a chrono::NaiveDateTime) -> NewLoadAvg<'a> {
    NewLoadAvg {
        one: item.load_avg.one,
        five: item.load_avg.five,
        fifteen: item.load_avg.fifteen,
        host_uuid: &item.uuid,
        created_at: *now,
    }
}

fn construct_disks<'a>(
    item: &'a HttpPostData,
    now: &'a chrono::NaiveDateTime,
) -> Vec<NewDisks<'a>> {
    let mut new_disks: Vec<NewDisks> = Vec::with_capacity(item.disks.len());
    for s in &item.disks {
        new_disks.push(NewDisks {
            disk_name: &s.name,
            mount_point: &s.mount_point,
            total_space: s.total_space,
            avail_space: s.avail_space,
            host_uuid: &item.uuid,
            created_at: *now,
        });
    }
    new_disks
}
