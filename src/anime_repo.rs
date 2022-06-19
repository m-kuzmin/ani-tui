/// Interface for all anime search and watch implementors.
#[async_trait]
#[allow(missing_docs)]
pub trait AnimeRepository {
    /// Data that is a result of a search query
    type SearchResult;
    /// An ID that can be used to perform queries on a season
    type Identifier;
    /// Contains the relevant info about an episode
    type Episode;
    /// Contains episode details
    type Detail;
    /// Contains the link to a file that can be played in a video player
    type Link;
    /// Performs a search in the data source.
    /// When implementing it is recommended to make the return type contain [`Self::Identifier`] that can be used in other functions.
    async fn search(&self, query: &str) -> Result<Self::SearchResult>;
    /// Lists the episodes in a series
    async fn list_eps(&self, _: Self::Identifier) -> Result<Vec<Self::Episode>>;
    /// Returns details about an episode in a series
    async fn detail(&self, _: Self::Episode) -> Result<Self::Detail>;
    /// Returns a watch link that can be played in a video player
    async fn watch_link(&self, _: Self::Episode) -> Result<Self::Link>;
}

/// An error returned from [`AnimeRepository`]
#[derive(Debug)]
pub enum AnimeRepositoryError {
    /// Nothing was found
    NotFound,
    /// This operation could not be performed by this implementor.
    /// Keep in mind that it doesnt mean that you will get `Unsopported` error for every query parameter.
    Unsupported,
    /// Datasource could not process your request. This could be caused by an internet connection error or a missing file.
    DatasourceError,
}

/// A result type shortcut returned by AnimeRepository
pub type Result<T> = std::result::Result<T, AnimeRepositoryError>;
