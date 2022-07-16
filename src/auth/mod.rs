use crate::{flow_run, CONFIG};

use moka::future::Cache;
use once_cell::sync::Lazy;
use sproot::Pool;
use std::time::Duration;
use uuid::Uuid;

pub mod checksessions;
pub mod sptkvalidator;

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
pub static AUTHPOOL: Lazy<Pool> = Lazy::new(|| {
    flow_run::build_pool(
        &CONFIG.auth_database_url,
        CONFIG.auth_database_max_connection,
    )
});
