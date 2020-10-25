#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;

mod errors;
mod models_db;
mod models_http;
mod schema;

use schema::data::dsl::*;
use schema::disks::dsl::*;
use schema::sensors::dsl::*;

use actix_web::{middleware, post, web, App, HttpResponse, HttpServer};
use chrono::prelude::*;
use diesel::dsl::insert_into;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use errors::AppError;
use models_db::*;
use models_http::*;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[post("/endpoints")]
async fn endpoints(db: web::Data<Pool>, item: web::Json<SData>) -> Result<HttpResponse, AppError> {
    if log_enabled!(log::Level::Info) {
        info!("endpoints : {:?}", item);
    }

    // Construct the data to insert into the db
    let mcreated_at = Utc::now().naive_local();
    let new_data = Data {
        os: &item.os,
        hostname: &item.hostname,
        uptime: item.uptime,
        uuid: &item.uuid,
        active_user: &item.user,
        mac_address: &item.mac_address,
        created_at: mcreated_at,
    };
    // Retrieve sensors list from the item
    let mut new_sensors: Vec<InsSensors> = Vec::with_capacity(item.sensors.len());
    for s in &item.sensors {
        new_sensors.push(InsSensors {
            label: &s.label,
            temp: s.temp,
            data_uuid: &item.uuid,
            created_at: mcreated_at,
        });
    }
    // Retrieve disks list from the item
    let mut new_disks: Vec<InsDisks> = Vec::with_capacity(item.disks.len());
    for s in &item.disks {
        new_disks.push(InsDisks {
            disk_name: &s.name,
            mount_point: &s.mount_point,
            total_space: s.total_space,
            avail_space: s.avail_space,
            data_uuid: &item.uuid,
            created_at: mcreated_at,
        });
    }

    let conn = db.get()?;
    // We use a transaction so that if one of the below fail, the previous will be reverted
    conn.transaction::<_, AppError, _>(|| {
        // Insert or update if conflict
        insert_into(data)
            .values(&new_data)
            .on_conflict(uuid)
            .do_update()
            .set(&new_data)
            .execute(&conn)?;
        // Insert the sensors
        insert_into(sensors).values(&new_sensors).execute(&conn)?;
        // Insert the disks
        insert_into(disks).values(&new_disks).execute(&conn)?;
        Ok(())
    })?;
    // Return a 200 status code as everything went well
    Ok(HttpResponse::Ok().finish())
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Load env variable from .env
    dotenv::dotenv().ok();
    // Define the verbose of the logs - info for general and actix
    std::env::set_var("RUST_LOG", "info,actix_server=info,actix_web=info");
    // Init the log module
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .data(pool.clone())
            .service(endpoints)
    })
    .bind(std::env::var("BINDING").expect("Missing binding"))?
    .run()
    .await
}
