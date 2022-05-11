use std::error::Error;

use super::{
    super::domain::{
        entities::{Anime, Episode},
        repositories::AnimeRepositoryContract,
    },
    datasources::GoGoPlayInterface,
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

    async fn get_anime_episodes(&self, anime: &Anime) -> Result<Vec<Episode>, Box<dyn Error>> {
        todo!()
    }

    async fn get_streaming_link(&self, ep: &Episode) -> Result<String, Box<dyn Error>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use mockall::predicate::eq;

    use crate::features::watch_anime::{
        data::models::AnimeModel, domain::repositories::AnimeRepositoryContract,
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
        repo.search_anime("some search").await.unwrap();
    }
}
