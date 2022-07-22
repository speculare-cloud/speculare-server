use diesel::{r2d2::ConnectionManager, PgConnection};
use diesel_migrations::MigrationHarness;
use sproot::Pool;

use crate::MIGRATIONS;

pub fn build_pool(db_url: &str, max_conn: u32) -> Pool {
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
        Err(err) => {
            error!("Failed to create db pool: {}", err);
            std::process::exit(1);
        }
    }
}

pub fn apply_migration(pool: &Pool) {
    // Get a connection from the R2D2 pool
    let mut pooled_conn = match pool.get() {
        Ok(pooled) => pooled,
        Err(err) => {
            error!(
                "Cannot get a connection from the pool to apply migrations: {:?}",
                err
            );
            std::process::exit(1);
        }
    };

    // Apply the migrations to the database
    if let Err(err) = pooled_conn.run_pending_migrations(MIGRATIONS) {
        error!("Cannot apply the migrations: {}", err);
        std::process::exit(1);
    }
}
