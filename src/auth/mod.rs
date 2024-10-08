use actix_web::{dev, web};
use moka::sync::Cache;
use once_cell::sync::Lazy;
use std::time::Duration;
use uuid::Uuid;

pub mod alert_host_owned;
pub mod alert_owned;
pub mod check_sessions;
pub mod sptk_validator;

static CHECKSESSIONS_CACHE: Lazy<Cache<String, Uuid>> = Lazy::new(|| {
    Cache::builder()
        .time_to_live(Duration::from_secs(60 * 60))
        .build()
});

static CHECKSPTK_CACHE: Lazy<Cache<String, String>> = Lazy::new(|| {
    Cache::builder()
        .time_to_live(Duration::from_secs(60 * 60))
        .build()
});

fn bytes_to_payload(buf: web::Bytes) -> dev::Payload {
    let (_, mut pl) = actix_http::h1::Payload::create(true);
    pl.unread_data(buf);
    dev::Payload::from(pl)
}
