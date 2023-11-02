#[macro_use]
extern crate rocket;
use anyhow::Result;
use dotenv::dotenv;
use rocket::response::status::NoContent;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::env;

#[get("/health_check")]
fn health_check() -> NoContent {
    NoContent
}

#[rocket::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    rocket::build()
        .mount("/", routes![health_check])
        .manage(pool)
        .launch()
        .await?;

    Ok(())
}
