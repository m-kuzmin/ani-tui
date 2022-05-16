/// Presentation event to state mappings
pub mod ploc;

/// Dependency resolution
#[doc(hidden)]
#[cfg(not(test))]
mod di {
    use std::sync::Arc;

    use once_cell::sync::OnceCell;

    use crate::core::dependency_resolution::{Dependency, Resolve};

    use super::ploc::{EpisodeQueryPloc, SearchAutocompletePloc};

    impl Resolve<Arc<SearchAutocompletePloc>> for Dependency {
        fn resolve() -> Arc<SearchAutocompletePloc> {
            static INSTANCE: OnceCell<Arc<SearchAutocompletePloc>> = OnceCell::new();
            INSTANCE
                .get_or_init(|| Arc::new(SearchAutocompletePloc::new(Dependency::resolve())))
                .clone()
        }
    }

    impl Resolve<Arc<EpisodeQueryPloc>> for Dependency {
        fn resolve() -> Arc<EpisodeQueryPloc> {
            static INSTANCE: OnceCell<Arc<EpisodeQueryPloc>> = OnceCell::new();
            INSTANCE
                .get_or_init(|| Arc::new(EpisodeQueryPloc::new(Dependency::resolve())))
                .clone()
        }
    }
}
