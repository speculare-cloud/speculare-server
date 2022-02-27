use crate::utils::DISALLOWED_STATEMENT;

use super::{pct, AbsDTORaw, PctDTORaw, QueryType};

use diesel::{sql_types::Text, *};
use regex::Regex;
use sproot::{
    errors::{AppError, AppErrorType},
    models::Alerts,
    ConnType,
};

lazy_static::lazy_static! {
    static ref INTERVAL_RGX: Regex = {
        match Regex::new(r"(\d+)([a-zA-Z' '])|([m,h,d,minutes,hours,days,minute,hour,day])") {
            Ok(reg) => reg,
            Err(e) => {
                error!("Cannot build the Regex to validate INTERVAL: {}", e);
                std::process::exit(1);
            }
        }
    };
}

/// Create the query for the Alert and get the QueryType
pub fn construct_query(alert: &Alerts) -> Result<(String, QueryType), AppError> {
    // Split the lookup String from the alert for analysis
    let lookup_parts: Vec<&str> = alert.lookup.split(' ').collect();

    // Assert that we have enough parameters
    if lookup_parts.len() < 5 {
        return Err(AppError {
            message: Some("query: the lookup query is invalid, define as follow: [aggr] [mode] [timeframe] of [table] {over} {table}".into()),
            cause: None,
            error_type: AppErrorType::ServerError
        });
    }

    // Determine the mode of the query it's for now, either Pct or Abs
    let req_mode = match lookup_parts[1] {
        "pct" => QueryType::Pct,
        "abs" => QueryType::Abs,
        _ => {
            return Err(AppError {
                message: Some(format!(
                    "query: mode {} is invalid. Valid are: pct, abs.",
                    lookup_parts[1]
                )),
                cause: None,
                error_type: AppErrorType::ServerError,
            });
        }
    };

    // If we're in mode Pct, we need more than 5 parts
    if req_mode == QueryType::Pct && lookup_parts.len() != 7 {
        return Err(AppError {
            message: Some(
                "query: lookup defined as mode pct but missing values, check usage.".into(),
            ),
            cause: None,
            error_type: AppErrorType::ServerError,
        });
    }

    // The type of the query this is pretty much the aggregation function Postgres is going to use
    let req_aggr = lookup_parts[0];
    // Assert that req_type is correct (avg, sum, min, max, count)
    if !["avg", "sum", "min", "max", "count"].contains(&req_aggr) {
        return Err(AppError {
            message: Some("query: aggr is invalid. Valid are: avg, sum, min, max, count.".into()),
            cause: None,
            error_type: AppErrorType::ServerError,
        });
    }

    // Get the timing of the query, that is the interval range
    let req_time = lookup_parts[2];
    // Assert that req_time is correctly formatted (Regex?)
    if !INTERVAL_RGX.is_match(req_time) {
        return Err(AppError {
            message: Some(
                "query: req_time is not correctly formatted (doesn't pass regex).".into(),
            ),
            cause: None,
            error_type: AppErrorType::ServerError,
        });
    }

    // This is the columns we ask for in the first place, this value is mandatory
    let req_one = lookup_parts[4];

    // Construct the SELECT part of the query
    let mut pg_select = String::new();
    let select_cols = req_one.split(',');
    for col in select_cols {
        // We're casting everything to float8 to handle pretty much any type we need
        pg_select.push_str(&format!("{}({})::float8 + ", req_aggr, col));
    }
    // Remove the last " + "
    pg_select.drain(pg_select.len() - 3..pg_select.len());
    // Based on the mode, we might need to do some different things
    match req_mode {
        // For pct we need to define numerator and divisor.
        QueryType::Pct => {
            // req_two only exists if req_mode == Pct
            let req_two = lookup_parts[6];

            pg_select.push_str(" as numerator, ");
            let select_cols = req_two.split(',');
            for col in select_cols {
                pg_select.push_str(&format!("{}({})::float8 + ", req_aggr, col));
            }
            pg_select.drain(pg_select.len() - 3..pg_select.len());
            pg_select.push_str(" as divisor");
        }
        // For abs we just need to define the addition of all columns as value
        QueryType::Abs => {
            pg_select.push_str(" as value");
        }
    }

    // Optional where clause
    // Allow us to add a WHERE condition to the query if needed
    let mut pg_where = String::new();
    if let Some(where_clause) = alert.where_clause.as_ref() {
        pg_where.push_str(" AND ");
        pg_where.push_str(where_clause);
    }

    // Base of the query, we plug every pieces together here
    let query = format!("SELECT time_bucket('{0}', created_at) as time, {1} FROM {2} WHERE host_uuid=$1 AND created_at > now() at time zone 'utc' - INTERVAL '{0}' {3} GROUP BY time ORDER BY time DESC", req_time, pg_select, alert.table, pg_where);

    trace!("Query[{:?}] is {}", req_mode, &query);

    // Assert that we don't have any malicious statement in the query
    // by changing it to uppercase and checking against our list of banned statement.
    let tmp_query = query.to_uppercase();
    for statement in DISALLOWED_STATEMENT {
        if tmp_query.contains(statement) {
            return Err(AppError {
                message: Some(format!(
                    "Alert {} for host_uuid {:.6} contains disallowed statement \"{}\"",
                    alert.name, alert.host_uuid, statement
                )),
                cause: None,
                error_type: AppErrorType::ServerError,
            });
        }
    }

    Ok((query, req_mode))
}

/// This function execute the query based on the QueryType,
/// because all type does not wait for the same result.
pub fn execute_query(
    query: &str,
    host_uuid: &str,
    qtype: &QueryType,
    conn: &ConnType,
) -> Result<String, AppError> {
    // Based on the type we decide which way to go
    // Each type has their own return structure and conversion method (from struct to String).
    match qtype {
        QueryType::Pct => {
            let results = sql_query(query)
                .bind::<Text, _>(host_uuid)
                .load::<PctDTORaw>(conn)?;
            Ok(pct::compute_pct(&results).to_string())
        }
        QueryType::Abs => {
            let results = sql_query(query)
                .bind::<Text, _>(host_uuid)
                .load::<AbsDTORaw>(conn)?;
            trace!("result abs is {:?}", &results);
            if results.is_empty() {
                Err(AppError {
                    message: Some("The result of the query (abs) is empty".into()),
                    cause: None,
                    error_type: AppErrorType::NotFound,
                })
            } else {
                Ok(results[0].value.to_string())
            }
        }
    }
}
