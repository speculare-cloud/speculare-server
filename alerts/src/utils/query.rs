use super::{pct, AbsDTORaw, PctDTORaw, QueryType};

use diesel::{sql_types::Text, *};
use sproot::{models::Alerts, ConnType};

/// Create the query for the Alert and get the QueryType
pub fn construct_query(alert: &Alerts) -> (String, QueryType) {
    // Split the lookup String from the alert for analysis
    let mut lookup_parts = alert.lookup.split(' ');

    // The type of the query
    // this is pretty much the aggregation function Postgres is going to use
    let req_type = if let Some(aggr) = lookup_parts.next() {
        aggr
    } else {
        panic!("Not aggregation function defined");
    };

    // Determine the mode of the query
    // The mode is for now, either Pct or Abs
    let req_mode = if let Some(mode) = lookup_parts.next() {
        match mode {
            "pct" => QueryType::Pct,
            "abs" => QueryType::Abs,
            _ => panic!("Cannot determine the query mode"),
        }
    } else {
        panic!("No mode defined in the query")
    };

    // Get the timing of the query, that is the interval range
    // -> meaning we'll get req_time past datas and get the req_type.
    let req_time = if let Some(time) = lookup_parts.next() {
        time
    } else {
        panic!("Can't determine the time range");
    };

    // This is the columns we ask for in the first place, this value is mandatory
    let req_one = lookup_parts.nth(1);

    // Construct the SELECT part of the query
    let mut pg_select = String::new();
    let select_cols = req_one.unwrap().split(',');
    for col in select_cols {
        // We're casting everything to float8 to handle pretty much any type we need
        pg_select.push_str(&format!("{}({})::float8 + ", req_type, col));
    }
    // Remove the last " + "
    pg_select.drain(pg_select.len() - 3..pg_select.len());
    // Based on the mode, we might need to do some different things
    match req_mode {
        // For pct we need to define numerator and divisor.
        QueryType::Pct => {
            // req_two only exists if req_mode == Pct
            let req_two = lookup_parts.nth(1);

            pg_select.push_str(" as numerator, ");
            let select_cols = req_two.unwrap().split(',');
            for col in select_cols {
                pg_select.push_str(&format!("{}({})::float8 + ", req_type, col));
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
    if alert.where_clause.is_some() {
        pg_where.push_str(" AND ");
        pg_where.push_str(alert.where_clause.as_ref().unwrap());
    };

    // Base of the query, we plug every pieces together here
    let query = format!("SELECT time_bucket('{0}', created_at) as time, {1} FROM {2} WHERE host_uuid=$1 AND created_at > now() at time zone 'utc' - INTERVAL '{0}' {3} GROUP BY time ORDER BY time DESC", req_time, pg_select, alert.table, pg_where);

    trace!("Query[{:?}] is {}", req_mode, &query);
    (query, req_mode)
}

/// This function execute the query based on the QueryType,
/// because all type does not wait for the same result.
pub fn execute_query(query: &str, host_uuid: &str, qtype: &QueryType, conn: &ConnType) -> String {
    // Based on the type we decide which way to go
    // Each type has their own return structure and convertion method (from struct to String).
    match qtype {
        QueryType::Pct => {
            let results = sql_query(query)
                .bind::<Text, _>(host_uuid)
                .load::<PctDTORaw>(conn);
            trace!("result pct is {:?}", &results);
            let results = results.unwrap();
            pct::compute_pct(&results).to_string()
        }
        QueryType::Abs => {
            let results = sql_query(query)
                .bind::<Text, _>(host_uuid)
                .load::<AbsDTORaw>(conn);
            trace!("result abs is {:?}", &results);
            let results = results.unwrap();
            assert!(!results.is_empty());
            results[0].value.to_string()
        }
    }
}
