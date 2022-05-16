use crate::features::watch_anime::data::models::AnimeSearchItemModel;

use super::entities::{AnimeDetails, AnimeSearchItem, Episode};

/// Anime repository interface
#[cfg_attr(test, automock)]
#[async_trait]
pub trait AnimeRepositoryContract {
    /// Provides a list of matching titles
    async fn search_anime(&self, query: &str) -> Option<Vec<AnimeSearchItem>>;
    /// Provides a list of episodes for an anime
    async fn get_anime_episodes(&self, anime: &AnimeSearchItem) -> Option<Vec<Episode>>;
    /// Provides additional details about an anime
    async fn get_anime_details(&self, anime: &AnimeSearchItem) -> Option<AnimeDetails>;
    /// Provides a streaming link that can be opened in a player
    async fn get_streaming_link(&self, ep: &Episode) -> Option<String>;
}
