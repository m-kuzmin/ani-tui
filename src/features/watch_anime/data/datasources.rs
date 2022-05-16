use std::sync::Arc;

use super::models::{AnimeSearchItemModel, EpisodeModel, SearchResultModel};
use crate::core::{delivery_mechanisms::WebClient, Model};

pub struct GoGoPlayDataSource {
    client: Arc<dyn WebClient + Send + Sync>,
}

impl GoGoPlayDataSource {
    pub fn new(client: Arc<dyn WebClient + Send + Sync>) -> Self {
        Self { client }
    }
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait GoGoPlayInterface {
    async fn search_anime(&self, title: &str) -> Option<SearchResultModel>;
    async fn get_anime_episode_list(
        &self,
        anime: AnimeSearchItemModel,
    ) -> Option<Vec<EpisodeModel>>;
    async fn get_streaming_link(&self, ep: &EpisodeModel) -> Option<String>;
}

#[async_trait]
impl GoGoPlayInterface for GoGoPlayDataSource {
    async fn search_anime(&self, title: &str) -> Option<SearchResultModel> {
        let html = self
            .client
            .get(
                "https://goload.pro/search.html",
                Some(vec![("keyword".to_string(), title.to_string())]),
            )
            .await?;
        SearchResultModel::from_html(&html)
    }

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
        Some(Vec::<EpisodeModel>::from_html(&html)?)
    }

    async fn get_streaming_link(&self, ep: &EpisodeModel) -> Option<String> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use crate::core::delivery_mechanisms::MockWebClient;
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
        let mut mock_client = MockWebClient::new();
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

        let datasource = GoGoPlayDataSource::new(mock_client);
        let result = datasource.search_anime("some anime").await.unwrap();

        assert_eq!(
            result,
            SearchResultModel {
                anime_list: vec![
                    (String::from("Some Anime"), String::from("some-anime")),
                    (
                        String::from("Some Other Anime"),
                        String::from("some-unmatching-link")
                    ),
                    (
                        String::from("This dark Episode: Doesnt end with ep number"),
                        String::from("break-follow-ep")
                    ),
                ]
            }
        );
    }

    #[tokio::test]
    async fn should_give_list_of_eps_for_anime_from_gogoplay() {
        let mut mock_client = MockWebClient::new();
        mock_client
            .expect_get()
            .times(1)
            .with(
                eq("https://goload.pro/videos/some-ident-episode-1"),
                eq(None),
            )
            .returning(|_, _| Some(fixture("some-anime-episode-1.html")));

        let datasource = GoGoPlayDataSource::new(mock_client);

        let result = datasource
            .get_anime_episode_list(AnimeSearchItemModel {
                title: String::from("some title"),
                ident: String::from("some-ident"),
            })
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
}
