#[macro_use]
extern crate log;

mod errors;

use actix_web::{web, post, middleware, App, HttpServer, HttpResponse};
use serde::{Serialize, Deserialize};
use errors::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Sensors {
    pub label: String,
    pub temp: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub os: String,
    pub hostname: String,
    pub uptime: u64,
    pub uuid: String,
    pub cpu_freq: u64,
    pub sensors: Vec<Sensors>,
    pub user: String,
    pub mac_address: String,
}

#[post("/endpoints")]
async fn endpoints(
    item: web::Json<Data>,
) -> Result<HttpResponse, AppError> {
    if log_enabled!(log::Level::Info){
        info!("{:?}", item);
    }
    Ok(HttpResponse::Ok().finish())
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info,actix_server=info,actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(endpoints)
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}