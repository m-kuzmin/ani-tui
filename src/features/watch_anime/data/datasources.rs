use std::sync::Arc;

use super::models::{AnimeSearchItemModel, EpisodeModel};
use crate::core::{delivery_mechanisms::WebClient, Model};

/// Implements [`GoGoPlayInterface`]
pub struct GoGoPlayDataSource {
    /// A webclient
    client: Arc<dyn WebClient + Send + Sync>,
}

impl GoGoPlayDataSource {
    /// Creates a new GoGoPlay datasource
    pub fn new(client: Arc<dyn WebClient + Send + Sync>) -> Self {
        Self { client }
    }
}

/// <https://goload.pro>-specific interface
#[cfg_attr(test, automock)]
#[async_trait]
pub trait GoGoPlayInterface {
    /// Searches for anime
    async fn search_anime(&self, title: &str) -> Option<Vec<AnimeSearchItemModel>>;
    /// Provides episode list for anime
    async fn get_anime_episode_list(
        &self,
        anime: AnimeSearchItemModel,
    ) -> Option<Vec<EpisodeModel>>;
    /// Provides a streaming link for anime episode
    async fn get_streaming_link(&self, ep: &EpisodeModel) -> Option<String>;
}

#[async_trait]
impl GoGoPlayInterface for GoGoPlayDataSource {
    /// Makes a get request to <https://goload.pro/search.html?keyword={TITLE}> and returns a parsed list of anime.
    async fn search_anime(&self, title: &str) -> Option<Vec<AnimeSearchItemModel>> {
        let html = self
            .client
            .get(
                "https://goload.pro/search.html",
                Some(vec![("keyword".to_string(), title.to_string())]),
            )
            .await?;
        Vec::<AnimeSearchItemModel>::from_html(&html)
    }

    /// Makes a get request to episode 1 of an anime and returns all episodes on page (<https://goload.pro/videos/{ANIME_IDENTIFIER}-episode-1>)
    async fn get_anime_episode_list(
        &self,
        anime: AnimeSearchItemModel,
    ) -> Option<Vec<EpisodeModel>> {
        let html = self
            .client
            .get(
                &format!("https://goload.pro/videos/{}-episode-1", anime.ident),
                None,
            )
            .await?;
        if &html == "404\n" {
            return None;
        }

        Some(Vec::<EpisodeModel>::from_html(&html)?)
    }

    async fn get_streaming_link(&self, _ep: &EpisodeModel) -> Option<String> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use crate::core::delivery_mechanisms::MockWebClient as WebClient;
    use mockall::predicate::*;
    use std::{fs::File, io::Read};

    use super::*;
    fn fixture(file: &str) -> String {
        let mut content = String::new();
        File::open(format!("tests/fixtures/{}", file))
            .unwrap()
            .read_to_string(&mut content)
            .unwrap();
        content
    }

    #[tokio::test]
    async fn should_give_anime_list_from_search_query_on_gogoplay() {
        let mut mock_client = WebClient::new();
        mock_client
            .expect_get()
            .times(1)
            .with(
                eq("https://goload.pro/search.html"),
                eq(Some(vec![(
                    "keyword".to_string(),
                    "some anime".to_string(),
                )])),
            )
            .returning(|_, _| Some(fixture("search.html")));

        let datasource = GoGoPlayDataSource::new(Arc::new(mock_client));
        let result = datasource.search_anime("some anime").await.unwrap();

        assert_eq!(
            result,
            vec![
                AnimeSearchItemModel::new("Some Anime", "some-anime"),
                AnimeSearchItemModel::new("Some Other Anime", "some-unmatching-link"),
                AnimeSearchItemModel::new(
                    "This dark Episode: Doesnt end with ep number",
                    "break-follow-ep"
                ),
            ]
        );
    }

    #[tokio::test]
    async fn should_give_list_of_eps_for_anime_from_gogoplay() {
        let mut mock_client = WebClient::new();

        mock_client
            .expect_get()
            .times(1)
            .with(
                eq("https://goload.pro/videos/some-ident-episode-1"),
                eq(None),
            )
            .returning(|_, _| Some(fixture("some-anime-episode-1.html")));

        let datasource = GoGoPlayDataSource::new(Arc::new(mock_client));

        let result = datasource
            .get_anime_episode_list(AnimeSearchItemModel::new("", "some-ident"))
            .await
            .unwrap();

        assert_eq!(
            result,
            vec![
                EpisodeModel::new("Episode 2 title", "some-ident", 2),
                EpisodeModel::new("Episode 1 title", "some-ident", 1)
            ]
        );
    }

    #[tokio::test]
    async fn should_detect_404_when_getting_ep_list() {
        let mut mock_client = WebClient::new();

        mock_client
            .expect_get()
            .times(1)
            .with(
                eq("https://goload.pro/videos/some-ident-episode-1"),
                eq(None),
            )
            .returning(|_, _| Some(String::from("404\n")));

        let datasource = GoGoPlayDataSource::new(Arc::new(mock_client));

        let result = datasource
            .get_anime_episode_list(AnimeSearchItemModel::new("", "some-ident"))
            .await;

        assert_eq!(result, None);
    }
}
