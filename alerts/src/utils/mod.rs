use diesel::{
    sql_types::{Float8, Text, Timestamp},
    *,
};
use evalexpr::*;
use sproot::{models::Alerts, ConnType};

pub mod abs;
pub mod pct;

#[derive(Debug)]
pub enum QueryType {
    Pct,
    Abs,
}

/// Struct to hold the return from the sql_query for percentage query
#[derive(QueryableByName, Debug)]
pub struct PctDTORaw {
    #[sql_type = "Float8"]
    pub numerator: f64,
    #[sql_type = "Float8"]
    pub divisor: f64,
    #[sql_type = "Timestamp"]
    pub time: chrono::NaiveDateTime,
}

/// Struct to hold the return from the sql_query for absolute query
#[derive(QueryableByName, Debug)]
pub struct AbsDTORaw {
    #[sql_type = "Float8"]
    pub value: f64,
    #[sql_type = "Timestamp"]
    pub time: chrono::NaiveDateTime,
}

/// Constant list of disallowed statement in the SQL query to avoid somthg bad
const DISALLOWED_STATEMENT: &[&str] = &[
    "DELETE",
    "UPDATE",
    "INSERT",
    "CREATE",
    "ALTER",
    "DROP",
    "TRUNCATE",
    "GRANT",
    "REVOKE",
    "BEGIN",
    "COMMIT",
    "SAVEPOINT",
    "ROLLBACK",
];

pub fn construct_query(alert: &Alerts) -> (String, QueryType) {
    let mut lookup_parts = alert.lookup.split(' ');

    let req_type = if let Some(aggr) = lookup_parts.next() {
        aggr
    } else {
        panic!("Not aggregation function defined");
    };

    let req_mode = if let Some(mode) = lookup_parts.next() {
        match mode {
            "pct" => QueryType::Pct,
            "abs" => QueryType::Abs,
            _ => panic!("Cannot determine the query mode"),
        }
    } else {
        panic!("No mode defined in the query")
    };

    let req_time = if let Some(time) = lookup_parts.next() {
        time
    } else {
        panic!("Can't determine the time range");
    };

    let req_one = lookup_parts.nth(1);

    let mut pg_select = String::new();
    let select_cols = req_one.unwrap().split(',');
    for col in select_cols {
        pg_select.push_str(&format!("{}({})::float8 + ", req_type, col));
    }
    pg_select.drain(pg_select.len() - 3..pg_select.len());
    match req_mode {
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
        QueryType::Abs => {
            pg_select.push_str(" as value");
        }
    }

    let mut pg_where = String::new();
    if alert.where_clause.is_some() {
        pg_where.push_str(" AND ");
        pg_where.push_str(alert.where_clause.as_ref().unwrap());
    };

    let query = format!("SELECT time_bucket('{0}', created_at) as time, {1} FROM {2} WHERE host_uuid=$1 AND created_at > now() at time zone 'utc' - INTERVAL '{0}' {3} GROUP BY time ORDER BY time DESC", req_time, pg_select, alert.table, pg_where);

    dbg!(&query, &req_mode);
    (query, req_mode)
}

pub fn execute_query(query: &str, host_uuid: &str, qtype: &QueryType, conn: &ConnType) -> String {
    let tmp_query = query.to_uppercase();
    for statement in DISALLOWED_STATEMENT {
        assert!(!tmp_query.contains(statement));
    }

    match qtype {
        QueryType::Pct => {
            let results = sql_query(query)
                .bind::<Text, _>(host_uuid)
                .load::<PctDTORaw>(conn);
            dbg!(&results);
            pct::compute_pct(&results.unwrap()).to_string()
        }
        QueryType::Abs => {
            let results = sql_query(query)
                .bind::<Text, _>(host_uuid)
                .load::<AbsDTORaw>(conn);
            dbg!(&results);
            let results = results.unwrap();
            assert!(results.len() == 1);
            results[0].value.to_string()
        }
    }
}

pub fn execute(query: &str, alert: &Alerts, qtype: &QueryType, conn: &ConnType) {
    let result = execute_query(query, &alert.host_uuid, qtype, conn);

    let should_warn = eval_boolean(&alert.warn.replace("$this", &result));
    let should_crit = eval_boolean(&alert.crit.replace("$this", &result));

    dbg!(should_warn, should_crit);
}
