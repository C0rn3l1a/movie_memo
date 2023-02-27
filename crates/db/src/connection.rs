use std::env;

use sqlx::{postgres::PgPoolOptions, Postgres, Pool, Error};

pub async fn get_connection_pool() -> Result<Pool<Postgres>, Error> {
    let database_url = env::var("DATABASE_URL").expect("No database url found, env.DATABASE_URL not found.");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url).await
}