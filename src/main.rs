#![warn(clippy::all, clippy::nursery, clippy::pedantic)]
#![allow(clippy::used_underscore_binding)]
use dotenv::dotenv;
use std::env;

mod tmdb;
use tmdb::Tmdb;

#[async_std::main]
async fn main() -> Result<(), surf::Error> {
    dotenv()?;

    let tmdb = Tmdb::new(env::var("TMDB_API_KEY")?);
    dbg!(tmdb.top_movies().await?);
    Ok(())
}
