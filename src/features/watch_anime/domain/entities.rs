#[derive(Debug, PartialEq, Eq)]
pub struct AnimeSearchItem {
    pub title: String,
    _ident: String,
}

impl AnimeSearchItem {
    pub fn new(title: &str, ident: &str) -> Self {
        Self {
            title: title.to_string(),
            _ident: ident.to_string(),
        }
    }

    pub fn ident(&self) -> &str {
        &self._ident
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Anime {
    pub title: String,
    pub desc: String,
}

impl Anime {
    pub fn new(title: &str, desc: &str) -> Self {
        Self {
            title: title.to_string(),
            desc: desc.to_string(),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Episode {
    pub title: String,
    pub ep_number: usize,
    _ident: String,
}

impl Episode {
    pub fn new(title: &str, ident: &str, ep_number: usize) -> Self {
        Self {
            title: title.to_string(),
            _ident: ident.to_string(),
            ep_number,
        }
    }

    pub fn ident(&self) -> &str {
        &self._ident
    }
}
