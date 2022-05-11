use super::entities::{Anime, Episode};
use std::error::Error;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait AnimeRepositoryContract {
    async fn search_anime(&self, query: &str) -> Option<Vec<Anime>>;
    async fn get_anime_episodes(&self, anime: &Anime) -> Result<Vec<Episode>, Box<dyn Error>>;
    async fn get_streaming_link(&self, ep: &Episode) -> Result<String, Box<dyn Error>>;
}
