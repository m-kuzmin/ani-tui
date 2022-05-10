#[derive(Debug)]
pub enum BrowserError {
    Reqwest(reqwest::Error),
    ParserError(ParserError),
}

pub type BrowserResult<T> = Result<T, self::BrowserError>;

impl From<reqwest::Error> for BrowserError {
    fn from(s: reqwest::Error) -> Self {
        Self::Reqwest(s)
    }
}

impl From<ParserError> for BrowserError {
    fn from(s: ParserError) -> Self {
        Self::ParserError(s)
    }
}

#[derive(Debug)]
pub enum ParserError {
    Title,
    Identifier,
    Description,
    Episode(EpisodeError),
    Search(EpisodeError),
}

#[derive(Debug)]
pub enum EpisodeError {
    None,
    Title,
    Identifier,
}
pub type ParserResult<T> = Result<T, self::ParserError>;
