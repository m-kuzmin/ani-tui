use super::models::{AnimeModel, EpisodeModel};
pub struct GoGoPlayDataSource {}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait GoGoPlayInterface {
    async fn get_anime_episode(&self, anime: AnimeModel, ep: usize) -> Option<EpisodeModel>;
    async fn search_anime(&self, title: &str) -> Option<Vec<AnimeModel>>;
}

#[async_trait]
impl GoGoPlayInterface for GoGoPlayDataSource {
    async fn get_anime_episode(&self, anime: AnimeModel, ep: usize) -> Option<EpisodeModel> {
        todo!()
    }

    async fn search_anime(&self, title: &str) -> Option<Vec<AnimeModel>> {
        todo!()
    }
}
