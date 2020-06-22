use crate::tmdb::SearchInfo;
use async_trait::async_trait;

pub(crate) mod bfi;
#[async_trait]
pub(crate) trait Scrape {
    // fn new() -> Self;

    async fn get_page(&self, page: Option<i32>) -> Result<String, surf::Error>;

    async fn movie_queries(&self) -> Result<Vec<String>, surf::Error>;
}
