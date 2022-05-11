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
}

impl Episode {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
        }
    }
}
