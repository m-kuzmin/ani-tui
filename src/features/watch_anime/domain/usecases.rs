use crate::core::Usecase;

use super::{
    entities::{Anime, Episode},
    repositories::AnimeRepository,
};

pub struct SearchAnime {
    repo: Box<dyn AnimeRepository + Sync + Send>,
}

impl SearchAnime {
    pub fn new<A>(repo: A) -> Self
    where
        A: 'static + AnimeRepository + Sync + Send,
    {
        Self {
            repo: Box::new(repo),
        }
    }
}

#[async_trait]
impl Usecase for SearchAnime {
    type Params = String;
    type Return = Option<Vec<Anime>>;

    async fn call(&self, s: &Self::Params) -> Self::Return {
        self.repo.search_anime(&s).await.ok()
    }
}

pub struct GetEpisodesOfAnime {
    repo: Box<dyn AnimeRepository + Send + Sync>,
}

impl GetEpisodesOfAnime {
    pub fn new<A>(repo: A) -> Self
    where
        A: AnimeRepository + Send + Sync + 'static,
    {
        Self {
            repo: Box::new(repo),
        }
    }
}

#[async_trait]
impl Usecase for GetEpisodesOfAnime {
    type Params = Anime;
    type Return = Option<Vec<Episode>>;

    async fn call(&self, anime: &Self::Params) -> Self::Return {
        self.repo.get_anime_episodes(anime).await.ok()
    }
}

pub struct GetStreamingLink {
    repo: Box<dyn AnimeRepository + Send + Sync>,
}

impl GetStreamingLink {
    pub fn new<A>(repo: A) -> Self
    where
        A: AnimeRepository + Send + Sync + 'static,
    {
        Self {
            repo: Box::new(repo),
        }
    }
}

#[async_trait]
impl Usecase for GetStreamingLink {
    type Params = Episode;
    type Return = Option<String>;

    async fn call(&self, ep: &Self::Params) -> Self::Return {
        self.repo.get_streaming_link(ep).await.ok()
    }
}

#[cfg(test)]
mod tests {
    use super::super::repositories::*;
    use super::*;
    use mockall::predicate::*;

    #[tokio::test]
    async fn should_search_for_anime_in_repository() {
        let mut mock_repo = MockAnimeRepository::new();
        mock_repo
            .expect_search_anime()
            .times(1)
            .with(eq("some anime"))
            .returning(|_| Ok(vec![Anime::new("some match", "")]));

        let usecase = SearchAnime::new(mock_repo);

        let result = usecase.call(&"some anime".to_string()).await.unwrap();
        assert_eq!(&vec![Anime::new("some match", "")], &result);
    }

    #[tokio::test]
    async fn should_get_list_of_eps_of_anime() {
        let mut mock_repo = MockAnimeRepository::new();

        mock_repo
            .expect_get_anime_episodes()
            .times(1)
            .with(eq(Anime::new("some title", "some desc")))
            .returning(|_| Ok(vec![Episode::new("some ep")]));

        let usecase = GetEpisodesOfAnime::new(mock_repo);

        let result = usecase
            .call(&Anime::new("some title", "some desc"))
            .await
            .unwrap();
        assert_eq!(vec![Episode::new("some ep")], result);
    }

    #[tokio::test]
    async fn should_get_streaming_link_for_ep() {
        let mut mock_repo = MockAnimeRepository::new();

        mock_repo
            .expect_get_streaming_link()
            .times(1)
            .with(eq(Episode::new("some title")))
            .returning(|_| Ok(String::from("some link")));
        let usecase = GetStreamingLink::new(mock_repo);

        let result = usecase.call(&Episode::new("some title")).await.unwrap();

        assert_eq!(&"some link", &result);
    }
}