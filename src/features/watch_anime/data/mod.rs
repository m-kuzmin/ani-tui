/// Model providers
pub mod datasources;
/// Data type definitions
pub mod models;
/// Implementation of domain layer repository interfaces
pub mod repositories;

/// Dependency resolution
#[doc(hidden)]
#[cfg(not(test))]
pub mod di {
    use once_cell::sync::OnceCell;
    use std::sync::Arc;

    use crate::core::{
        delivery_mechanisms::{CachingWebClient, Link, WebClient},
        dependency_resolution::{Dependency, Resolve},
        Cache,
    };

    use super::{
        super::domain::repositories::AnimeRepositoryContract,
        datasources::{GoGoPlayDataSource, GoGoPlayInterface},
        repositories::AnimeRepository,
    };

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
                    Arc::new(CachingWebClient::new(
                        reqwest::Client::builder().user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:100.0) Gecko/20100101 Firefox/100.0").build().unwrap(),
                        Cache::<Link, String>::new(),
                    ))
                })
                .clone()
        }
    }
}
