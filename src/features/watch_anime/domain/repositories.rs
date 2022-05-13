use super::entities::{AnimeSearchItem, Episode};

#[cfg_attr(test, automock)]
#[async_trait]
pub trait AnimeRepositoryContract {
    async fn search_anime(&self, query: &str) -> Option<Vec<AnimeSearchItem>>;
    async fn get_anime_episodes(&self, anime: &AnimeSearchItem) -> Option<Vec<Episode>>;
    async fn get_streaming_link(&self, ep: &Episode) -> Option<String>;
}
