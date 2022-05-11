use super::models::{AnimeModel, EpisodeModel};
pub struct GoGoPlayDataSource {}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait GoGoPlayInterface {
    async fn get_anime_episode_list(&self, anime: AnimeModel) -> Option<Vec<EpisodeModel>>;
    async fn search_anime(&self, title: &str) -> Option<Vec<AnimeModel>>;
    async fn get_streaming_link(&self, ep: &EpisodeModel) -> Option<String>;
}

#[async_trait]
impl GoGoPlayInterface for GoGoPlayDataSource {
    async fn get_anime_episode_list(&self, anime: AnimeModel) -> Option<Vec<EpisodeModel>> {
        todo!()
    }

    async fn search_anime(&self, title: &str) -> Option<Vec<AnimeModel>> {
        todo!()
    }

    async fn get_streaming_link(&self, ep: &EpisodeModel) -> Option<String> {
        todo!()
    }
}
