use sqlx::{postgres::PgPoolOptions, PgPool, Pool};

use std::env;

pub async fn create() -> PgPoolOptions {
    let client_uri =
        env::var("POSTGRES_DB_URI").expect("You must set the POSTGRES_DB_URI environment var!");
    PgPoolOptions::new().connect(client_uri).await?
}
