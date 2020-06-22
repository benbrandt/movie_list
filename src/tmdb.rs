use http_client::native::NativeClient as Client;
use serde::{Deserialize, Serialize};

const API_URL: &str = "https://api.themoviedb.org/3/";

#[derive(Serialize, Deserialize)]
struct Paged {
    api_key: String,
    page: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Movie {
    id: i32,
    title: String,
    original_title: String,
    overview: String,
    tagline: String,
    runtime: i32,
    release_date: String,
    original_language: String,
    poster_path: Option<String>,
    backdrop_path: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct Search {
    api_key: String,
    query: String,
    year: Option<i32>,
}

pub(crate) struct SearchInfo {
    query: String,
    year: Option<i32>,
}

#[derive(Serialize, Deserialize)]
struct SearchResult {
    id: i32,
}

#[derive(Serialize, Deserialize)]
struct SearchResults {
    results: Vec<SearchResult>,
    total_results: i32,
}

pub struct Tmdb {
    api_key: String,
}

impl Tmdb {
    pub(crate) const fn new(api_key: String) -> Self {
        Self { api_key }
    }

    fn get(&self, path: &str) -> Result<surf::Request<Client>, surf::Error> {
        #[derive(Serialize, Deserialize)]
        struct ApiKey {
            api_key: String,
        }

        Ok(
            surf::get(format!("{}{}", API_URL, path)).set_query(&ApiKey {
                api_key: self.api_key.clone(),
            })?,
        )
    }

    pub(crate) async fn get_movie(&self, id: i32) -> Result<Movie, surf::Error> {
        Self::get(self, &format!("movie/{}", id))?.recv_json().await
    }

    pub(crate) async fn search_movie(
        &self,
        query: String,
        year: Option<i32>,
    ) -> Result<Option<Movie>, surf::Error> {
        let SearchResults { results, .. } = Self::get(self, "search/movie")?
            .set_query(&Search {
                api_key: self.api_key.clone(),
                query,
                year,
            })?
            .recv_json()
            .await?;
        let movie = match results.as_slice() {
            [item, ..] => Some(Self::get_movie(self, item.id).await?),
            _ => None,
        };
        Ok(movie)
    }

    pub(crate) async fn top_movies(&self) -> Result<Vec<Movie>, surf::Error> {
        let mut movies = vec![];
        // for page in 1..=5 {
        for page in 1..=1 {
            let SearchResults { results, .. } = Self::get(self, "movie/top_rated")?
                .set_query(&Paged {
                    api_key: self.api_key.clone(),
                    page,
                })?
                .recv_json()
                .await?;
            for result in results {
                movies.push(Self::get_movie(self, result.id).await?);
            }
        }
        Ok(movies)
    }
}
