use std::sync::Arc;

#[cfg_attr(test, double)]
use super::super::domain::usecases::{GetEpisodesOfAnime, SearchAnime};

use crate::{
    core::{
        presentation::{EventlessPloc, Ploc},
        Usecase,
    },
    features::watch_anime::domain::entities::{AnimeSearchItem, Episode},
};

pub struct SearchAutocompletePloc {
    searcher: Arc<SearchAnime>,
}

#[async_trait]
impl Ploc<SearchEvent, SearchState> for SearchAutocompletePloc {
    async fn dispatch(&self, e: SearchEvent) -> SearchState {
        match e {
            SearchEvent::Autocomplete(s) => {
                let autocomplete = self.searcher.call(&s).await;

                if matches!(autocomplete, Some(ref autocomplete) if autocomplete.len() > 0) {
                    SearchState::Autocomplete(autocomplete.unwrap())
                } else {
                    return SearchState::Empty;
                }
            }
        }
    }

    async fn initial_state(&self) -> SearchState {
        SearchState::Empty
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum SearchEvent {
    Autocomplete(String),
}

#[derive(Debug, PartialEq, Eq)]
pub enum SearchState {
    Autocomplete(Vec<AnimeSearchItem>),
    Empty,
}

impl SearchAutocompletePloc {
    pub fn new(searcher: Arc<SearchAnime>) -> Self {
        Self { searcher }
    }
}

pub struct EpisodeSelectorPloc {
    ep_list: GetEpisodesOfAnime,
    anime: AnimeSearchItem,
}

pub enum EpisodeSelectionEvent {}

#[derive(Debug, PartialEq, Eq)]
pub enum EpisodeSelectionState {
    DisplayEpList(AnimeSearchItem, Vec<Episode>),
    NoEpisodes(AnimeSearchItem),
}

#[async_trait]
impl Ploc<EventlessPloc, EpisodeSelectionState> for EpisodeSelectorPloc {
    async fn dispatch(&self, _: EventlessPloc) -> EpisodeSelectionState {
        self.initial_state().await
    }

    async fn initial_state(&self) -> EpisodeSelectionState {
        let eps = self.ep_list.call(&self.anime).await;
        if matches!(eps, Some(ref eps) if eps.len() > 0) {
            EpisodeSelectionState::DisplayEpList(self.anime.clone(), eps.unwrap())
        } else {
            EpisodeSelectionState::NoEpisodes(self.anime.clone())
        }
    }
}

impl EpisodeSelectorPloc {
    pub fn new(ep_list: GetEpisodesOfAnime, anime: AnimeSearchItem) -> Self {
        Self { ep_list, anime }
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
            let mock_searcher = SearchAnime::default();
            let ploc = SearchAutocompletePloc::new(mock_searcher);
            let result = ploc.initial_state().await;

            assert_eq!(result, SearchState::Empty);
        }

        #[tokio::test]
        async fn should_provide_autocomplete() {
            let mut mock_searcher = SearchAnime::default();

            mock_searcher
                .expect_call()
                .times(1)
                .with(eq(String::from("some search")))
                .returning(|_| Some(vec![AnimeSearchItem::new("some match", "_")]));

            let ploc = SearchAutocompletePloc::new(mock_searcher);

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

            let ploc = SearchAutocompletePloc::new(mock_searcher);

            let result = ploc
                .dispatch(SearchEvent::Autocomplete(String::from("some search")))
                .await;

            assert_eq!(result, SearchState::Empty);
        }

        #[tokio::test]
        async fn should_provide_empty_autocomplete_on_none() {
            let mut mock_searcher = SearchAnime::default();

            mock_searcher
                .expect_call()
                .times(1)
                .with(eq(String::from("some search")))
                .returning(|_| None);

            let ploc = SearchAutocompletePloc::new(mock_searcher);

            let result = ploc
                .dispatch(SearchEvent::Autocomplete(String::from("some search")))
                .await;

            assert_eq!(result, SearchState::Empty);
        }
    }

    mod select_episode {
        use mockall::predicate::eq;

        use super::super::*;
        use crate::features::watch_anime::{
            domain::entities::Episode,
            presentation::ploc::{EpisodeSelectionState, EpisodeSelectorPloc},
        };

        #[tokio::test]
        async fn should_provide_initial_state_with_ep_list_for_anime() {
            let mut mock_ep_selector = GetEpisodesOfAnime::default();
            mock_ep_selector
                .expect_call()
                .times(1)
                .with(eq(AnimeSearchItem::new("some anime title", "_")))
                .returning(|_| Some(vec![Episode::new("some ep title", "_", 1)]));

            let ploc = EpisodeSelectorPloc::new(
                mock_ep_selector,
                AnimeSearchItem::new("some anime title", "_"),
            );
            let result = ploc.initial_state().await;

            assert_eq!(
                result,
                EpisodeSelectionState::DisplayEpList(
                    AnimeSearchItem::new("some anime title", "_"),
                    vec![Episode::new("some ep title", "_", 1)]
                )
            );
        }

        #[tokio::test]
        async fn should_provide_empty_state_when_usecase_ret_none() {
            let mut mock_ep_selector = GetEpisodesOfAnime::default();
            mock_ep_selector
                .expect_call()
                .times(1)
                .with(eq(AnimeSearchItem::new("some anime title", "_")))
                .returning(|_| None);

            let ploc = EpisodeSelectorPloc::new(
                mock_ep_selector,
                AnimeSearchItem::new("some anime title", "_"),
            );
            let result = ploc.initial_state().await;

            assert_eq!(
                result,
                EpisodeSelectionState::NoEpisodes(AnimeSearchItem::new("some anime title", "_"))
            );
        }

        #[tokio::test]
        async fn should_provide_empty_state_when_usecase_ret_empty_vec() {
            let mut mock_ep_selector = GetEpisodesOfAnime::default();
            mock_ep_selector
                .expect_call()
                .times(1)
                .with(eq(AnimeSearchItem::new("some anime title", "_")))
                .returning(|_| Some(vec![]));

            let ploc = EpisodeSelectorPloc::new(
                mock_ep_selector,
                AnimeSearchItem::new("some anime title", "_"),
            );
            let result = ploc.initial_state().await;

            assert_eq!(
                result,
                EpisodeSelectionState::NoEpisodes(AnimeSearchItem::new("some anime title", "_"))
            );
        }
    }
}
