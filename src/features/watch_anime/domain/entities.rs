/// An anime search result item
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AnimeSearchItem {
    /// Title of an anime
    pub title: String,

    /// From [`AnimeSearchItemModel`][si]
    ///
    /// [si]: crate::features::watch_anime::data::models::AnimeSearchItemModel
    #[doc(hidden)]
    _ident: String,
}

impl AnimeSearchItem {
    /// Creates a new AnimeSearchItem
    pub fn new(title: &str, ident: &str) -> Self {
        Self {
            title: title.to_string(),
            _ident: ident.to_string(),
        }
    }

    /// Returns a identifier from [`AnimeSearchItemModel`][si]
    ///
    /// [si]: crate::features::watch_anime::data::models::AnimeSearchItemModel
    pub fn ident(&self) -> &str {
        &self._ident
    }
}

/// Details from anime page
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnimeDetails {
    /// Anime Title
    pub title: String,
    /// Description
    pub desc: String,
    /// List of anime episodes
    pub eps: Vec<Episode>,

    /// From anime model
    #[doc(hidden)]
    _ident: String,
}

impl AnimeDetails {
    /// Creates a new instance
    pub fn new(title: &str, desc: &str, eps: Vec<Episode>, ident: &str) -> Self {
        Self {
            title: title.to_string(),
            desc: desc.to_string(),
            _ident: ident.to_string(),
            eps,
        }
    }

    /// Gets the internal identfier
    pub fn ident(&self) -> &str {
        &self._ident
    }
}

/// Stores information about an episode
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Episode {
    /// Episode's title
    pub title: String,
    /// Episode number
    pub ep_number: usize,

    /// From [`EpisodeModel`][em]
    ///
    /// [em]: crate::features::watch_anime::data::models::EpisodeModel
    #[doc(hidden)]
    _ident: String,
}

impl Episode {
    /// Creates a new episode entity
    pub fn new(title: &str, ident: &str, ep_number: usize) -> Self {
        Self {
            title: title.to_string(),
            _ident: ident.to_string(),
            ep_number,
        }
    }

    /// Returns an identifier from [`EpisodeModel`][em]
    ///
    /// [em]: crate::features::watch_anime::data::models::EpisodeModel
    pub fn ident(&self) -> &str {
        &self._ident
    }
}
