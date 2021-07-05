#[macro_use]
extern crate diesel_migrations;

use diesel::{prelude::PgConnection, r2d2::ConnectionManager};

// Embed migrations into the binary
embed_migrations!();

fn main() {
    // If this is the actix-server handling all alerts related things, here's a pseudo code:
    //
    //  - Do all the basics check, initialize logger and all

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
