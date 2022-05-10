use thiserror::Error;

#[derive(Debug, Error)]
pub enum BrowserError {
    #[error("Request error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Parser error: {0}")]
    ParserError(#[from] ParserError),
    #[error("Nothing found")]
    NotFound404,
}

pub type BrowserResult<T> = Result<T, self::BrowserError>;

#[derive(Debug, Error)]
pub enum ParserError {
    #[error("Could not parse title")]
    Title,
    #[error("Could not parse identifier")]
    Identifier,
    #[error("Could not parse description")]
    Description,
    #[error("Could not parse episode: {0}")]
    Episode(LinkParseError),
    #[error("Could not parse search results: {0}")]
    Search(LinkParseError),
}

#[derive(Debug, Error)]
pub enum LinkParseError {
    #[error("Nothing found")]
    None,
    #[error("No title")]
    Title,
    #[error("No identifier")]
    Identifier,
}
pub type ParserResult<T> = Result<T, self::ParserError>;
