use std::env;
use error_stack::{Report, Result};
use sqlx::{postgres::PgPoolOptions, Postgres, Pool, Error};

pub async fn get_connection_pool() -> Result<Pool<Postgres>, Error> {
    let database_url = env::var("DATABASE_URL").expect("No database url found, env.DATABASE_URL not found.");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url).await.map_err(|error| {
            match error {
                Error::PoolClosed => Report::new(Error::PoolClosed).attach_printable("Sqlx PoolClosed: {error}"),
                _ => Report::new(Error::PoolTimedOut).attach_printable("Sqlx PoolTimedOut: {error}")
            }
            
        })
}