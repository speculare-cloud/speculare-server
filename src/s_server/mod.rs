mod api;
mod routes;
pub mod server;

pub use server::*;

use std::env::VarError;

// Lazy static of the Token from .env to use in validator
lazy_static::lazy_static! {
    static ref TOKEN: Result<String, VarError> = {
        std::env::var("API_TOKEN")
    };
}
