#![warn(clippy::all, clippy::nursery, clippy::pedantic)]
#![allow(clippy::used_underscore_binding)]
use dotenv::dotenv;
use std::env;

mod scrape;
mod tmdb;

use scrape::{bfi::Bfi, Scrape};
use tmdb::Tmdb;

#[async_std::main]
async fn main() -> Result<(), surf::Error> {
    dotenv()?;

    // let tmdb = Tmdb::new(env::var("TMDB_API_KEY")?);
    // dbg!(tmdb.top_movies().await?);
    // let bfi =
    //     Scrape::new("https://www.bfi.org.uk/films-tv-people/sightandsoundpoll2012/directors/");
    dbg!(Bfi.movie_queries().await?);
    Ok(())
}
