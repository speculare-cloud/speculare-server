use crate::{
    embedded_migrations, server,
    utils::{mail::get_tls_parameters, monitoring::launch_monitoring},
    CONFIG,
};

use sproot::Pool;

/// Will start the program normally (launch alerts, ...)
pub async fn flow_run_start(pool: Pool) -> std::io::Result<()> {
    // Check if the SMTP server host is "ok" for TLS
    if CONFIG.smtp_tls {
        match get_tls_parameters() {
            Ok(_) => {}
            Err(e) => {
                error!("MAILER: cannot build tls_parameters: {}", e);
                // TODO - Add a check for the smtp_host at startup
                // to avoid crash only here.
                std::process::exit(1);
            }
        }
    }
    // Get a connection from the R2D2 pool
    let pooled_conn = match pool.get() {
        Ok(pooled) => pooled,
        Err(e) => {
            error!(
                "Cannot get a connection from the pool to apply migrations: {}",
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
    // Launch the monitoring of each alarms
    launch_monitoring(pool.clone())
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.message()))?;
    // Continue the initialization of the actix web server
    server::server(pool).await
}
