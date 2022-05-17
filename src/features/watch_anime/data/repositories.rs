use std::{collections::HashSet, sync::Arc};

use crate::features::watch_anime::domain::entities::{AnimeDetails, AnimeSearchItem};

use super::{
    super::domain::{entities::Episode, repositories::AnimeRepositoryContract},
    datasources::GoGoPlayInterface,
    models::EpisodeModel,
};

/// [`AnimeRepositoryContract`] implementor
pub struct AnimeRepository {
    /// A GoGoPlay data source
    gogo_play: Arc<dyn GoGoPlayInterface + Send + Sync>,
}

impl AnimeRepository {
    /// Creates a new anime repository
    pub fn new(gogo_play: Arc<dyn GoGoPlayInterface + Send + Sync>) -> Self {
        Self { gogo_play }
    }
}

#[async_trait]
impl AnimeRepositoryContract for AnimeRepository {
    /// Searches for anime in GoGoPlay datasource
    async fn search_anime(&self, query: &str) -> Option<Vec<AnimeSearchItem>> {
        let mut res = self.gogo_play.search_anime(query).await?;
        let mut set = HashSet::new();
        res.retain(|x| set.insert(x.ident.clone()));

        Some(res.into_iter().map(AnimeSearchItem::from).collect())
    }
    /// Provides a list of episodes for anime from GoGoPlay datasource
    async fn get_anime_episodes(&self, anime: &AnimeSearchItem) -> Option<Vec<Episode>> {
        Some(
            self.gogo_play
                .get_anime_episode_list(anime.into())
                .await?
                .into_iter()
                .map(Episode::from)
                .collect(),
        )
    }

    /// Provides a streaming link for an episode from GoGoPlay datasource
    async fn get_streaming_link(&self, ep: &Episode) -> Option<String> {
        self.gogo_play
            .get_streaming_link(&EpisodeModel::from(ep))
            .await
    }

    async fn get_anime_details(&self, anime: &AnimeSearchItem) -> Option<AnimeDetails> {
        self.gogo_play
            .get_anime_details(&anime.into())
            .await
            .map(AnimeDetails::from)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use mockall::predicate::eq;

    use crate::features::watch_anime::{
        data::models::{AnimeDetailsModel, AnimeSearchItemModel, EpisodeModel},
        domain::{
            entities::{AnimeDetails, AnimeSearchItem, Episode},
            repositories::AnimeRepositoryContract,
        },
    };

    use super::{super::datasources::MockGoGoPlayInterface as GoGoPlayDataSource, AnimeRepository};

    #[tokio::test]
    async fn should_give_anime_search_resultrs_from_gogoplay() {
        let mut mock_datasource = GoGoPlayDataSource::new();

        mock_datasource
            .expect_search_anime()
            .times(1)
            .with(eq("some search"))
            .returning(|_| Some(vec![AnimeSearchItemModel::new("some anime", "some-ident")]));

        let repo = AnimeRepository::new(Arc::new(mock_datasource));
        let result = repo.search_anime("some search").await.unwrap();

        assert_eq!(
            result,
            vec![AnimeSearchItem::new("some anime", "some-ident")]
        );
    }

    #[tokio::test]
    async fn should_dedup_search_results() {
        let mut mock_datasource = GoGoPlayDataSource::new();

        mock_datasource
            .expect_search_anime()
            .times(1)
            .with(eq("some search"))
            .returning(|_| {
                let duplicate = AnimeSearchItemModel::new("some anime title", "some-ident");
                Some(vec![duplicate.clone(), duplicate.clone()])
            });

        let repo = AnimeRepository::new(Arc::new(mock_datasource));
        let result = repo.search_anime("some search").await.unwrap();

        assert_eq!(
            result,
            vec![AnimeSearchItem::new("some anime title", "some-ident")]
        )
    }

    #[tokio::test]
    async fn should_give_anime_episode_list_from_gogoplay() {
        let mut mock_datasource = GoGoPlayDataSource::new();

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

        let repo = AnimeRepository::new(Arc::new(mock_datasource));
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
        let mut mock_datasource = GoGoPlayDataSource::new();

        mock_datasource
            .expect_get_streaming_link()
            .times(1)
            .with(eq(EpisodeModel::new("some title", "some-ident", 1)))
            .returning(|_| Some(String::from("some/link")));

        let repo = AnimeRepository::new(Arc::new(mock_datasource));
        let result = repo
            .get_streaming_link(&Episode::new("some title", "some-ident", 1))
            .await
            .unwrap();

        assert_eq!(&result, "some/link");
    }

    #[tokio::test]
    async fn should_give_anime_details_from_gogoplay() {
        let mut mock_datasource = GoGoPlayDataSource::new();

        mock_datasource
            .expect_get_anime_details()
            .times(1)
            .with(eq(AnimeSearchItemModel::new("some title", "some-ident")))
            .returning(|_| {
                Some(AnimeDetailsModel::new(
                    "some title",
                    "some description",
                    vec![EpisodeModel::new("some title", "some ident", 1)],
                    "some-ident",
                ))
            });

        let repo = AnimeRepository::new(Arc::new(mock_datasource));
        let result = repo
            .get_anime_details(&AnimeSearchItem::new("some title", "some-ident"))
            .await
            .unwrap();

        assert_eq!(
            result,
            AnimeDetails::new(
                "some title",
                "some description",
                vec![Episode::new("some title", "some ident", 1)],
                "some-ident"
            )
        );
    }
}
