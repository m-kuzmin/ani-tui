/// Interface for all anime search and watch implementors.
#[async_trait]
#[allow(missing_docs)]
pub trait AnimeRepository {
    type SearchResult;
    type Identifier;
    type Episode;
    type Link;
    type Detail;
    async fn search(&self, query: &str) -> Result<Self::SearchResult>;
    async fn list_eps(&self, _: Self::Identifier) -> Result<Vec<Self::Episode>>;
    async fn detail(&self, _: Self::Identifier) -> Result<Self::Detail>;
    async fn watch_link(&self, _: Self::Identifier) -> Result<Self::Link>;
}

#[derive(Debug)]
#[allow(missing_docs)]
pub enum AnimeRepositoryError {
    /// Nothing was found
    NotFound,
    /// This operation could not be performed by this implementor.
    /// Keep in mind that it doesnt mean that you will get `Unsopported` error for every query parameter.
    Unsupported,
    /// Connection error
    ConnectionError,
}

#[allow(missing_docs)]
pub type Result<T> = std::result::Result<T, AnimeRepositoryError>;
