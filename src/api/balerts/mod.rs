//! For the Alerts, we need to be sure that it's correct.
//! This means we need to construct it and run it to be
//! sure that it does not fails.
//! In order to do that for both update and insert, I decided
//! to implement a new endpoint - named alerts/test - which
//! sole responsibility is to construct and run the query.
//! It will report the result via the REST API (either error)
//! or the result of the query.
//! In case the query was successfull, the alert struct will
//! be hashed with a server's secret and stored in a cache.
//! The cache will be used in insert/update of alerts to be
//! sure that the version passed, is in fact correct and does
//! not need to be checked again

pub mod alerts;
pub mod incidents;
