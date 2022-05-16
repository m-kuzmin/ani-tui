pub mod data;
pub mod domain;
pub mod presentation;

#[cfg(not(test))]
pub mod di {
    use once_cell::sync::OnceCell;
    use std::sync::Arc;

    use crate::core::{
        delivery_mechanisms::{CachedWebClient, Link, WebClient},
        dependency_resolution::{Dependency, Resolve},
        Cache,
    };

    use super::{
        data::{
            datasources::{GoGoPlayDataSource, GoGoPlayInterface},
            repositories::AnimeRepository,
        },
        domain::{repositories::AnimeRepositoryContract, usecases::SearchAnime},
        presentation::ploc::SearchAutocompletePloc,
    };

    impl Resolve<Arc<SearchAutocompletePloc>> for Dependency {
        fn resolve() -> Arc<SearchAutocompletePloc> {
            static INSTANCE: OnceCell<Arc<SearchAutocompletePloc>> = OnceCell::new();
            INSTANCE
                .get_or_init(|| Arc::new(SearchAutocompletePloc::new(Dependency::resolve())))
                .clone()
        }
    }

    impl Resolve<Arc<SearchAnime>> for Dependency {
        fn resolve() -> Arc<SearchAnime> {
            static INSTANCE: OnceCell<Arc<SearchAnime>> = OnceCell::new();
            INSTANCE
                .get_or_init(|| Arc::new(SearchAnime::new(Self::resolve())))
                .clone()
        }
    }
    impl Resolve<Arc<dyn AnimeRepositoryContract + Send + Sync>> for Dependency {
        fn resolve() -> Arc<dyn AnimeRepositoryContract + Send + Sync> {
            static INSTANCE: OnceCell<Arc<dyn AnimeRepositoryContract + Send + Sync>> =
                OnceCell::new();
            INSTANCE
                .get_or_init(|| Arc::new(AnimeRepository::new(Dependency::resolve())))
                .clone()
        }
    }

    impl Resolve<Arc<dyn GoGoPlayInterface + Send + Sync>> for Dependency {
        fn resolve() -> Arc<dyn GoGoPlayInterface + Send + Sync> {
            static INSTANCE: OnceCell<Arc<dyn GoGoPlayInterface + Send + Sync>> = OnceCell::new();
            INSTANCE
                .get_or_init(|| Arc::new(GoGoPlayDataSource::new(Dependency::resolve())))
                .clone()
        }
    }

    impl Resolve<Arc<dyn WebClient + Send + Sync>> for Dependency {
        fn resolve() -> Arc<dyn WebClient + Send + Sync> {
            static INSTANCE: OnceCell<Arc<dyn WebClient + Send + Sync>> = OnceCell::new();
            INSTANCE
                .get_or_init(|| {
                    Arc::new(CachedWebClient::new(
                        reqwest::Client::builder().user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:100.0) Gecko/20100101 Firefox/100.0").build().unwrap(),
                        Cache::<Link, String>::new(),
                    ))
                })
                .clone()
        }
    }
}
