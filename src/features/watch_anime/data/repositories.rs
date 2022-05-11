use super::{
    super::domain::{
        entities::{Anime, Episode},
        repositories::AnimeRepositoryContract,
    },
    datasources::GoGoPlayInterface,
    models::EpisodeModel,
};

pub struct AnimeRepository {
    gogo_play: Box<dyn GoGoPlayInterface + Send + Sync>,
}

impl AnimeRepository {
    pub fn new<G>(gogo_play: G) -> Self
    where
        G: GoGoPlayInterface + Sync + Send + 'static,
    {
        Self {
            gogo_play: Box::new(gogo_play),
        }
    }
}

#[async_trait]
impl AnimeRepositoryContract for AnimeRepository {
    async fn search_anime(&self, query: &str) -> Option<Vec<Anime>> {
        Some(
            self.gogo_play
                .search_anime(query)
                .await?
                .into_iter()
                .map(|model| Anime::from(model))
                .collect(),
        )
    }

    async fn get_anime_episodes(&self, anime: &Anime) -> Option<Vec<Episode>> {
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
        data::models::{AnimeModel, EpisodeModel},
        domain::{
            entities::{Anime, Episode},
            repositories::AnimeRepositoryContract,
        },
    };

    use super::{super::datasources::MockGoGoPlayInterface, AnimeRepository};

    #[tokio::test]
    async fn should_search_anime_on_gogoplay() {
        let mut mock_datasource = MockGoGoPlayInterface::new();
        mock_datasource
            .expect_search_anime()
            .times(1)
            .with(eq("some search"))
            .returning(|_| {
                Some(vec![AnimeModel {
                    title: String::from("some anime"),
                    desc: String::from("some desc"),
                }])
            });

        let repo = AnimeRepository::new(mock_datasource);
        let result = repo.search_anime("some search").await.unwrap();

        assert_eq!(result, vec![Anime::new("some anime", "some desc")]);
    }

    #[tokio::test]
    async fn should_get_anime_episodes_on_gogoplay() {
        let mut mock_datasource = MockGoGoPlayInterface::new();
        mock_datasource
            .expect_get_anime_episode_list()
            .times(1)
            .with(eq(AnimeModel {
                title: String::from("some title"),
                desc: String::from("some desc"),
            }))
            .returning(|_| {
                Some(vec![EpisodeModel {
                    title: String::from("some episode"),
                }])
            });

        let repo = AnimeRepository::new(mock_datasource);
        let result = repo
            .get_anime_episodes(&Anime {
                title: String::from("some title"),
                desc: String::from("some desc"),
            })
            .await
            .unwrap();

        assert_eq!(
            result,
            vec![Episode {
                title: String::from("some episode"),
            }]
        );
    }

    #[tokio::test]
    async fn should_get_streaming_link_on_gogoplay() {
        let mut mock_datasource = MockGoGoPlayInterface::new();

        mock_datasource
            .expect_get_streaming_link()
            .times(1)
            .with(eq(EpisodeModel {
                title: String::from("some title"),
            }))
            .returning(|_| Some(String::from("some link")));

        let repo = AnimeRepository::new(mock_datasource);
        let result = repo
            .get_streaming_link(&Episode::new("some title"))
            .await
            .unwrap();

        assert_eq!(&result, "some link");
    }
}
