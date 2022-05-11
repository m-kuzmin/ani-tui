use super::entities::{Anime, Episode};
#[cfg(test)]
use mockall::automock;
use std::error::Error;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait AnimeRepository {
    async fn search_anime(&self, query: &str) -> Result<Vec<Anime>, Box<dyn Error>>;
    async fn get_anime_episodes(&self, anime: &Anime) -> Result<Vec<Episode>, Box<dyn Error>>;
    async fn get_streaming_link(&self, ep: &Episode) -> Result<String, Box<dyn Error>>;
}
