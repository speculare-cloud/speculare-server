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
    pub timing: i64,       // Number of seconds between checks
    pub warn: String, // $this > 50 ($this refer to the result of the query, should return a bool)
    pub crit: String, // $this > 80 ($this refer to the result of the query, should return a bool)
    pub info: String, // Description of the alarms
    pub host_uuid: String, // Targeted host
}

#[derive(QueryableByName, Debug)]
pub struct DTORaw {
    #[sql_type = "Int8"]
    pub numerator: i64,
    #[sql_type = "Int8"]
    pub divisor: i64,
    #[sql_type = "Timestamp"]
    pub created_at: chrono::NaiveDateTime,
}

// // Embed migrations into the binary
embed_migrations!();

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
    };

    let mut parts = alert.lookup.split(' ');
    let req_type = parts.next();
    let req_time = parts.next();
    let req_numerator = parts.nth(1);
    let req_divisor = parts.nth(1);

    dbg!(&req_type);
    dbg!(&req_time);
    dbg!(&req_numerator);
    dbg!(&req_divisor);

    let pg_agregate = match req_type {
        Some("average") => "avg",
        Some(&_) => {
            dbg!("Unknown agregation function");
            "max"
        }
        None => "avg",
    };
    dbg!(&pg_agregate);

    let mut pg_select = String::new();
    let mut pg_fields = String::new();
    let select_cols = req_numerator.unwrap().split(',');
    for col in select_cols {
        pg_fields.push_str(&format!("{}, ", col));
        pg_select.push_str(&format!("{}({})::int8 + ", pg_agregate, col));
    }
    pg_select.drain(pg_select.len() - 3..pg_select.len());
    pg_select.push_str(" as numerator, ");

    let select_cols = req_divisor.unwrap().split(',');
    for col in select_cols {
        pg_fields.push_str(&format!("{}, ", col));
        pg_select.push_str(&format!("{}({})::int8 + ", pg_agregate, col));
    }
    pg_select.drain(pg_select.len() - 3..pg_select.len());
    pg_select.push_str(" as divisor");
    pg_fields.push_str("created_at as time");
    dbg!(&pg_fields);
    dbg!(&pg_select);

    let limit;
    let pg_interval = match req_time {
        Some(val) => {
            assert!(val.len() >= 3);
            let nb = val[..2].parse::<i64>().unwrap();
            let scale = &val[2..];
            dbg!(&nb);
            dbg!(&scale);
            match scale {
                "h" => {
                    limit = nb * 60 * 60;
                    format!(
                        "(extract(hour from time)::int/{})* '{}h'::interval as created_at",
                        nb, nb
                    )
                }
                "m" => {
                    limit = nb * 60;
                    format!("(extract(hour from time)::int)* '1h'::interval + (extract(minute from time)::int/{})* '{}m'::interval as created_at", nb, nb)
                }
                "s" => {
                    limit = nb;
                    format!("(extract(hour from time)::int)* '1h'::interval + (extract(minute from time)::int)* '1m'::interval + (extract(second from time)::int/{})* '{}s'::interval as created_at", nb, nb)
                }
                _ => {
                    limit = 1;
                    "".into()
                }
            }
        }
        None => {
            limit = 1;
            "(extract(hour from time)::int)* '1h'::interval + (extract(minute from time)::int/10)* '10m'::interval as created_at".into()
        }
    };
    dbg!(&limit);
    dbg!(&pg_interval);

    let query = format!("WITH s AS (SELECT {} FROM cputimes WHERE host_uuid=$1 ORDER BY created_at DESC LIMIT $2) SELECT {}, time::date + {} FROM s GROUP BY created_at ORDER BY created_at DESC LIMIT 2", pg_fields, pg_select, pg_interval);
    dbg!(&query);

    let result = sql_query(query)
        .bind::<Text, _>(alert.host_uuid)
        .bind::<Int8, _>(limit * 2)
        .load::<DTORaw>(&pool.get().unwrap());
    dbg!(&result);

    let result = result.unwrap();
    assert!(result.len() == 2);
    let prev_divisor = result[1].divisor as f64;
    let prev_numerator = result[1].numerator as f64;
    let curr_divisor = result[0].divisor as f64;
    let curr_numerator = result[0].numerator as f64;

    let prev_total = prev_divisor + prev_numerator;
    let curr_total = curr_divisor + curr_numerator;

    let total_d = curr_total - prev_total;
    let divisor_d = curr_divisor - prev_divisor;

    let percentage = ((total_d - divisor_d) / total_d) * 100.0;
    dbg!(&percentage);

    dbg!(&eval_boolean(
        &alert.warn.replace("$this", &percentage.to_string())
    ));

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
