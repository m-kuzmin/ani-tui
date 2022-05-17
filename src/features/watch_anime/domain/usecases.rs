use std::sync::Arc;

use crate::core::Usecase;

use super::{
    entities::{AnimeDetails, AnimeSearchItem, Episode},
    repositories::AnimeRepositoryContract,
};

/// Searches for anime and returns matching titles
pub struct SearchAnime {
    /// An anime repository
    repo: Arc<dyn AnimeRepositoryContract + Send + Sync>,
}

impl SearchAnime {
    /// Creates a new usecase object
    pub fn new(repo: Arc<dyn AnimeRepositoryContract + Send + Sync>) -> Self {
        Self { repo: repo.clone() }
    }
}

/// Returns a list of matching titles or None in case of error
#[cfg_attr(test, automock)]
#[async_trait]
impl Usecase for SearchAnime {
    /// Anime title
    type Params = String;
    /// Search results
    type Return = Option<Vec<AnimeSearchItem>>;

    /// Performs a search
    async fn call(&self, s: &Self::Params) -> Self::Return {
        self.repo.search_anime(s).await
    }
}

/// Provides a list of episodes for anime
pub struct GetEpisodesOfAnime {
    /// An anime repository
    repo: Arc<dyn AnimeRepositoryContract + Send + Sync>,
}

impl GetEpisodesOfAnime {
    /// Creates a new usecase object
    pub fn new(repo: Arc<dyn AnimeRepositoryContract + Send + Sync>) -> Self {
        Self { repo }
    }
}

/// Returns a list of anime episodes from anime search result item
#[cfg_attr(test, automock)]
#[async_trait]
impl Usecase for GetEpisodesOfAnime {
    /// An anime search result
    type Params = AnimeSearchItem;
    /// A list of episodes or [`None`] in case of error
    type Return = Option<Vec<Episode>>;

    /// Provides a list of episodes
    async fn call(&self, anime: &Self::Params) -> Self::Return {
        self.repo.get_anime_episodes(anime).await
    }
}

/// Provides details for an anime
pub struct GetAnimeDetails {
    /// An anime repository
    repo: Arc<dyn AnimeRepositoryContract + Send + Sync>,
}

#[async_trait]
impl Usecase for GetAnimeDetails {
    type Params = AnimeSearchItem;
    type Return = Option<AnimeDetails>;

    async fn call(&self, anime: &Self::Params) -> Self::Return {
        self.repo.get_anime_details(anime).await
    }
}

impl GetAnimeDetails {
    /// Creates a new instance
    pub fn new(repo: Arc<dyn AnimeRepositoryContract + Send + Sync>) -> Self {
        Self { repo }
    }
}

/// Provides a streaming link for anime episode
pub struct GetStreamingLink {
    /// An anime repository
    repo: Arc<dyn AnimeRepositoryContract + Send + Sync>,
}

impl GetStreamingLink {
    /// Creates a new usecase object
    pub fn new(repo: Arc<dyn AnimeRepositoryContract + Send + Sync>) -> Self {
        Self { repo }
    }
}

/// Provides a streaming link as string or [`None`] in cace of error.
#[async_trait]
impl Usecase for GetStreamingLink {
    /// An episode of an anime
    type Params = Episode;
    /// A URL string
    type Return = Option<String>;

    /// Provides a streaming link
    async fn call(&self, ep: &Self::Params) -> Self::Return {
        self.repo.get_streaming_link(ep).await
    }
}

#[cfg(test)]
mod tests {
    use crate::features::watch_anime::domain::entities::AnimeDetails;

    use super::{super::repositories::MockAnimeRepositoryContract as AnimeRepository, *};
    use mockall::predicate::*;

    #[tokio::test]
    async fn should_search_for_anime_in_repository() {
        let mut mock_repo = AnimeRepository::new();
        mock_repo
            .expect_search_anime()
            .times(1)
            .with(eq("some anime"))
            .returning(|_| {
                Some(vec![AnimeSearchItem::new(
                    "some matching title",
                    "hidden ident originating in model",
                )])
            });

        let usecase = SearchAnime::new(Arc::new(mock_repo));

        let result = usecase.call(&"some anime".to_string()).await.unwrap();
        assert_eq!(
            &vec![AnimeSearchItem::new(
                "some matching title",
                "hidden ident originating in model"
            )],
            &result
        );
    }

    #[tokio::test]
    async fn should_get_list_of_eps_of_anime() {
        let mut mock_repo = AnimeRepository::new();

        mock_repo
            .expect_get_anime_episodes()
            .times(1)
            .with(eq(AnimeSearchItem::new("some title", "some-ident")))
            .returning(|_| Some(vec![Episode::new("some ep", "some-ident", 1)]));

        let usecase = GetEpisodesOfAnime::new(Arc::new(mock_repo));

        let result = usecase
            .call(&AnimeSearchItem::new("some title", "some-ident"))
            .await
            .unwrap();
        assert_eq!(vec![Episode::new("some ep", "some-ident", 1)], result);
    }

    #[tokio::test]
    async fn should_get_streaming_link_for_ep() {
        let mut mock_repo = AnimeRepository::new();

        mock_repo
            .expect_get_streaming_link()
            .times(1)
            .with(eq(Episode::new("some title", "some-ident", 1)))
            .returning(|_| Some(String::from("some link")));

        let usecase = GetStreamingLink::new(Arc::new(mock_repo));

        let result = usecase
            .call(&Episode::new("some title", "some-ident", 1))
            .await
            .unwrap();

        assert_eq!(&"some link", &result);
    }

    #[tokio::test]
    async fn should_get_anime_details() {
        let mut mock_repo = AnimeRepository::new();

        mock_repo
            .expect_get_anime_details()
            .times(1)
            .with(eq(AnimeSearchItem::new("", "some-ident")))
            .returning(|_| {
                Some(AnimeDetails::new(
                    "some title",
                    "some description",
                    vec![Episode::new("some title", "some-ident", 1)],
                    "some-ident",
                ))
            });

        let usecase = GetAnimeDetails::new(Arc::new(mock_repo));

        let result = usecase
            .call(&AnimeSearchItem::new("", "some-ident"))
            .await
            .unwrap();

        assert_eq!(
            result,
            AnimeDetails::new(
                "some title",
                "some description",
                vec![Episode::new("some title", "some-ident", 1)],
                "some-ident"
            )
        );
    }
}
