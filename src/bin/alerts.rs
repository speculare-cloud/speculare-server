#[macro_use]
extern crate diesel_migrations;

use diesel::{prelude::PgConnection, r2d2::ConnectionManager};
use evalexpr::*;
use serde::{Deserialize, Serialize};

// TEST
use diesel::{
    sql_types::{Int8, Text, Timestamp},
    *,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Alerts {
    pub name: String,   // Name of the alarms (only _ is allowed)
    pub table: String,  // Table name targeted
    pub lookup: String, // average 10m percentage of w,x over y,z
    // (will compute the (10m avg(w)+avg(x) over avg(y)+avg(z)) * 100, result is in percentage as asked using percentage and over)
    pub timing: i64,                  // Number of seconds between checks
    pub warn: String, // $this > 50 ($this refer to the result of the query, should return a bool)
    pub crit: String, // $this > 80 ($this refer to the result of the query, should return a bool)
    pub info: String, // Description of the alarms
    pub host_uuid: String, // Targeted host
    pub where_clause: Option<String>, // Where SQL condition
}

#[derive(QueryableByName, Debug)]
pub struct DTORaw {
    #[sql_type = "Int8"]
    pub numerator: i64,
    #[sql_type = "Int8"]
    pub divisor: i64,
    #[sql_type = "Timestamp"]
    pub time: chrono::NaiveDateTime,
}

// // Embed migrations into the binary
embed_migrations!();

/// Compute the percentage of difference between a Vec containing two DTORaw
///
/// This give us the percentage of use of results[1] over results[0].
fn compute_percentage(results: &[DTORaw]) -> f64 {
    // results must contains exactly two items.
    assert!(results.len() == 2);

    // Define temp variable
    // results[0] is the previous value in time
    // results[1] is the current value
    let (prev_div, curr_div) = (results[1].divisor, results[0].divisor);
    let (prev_num, curr_num) = (results[1].numerator, results[0].numerator);
    // Compute the delta value between both previous and current
    let total_d = ((curr_div + curr_num) - (prev_div + prev_num)) as f64;
    let divisor_d = (curr_div - prev_div) as f64;

    // Return the computed percentage
    ((total_d - divisor_d) / total_d) * 100.0
}

fn main() {
    // Load env variable from .env
    dotenv::dotenv().ok();
    // Init the logger and set the debug level correctly
    sproot::configure_logger();
    // Init the connection to the postgresql
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    // Get the max number of connection to open
    // No fear to parse it to u32 and unwrap, if not a correct value crash is ok
    let max_db_connection = match std::env::var("DATABASE_MAX_CONNECTION") {
        Ok(value) => value,
        Err(_) => "10".into(),
    }
    .parse::<u32>()
    .unwrap();
    // Create a pool of connection
    // This step might spam for error max_db_connection of times, this is normal.
    let pool = r2d2::Pool::builder()
        .max_size(max_db_connection)
        .build(manager)
        .expect("Failed to create pool");
    // Apply the migrations to the database
    // It's safe to unwrap as if there is an error at this step, we don't continue running the app
    embedded_migrations::run(&pool.get().expect("Cannot get a connection from the pool.")).unwrap();

    // TEST
    let alert = Alerts {
        name: "cpu_usage".into(),
        table: "cputimes".into(),
        lookup: "average 10m of cuser,nice,system,irq,softirq,steal over idle,iowait".into(),
        timing: 60,
        warn: "$this > 50".into(),
        crit: "$this > 80".into(),
        info: "average cpu utilization over the last 10 minutes".into(),
        host_uuid: "07c2ff2b28f01e93ef1ea6e311664a97facefba7".into(),
        where_clause: None,
    };

    let mut lookup_parts = alert.lookup.split(' ');

    let req_type = lookup_parts.next();
    let req_time = lookup_parts.next();
    let req_numerator = lookup_parts.nth(1);
    let req_divisor = lookup_parts.nth(1);
    dbg!(&req_type);
    dbg!(&req_time);
    dbg!(&req_numerator);
    dbg!(&req_divisor);

    let pg_agregate = match req_type {
        Some("average") => "avg",
        Some(&_) => {
            panic!("Unhandled aggregation function");
        }
        None => {
            panic!("Can't determine the aggreation function, no req_type");
        }
    };
    dbg!(&pg_agregate);

    let pg_time = match req_time {
        Some(v) => v,
        None => {
            panic!("Can't determine the time range, no req_time");
        }
    };
    dbg!(&pg_time);

    let mut pg_select = String::new();
    let select_cols = req_numerator.unwrap().split(',');
    for col in select_cols {
        pg_select.push_str(&format!("{}({})::int8 + ", pg_agregate, col));
    }
    pg_select.drain(pg_select.len() - 3..pg_select.len());
    pg_select.push_str(" as numerator, ");

    let select_cols = req_divisor.unwrap().split(',');
    for col in select_cols {
        pg_select.push_str(&format!("{}({})::int8 + ", pg_agregate, col));
    }
    pg_select.drain(pg_select.len() - 3..pg_select.len());
    pg_select.push_str(" as divisor");
    dbg!(&pg_select);

    let mut pg_where = String::new();
    if alert.where_clause.is_some() {
        pg_where.push_str(" AND ");
        pg_where.push_str(&alert.where_clause.unwrap());
    };

    let query = format!("SELECT time_bucket('{0}', created_at) as time, {1} FROM {2} WHERE host_uuid=$1 AND created_at > now() at time zone 'utc' - INTERVAL '{0}' {3} GROUP BY time ORDER BY time DESC", pg_time, pg_select, alert.table, pg_where);
    dbg!(&query);

    let statements = vec![
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
    for statement in statements {
        assert!(!query.contains(statement));
    }

    let results = sql_query(query)
        .bind::<Text, _>(alert.host_uuid)
        .load::<DTORaw>(&pool.get().unwrap());
    dbg!(&results);

    let percentage = compute_percentage(&results.unwrap());
    dbg!(&percentage);

    let shound_warn = eval_boolean(&alert.warn.replace("$this", &percentage.to_string()));
    dbg!(&shound_warn);

    let shound_crit = eval_boolean(&alert.crit.replace("$this", &percentage.to_string()));
    dbg!(&shound_crit);

    //  - Start an async task which will loop forever
    //      - In that loop we'll get all alerts
    //          - For all those alerts we'll perform the check needed
    //          - If an incident is already ongoing for that alert
    //              - We're analyzing the result and determine if the incident is finished
    //                  - If the incident is finished we update it with Diesel setting end date
    //                  - We'll also in (in the future) send an email containing all the information during that range time (cpu, memory, disks, net, ...)
    //          - Else analyze the result and determine if we need to trigger an incident
    //              - If new incident we create it with Diesel and email/... to recipients
    //          - After all the works we just "sleep" for the next interval (which can be configured ?)[a lower interval will be more resource intensive but will give better reaction]
    //  - Start the actix server with different routes
    //      - A route to create new alerts
    //      - A route to update existing alerts (modify, pause, ...)
    //      - A route to get all alerts
    //      - A route to delete alerts
    println!("zpour");
}
