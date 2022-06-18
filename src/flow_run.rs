use crate::{server, CONFIG, MIGRATIONS};

use diesel::{prelude::PgConnection, r2d2::ConnectionManager};
use diesel_migrations::MigrationHarness;
use sproot::Pool;

fn build_pool(db_url: &str, max_conn: u32) -> Pool {
    // Init the connection to the postgresql
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    // This step might spam for error CONFIG.database_max_connection of times, this is normal.
    match r2d2::Pool::builder()
        .max_size(max_conn)
        .min_idle(Some((10 * max_conn) / 100))
        .build(manager)
    {
        Ok(pool) => {
            info!("R2D2 PostgreSQL pool created");
            pool
        }
        Err(e) => {
            error!("Failed to create db pool: {}", e);
            std::process::exit(1);
        }
    }
}

fn apply_migration(pool: &Pool) {
    // Get a connection from the R2D2 pool
    let mut pooled_conn = match pool.get() {
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
    if let Err(e) = pooled_conn.run_pending_migrations(MIGRATIONS) {
        error!("Cannot apply the migrations: {}", e);
        std::process::exit(1);
    }
}

/// Will start the program normally
pub async fn flow_run_start() -> std::io::Result<()> {
    // Init the connection to the postgresql
    let metrics_db = build_pool(&CONFIG.database_url, CONFIG.database_max_connection);
    #[cfg(feature = "auth")]
    let auth_db = build_pool(
        &CONFIG.auth_database_url,
        CONFIG.auth_database_max_connection,
    );

    // Apply the migrations to the database
    apply_migration(&metrics_db);

    #[cfg(feature = "auth")]
    let auth_pool = Some(auth_db);
    #[cfg(not(feature = "auth"))]
    let auth_pool = None;

    // Continue the initialization of the actix web server
    server::server(metrics_db, auth_pool).await
}
