use crate::core::Model;

use super::models::{AnimeSearchItemModel, EpisodeModel, SearchResultModel};
pub struct GoGoPlayDataSource {
    client: Box<dyn WebClient + Send + Sync>,
}

impl GoGoPlayDataSource {
    pub fn new<W>(client: W) -> Self
    where
        W: WebClient + Send + Sync + 'static,
    {
        Self {
            client: Box::new(client),
        }
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
        todo!()
    }

    async fn get_streaming_link(&self, ep: &EpisodeModel) -> Option<String> {
        todo!()
    }
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait WebClient {
    async fn get(&self, url: &str, query_param: Option<Vec<(String, String)>>) -> Option<String>;
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read};

    use mockall::predicate::*;

    use super::*;
    fn fixture(file: &str) -> String {
        let mut content = String::new();
        File::open(format!("tests/fixtures/{}", file))
            .unwrap()
            .read_to_string(&mut content);
        content
    }

    #[tokio::test]
    async fn should_search_for_anime_on_gogoplay() {
        let mut mock_client = MockWebClient::new();
        mock_client
            .expect_get()
            .times(1)
            // https://goload.pro/search.html?keyword=kemo
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
}
