#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;

mod errors;
mod models;
mod schema;

use schema::data::dsl::*;
use schema::datasensors::dsl::*;
use schema::sensors::dsl::*;

use actix_web::{middleware, post, web, App, HttpResponse, HttpServer};
use chrono::prelude::*;
use diesel::dsl::{exists, insert_into, select};
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use errors::AppError;
use models::*;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[post("/endpoints")]
async fn endpoints(db: web::Data<Pool>, item: web::Json<SData>) -> Result<HttpResponse, AppError> {
    let conn = db.get()?;

    if log_enabled!(log::Level::Info) {
        info!("endpoints : {:?}", item);
    }

    conn.transaction::<_, AppError, _>(|| {
        let new_data = NewData {
            os: &item.os,
            hostname: &item.hostname,
            uptime: item.uptime,
            uuid: &item.uuid,
            cpu_freq: item.cpu_freq,
            active_user: &item.user,
            mac_address: &item.mac_address,
            created_at: Utc::now().naive_local(),
        };

        let mdata: Data = insert_into(data)
            .values(&new_data)
            .on_conflict(uuid)
            .do_update()
            .set(&new_data)
            .get_result(&conn)?;

        let mut new_sensors: Vec<NewSensors> = Vec::new();
        for s in &item.sensors {
            new_sensors.push(NewSensors {
                label: &s.label,
                temp: s.temp,
            });
        }
        let msensors: Vec<Sensors> = insert_into(sensors)
            .values(&new_sensors)
            .get_results(&conn)?;

        let mut new_data_sensors: Vec<NewDataSensors> = Vec::new();
        for s in msensors {
            new_data_sensors.push(NewDataSensors {
                data_id: mdata.id,
                sensors_id: s.id,
            });
        }
        insert_into(datasensors)
            .values(&new_data_sensors)
            .execute(&conn)?;
        Ok(())
    })?;

    Ok(HttpResponse::Ok().finish())
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info,actix_server=info,actix_web=info");
    env_logger::init();

    dotenv::dotenv().ok();

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
    .bind("10.9.1.138:8081")?
    .run()
    .await
}
