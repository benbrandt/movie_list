use crate::scrape::Scrape;
use async_trait::async_trait;
use scraper::{Html, Selector};

pub(crate) struct Bfi;

#[async_trait]
impl Scrape for Bfi {
    async fn get_page(&self, _: Option<i32>) -> Result<String, surf::Error> {
        surf::get("https://www.bfi.org.uk/films-tv-people/sightandsoundpoll2012/directors/")
            .recv_string()
            .await
    }

    async fn movie_queries(&self) -> Result<Vec<String>, surf::Error> {
        let document = Html::parse_document(&self.get_page(None).await?);
        let item_selector = Selector::parse(".sas-film-list-row").unwrap();
        let title_selector = Selector::parse(".show-for-small a").unwrap();

        let mut titles = vec![];
        for item in document.select(&item_selector) {
            for title in item.select(&title_selector) {
                titles.push(item.text().collect::<Vec<_>>().join(""));
            }
        }
        Ok(titles)
    }
}
