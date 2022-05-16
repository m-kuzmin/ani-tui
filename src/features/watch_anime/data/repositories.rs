use std::sync::Arc;

use crate::features::watch_anime::domain::entities::AnimeSearchItem;

use super::{
    super::domain::{entities::Episode, repositories::AnimeRepositoryContract},
    datasources::GoGoPlayInterface,
    models::EpisodeModel,
};

pub struct AnimeRepository {
    gogo_play: Arc<dyn GoGoPlayInterface + Send + Sync>,
}

impl AnimeRepository {
    pub fn new(gogo_play: Arc<dyn GoGoPlayInterface + Send + Sync>) -> Self {
        Self { gogo_play }
    }
}

#[async_trait]
impl AnimeRepositoryContract for AnimeRepository {
    async fn search_anime(&self, query: &str) -> Option<Vec<AnimeSearchItem>> {
        Some(
            self.gogo_play
                .search_anime(query)
                .await?
                .anime_list
                .into_iter()
                .map(|(title, ident)| AnimeSearchItem::new(&title, &ident))
                .collect(),
        )
    }

    async fn get_anime_episodes(&self, anime: &AnimeSearchItem) -> Option<Vec<Episode>> {
        Some(
            self.gogo_play
                .get_anime_episode_list(anime.into())
                .await?
                .into_iter()
                .map(|model| Episode::from(model))
                .collect(),
        )
    }

    async fn get_streaming_link(&self, ep: &Episode) -> Option<String> {
        self.gogo_play
            .get_streaming_link(&EpisodeModel::from(ep))
            .await
    }
}

#[cfg(test)]
mod tests {
    use mockall::predicate::eq;

    use crate::features::watch_anime::{
        data::models::{AnimeSearchItemModel, EpisodeModel, SearchResultModel},
        domain::{
            entities::{AnimeSearchItem, Episode},
            repositories::AnimeRepositoryContract,
        },
    };

    use super::{super::datasources::MockGoGoPlayInterface, AnimeRepository};

    #[tokio::test]
    async fn should_give_anime_search_resultrs_from_gogoplay() {
        let mut mock_datasource = MockGoGoPlayInterface::new();
        mock_datasource
            .expect_search_anime()
            .times(1)
            .with(eq("some search"))
            .returning(|_| {
                Some(SearchResultModel {
                    anime_list: vec![("some anime".to_string(), "some-ident".to_string())],
                })
            });

        let repo = AnimeRepository::new(mock_datasource);
        let result = repo.search_anime("some search").await.unwrap();

        assert_eq!(
            result,
            vec![AnimeSearchItem::new("some anime", "some-ident")]
        );
    }

    #[tokio::test]
    async fn should_give_anime_episode_list_from_gogoplay() {
        let mut mock_datasource = MockGoGoPlayInterface::new();
        mock_datasource
            .expect_get_anime_episode_list()
            .times(1)
            .with(eq(AnimeSearchItemModel {
                title: String::from("some title"),
                ident: String::from("some-ident"),
            }))
            .returning(|_| {
                Some(vec![
                    EpisodeModel::new("Episode 2 title", "some-ident", 2),
                    EpisodeModel::new("Episode 1 title", "some-ident", 1),
                ])
            });

        let repo = AnimeRepository::new(mock_datasource);
        let result = repo
            .get_anime_episodes(&AnimeSearchItem::new("some title", "some-ident"))
            .await
            .unwrap();

        assert_eq!(
            result,
            vec![
                Episode::new("Episode 2 title", "some-ident", 2),
                Episode::new("Episode 1 title", "some-ident", 1)
            ]
        );
    }

    #[tokio::test]
    async fn should_give_streaming_link_from_gogoplay() {
        let mut mock_datasource = MockGoGoPlayInterface::new();

        mock_datasource
            .expect_get_streaming_link()
            .times(1)
            .with(eq(EpisodeModel::new("some title", "some-ident", 1)))
            .returning(|_| Some(String::from("some/link")));

        let repo = AnimeRepository::new(mock_datasource);
        let result = repo
            .get_streaming_link(&Episode::new("some title", "some-ident", 1))
            .await
            .unwrap();

        assert_eq!(&result, "some/link");
    }
}
