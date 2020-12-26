use diesel::{prelude::PgConnection, r2d2::ConnectionManager};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub type ConnType = r2d2::PooledConnection<ConnectionManager<PgConnection>>;
