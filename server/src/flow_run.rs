use crate::{embedded_migrations, server, CONFIG};

use diesel::{prelude::PgConnection, r2d2::ConnectionManager};

/// Will start the program normally
pub async fn flow_run_start() -> std::io::Result<()> {
    // Init the connection to the postgresql
    let manager = ConnectionManager::<PgConnection>::new(&CONFIG.database_url);
    // This step might spam for error CONFIG.database_max_connection of times, this is normal.
    let pool = match r2d2::Pool::builder()
        .max_size(CONFIG.database_max_connection)
        .min_idle(Some((10 * CONFIG.database_max_connection) / 100))
        .build(manager)
    {
        Ok(pool) => pool,
        Err(e) => {
            error!("Failed to create db pool: {}", e);
            std::process::exit(1);
        }
    };
    // Get a connection from the R2D2 pool
    let pooled_conn = match pool.get() {
        Ok(pooled) => pooled,
        Err(e) => {
            error!(
                "Cannot get a connection from the pool to apply migrations: {:?}",
                e
            );
            std::process::exit(1);
        }
    };
    // Apply the migrations to the database
    if let Err(e) = embedded_migrations::run(&pooled_conn) {
        error!("Cannot apply the migrations: {}", e);
        std::process::exit(1);
    }
    // Continue the initialization of the actix web server
    server::server(pool).await
}
