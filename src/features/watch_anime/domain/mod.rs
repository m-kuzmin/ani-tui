/// Data type definitions
pub mod entities;
/// Repository interfaces
pub mod repositories;
/// Usecases that interact with a repository interface
pub mod usecases;

/// Dependency resolution
#[doc(hidden)]
#[cfg(not(test))]
pub mod di {
    use std::sync::Arc;

    use once_cell::sync::OnceCell;

    use crate::core::dependency_resolution::{Dependency, Resolve};

    use super::usecases::{GetEpisodesOfAnime, SearchAnime};

    impl Resolve<Arc<SearchAnime>> for Dependency {
        fn resolve() -> Arc<SearchAnime> {
            static INSTANCE: OnceCell<Arc<SearchAnime>> = OnceCell::new();
            INSTANCE
                .get_or_init(|| Arc::new(SearchAnime::new(Dependency::resolve())))
                .clone()
        }
    }

    impl Resolve<Arc<GetEpisodesOfAnime>> for Dependency {
        fn resolve() -> Arc<GetEpisodesOfAnime> {
            static INSTANCE: OnceCell<Arc<GetEpisodesOfAnime>> = OnceCell::new();
            INSTANCE
                .get_or_init(|| Arc::new(GetEpisodesOfAnime::new(Dependency::resolve())))
                .clone()
        }
    }
}
