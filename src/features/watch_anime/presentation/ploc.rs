use std::sync::Arc;

#[cfg_attr(test, double)]
use super::super::domain::usecases::{GetEpisodesOfAnime, SearchAnime};

use crate::{
    core::{presentation::Ploc, Usecase},
    features::watch_anime::domain::entities::{AnimeSearchItem, Episode},
};

/// Autocompletes anime search
pub struct SearchAutocompletePloc {
    /// A search anime usecase
    searcher: Arc<SearchAnime>,
}

#[async_trait]
impl Ploc<SearchEvent, SearchState> for SearchAutocompletePloc {
    async fn dispatch(&self, e: SearchEvent) -> SearchState {
        match e {
            SearchEvent::Autocomplete(s) => {
                let autocomplete = self.searcher.call(&s).await;

                if matches!(autocomplete, Some(ref autocomplete) if !autocomplete.is_empty()) {
                    SearchState::Autocomplete(autocomplete.unwrap())
                } else {
                    return SearchState::NoSuggestions;
                }
            }
        }
    }

    async fn initial_state(&self) -> SearchState {
        SearchState::NoSuggestions
    }
}

/// [Autocomplete][SearchAutocompletePloc] events
#[derive(Debug, PartialEq, Eq)]
pub enum SearchEvent {
    /// A string to provide autocomplete for
    Autocomplete(String),
}

///[Autocomplete][SearchAutocompletePloc] state
#[derive(Debug, PartialEq, Eq)]
pub enum SearchState {
    /// Autocomplete options
    Autocomplete(Vec<AnimeSearchItem>),
    /// No suggestions
    NoSuggestions,
}

impl SearchAutocompletePloc {
    /// Creates a new Ploc
    pub fn new(searcher: Arc<SearchAnime>) -> Self {
        Self { searcher }
    }
}

/// Provides a list of episodes for an anime
pub struct EpisodeQueryPloc {
    /// An episode query usecase
    ep_lister: Arc<GetEpisodesOfAnime>,
}

/// A request to fetch a list of episodes for an anime
pub struct EpisodeQueryEvent(pub AnimeSearchItem);

/// Information about anime's episodes
#[derive(Debug, PartialEq, Eq)]
pub enum EpisodeQueryState {
    /// A list of episodes for an anime
    DisplayEpList(Vec<Episode>),
    /// No episodes in an anime
    NoEpisodes,
}

#[async_trait]
impl Ploc<EpisodeQueryEvent, EpisodeQueryState> for EpisodeQueryPloc {
    /// Provides a list of episodes for an anime or [`NoEpisodes`][EpisodeQueryState::NoEpisodes]
    async fn dispatch(&self, query: EpisodeQueryEvent) -> EpisodeQueryState {
        let eps = self.ep_lister.call(&query.0).await;
        if matches!(eps, Some(ref eps) if !eps.is_empty()) {
            EpisodeQueryState::DisplayEpList(
                eps.expect("matches!() should guearantee that this never panics"),
            )
        } else {
            EpisodeQueryState::NoEpisodes
        }
    }

    async fn initial_state(&self) -> EpisodeQueryState {
        //! Always sets the initial state to [`NoEpisodes`][EpisodeQueryState::NoEpisodes]
        EpisodeQueryState::NoEpisodes
    }
}

impl EpisodeQueryPloc {
    /// Creates a new Ploc
    pub fn new(ep_lister: Arc<GetEpisodesOfAnime>) -> Self {
        Self { ep_lister }
    }
}

#[cfg(test)]
mod tests {
    mod search_anime {
        use super::super::super::super::domain::entities::AnimeSearchItem;
        use super::super::*;
        use mockall::predicate::eq;

        #[tokio::test]
        async fn should_provide_initial_state() {
            let mock_searcher = Arc::new(SearchAnime::default());
            let ploc = SearchAutocompletePloc::new(mock_searcher);
            let result = ploc.initial_state().await;

            assert_eq!(result, SearchState::NoSuggestions);
        }

        #[tokio::test]
        async fn should_provide_autocomplete() {
            let mut mock_searcher = SearchAnime::default();

            mock_searcher
                .expect_call()
                .times(1)
                .with(eq(String::from("some search")))
                .returning(|_| Some(vec![AnimeSearchItem::new("some match", "_")]));

            let ploc = SearchAutocompletePloc::new(Arc::new(mock_searcher));

            let result = ploc
                .dispatch(SearchEvent::Autocomplete(String::from("some search")))
                .await;

            assert_eq!(
                result,
                SearchState::Autocomplete(vec![AnimeSearchItem::new("some match", "_")])
            );
        }

        #[tokio::test]
        async fn should_provide_empty_autocomplete_vec_on_empty_vec() {
            let mut mock_searcher = SearchAnime::default();

            mock_searcher
                .expect_call()
                .times(1)
                .with(eq(String::from("some search")))
                .returning(|_| Some(vec![]));

            let ploc = SearchAutocompletePloc::new(Arc::new(mock_searcher));

            let result = ploc
                .dispatch(SearchEvent::Autocomplete(String::from("some search")))
                .await;

            assert_eq!(result, SearchState::NoSuggestions);
        }

        #[tokio::test]
        async fn should_provide_empty_autocomplete_on_none() {
            let mut mock_searcher = SearchAnime::default();

            mock_searcher
                .expect_call()
                .times(1)
                .with(eq(String::from("some search")))
                .returning(|_| None);

            let ploc = SearchAutocompletePloc::new(Arc::new(mock_searcher));

            let result = ploc
                .dispatch(SearchEvent::Autocomplete(String::from("some search")))
                .await;

            assert_eq!(result, SearchState::NoSuggestions);
        }
    }

    mod select_episode {
        use mockall::predicate::eq;

        use super::super::*;
        use crate::features::watch_anime::presentation::ploc::{
            EpisodeQueryPloc, EpisodeQueryState,
        };

        #[tokio::test]
        async fn should_provide_initial_state_as_empty() {
            let mock_ep_selector = GetEpisodesOfAnime::default();
            let ploc = EpisodeQueryPloc::new(Arc::new(mock_ep_selector));
            let result = ploc.initial_state().await;

            assert_eq!(result, EpisodeQueryState::NoEpisodes);
        }

        #[tokio::test]
        async fn should_provide_empty_state_when_usecase_ret_none() {
            let mut mock_ep_selector = GetEpisodesOfAnime::default();
            mock_ep_selector
                .expect_call()
                .times(1)
                .with(eq(AnimeSearchItem::new("some anime title", "_")))
                .returning(|_| None);

            let ploc = EpisodeQueryPloc::new(Arc::new(mock_ep_selector));
            let anime = AnimeSearchItem::new("some anime title", "_");
            let result = ploc.dispatch(EpisodeQueryEvent(anime)).await;

            assert_eq!(result, EpisodeQueryState::NoEpisodes);
        }

        #[tokio::test]
        async fn should_provide_empty_state_when_usecase_ret_empty_vec() {
            let mut mock_ep_selector = GetEpisodesOfAnime::default();
            mock_ep_selector
                .expect_call()
                .times(1)
                .with(eq(AnimeSearchItem::new("some anime title", "_")))
                .returning(|_| Some(vec![]));

            let anime = AnimeSearchItem::new("some anime title", "_");
            let ploc = EpisodeQueryPloc::new(Arc::new(mock_ep_selector));
            let result = ploc.dispatch(EpisodeQueryEvent(anime)).await;

            assert_eq!(result, EpisodeQueryState::NoEpisodes);
        }
    }
}
