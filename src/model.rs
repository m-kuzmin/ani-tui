use std::{
    cmp::Ordering,
    convert::Infallible,
    fmt::{self, Display},
    ops::Deref,
    str::FromStr,
};

#[derive(Debug)]
pub struct Anime {
    pub title: String,
    pub id: Identifier<String>,
    pub desc: String,
    pub eps: Vec<Episode>,
}

impl PartialEq for Anime {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Anime {}

impl fmt::Display for Anime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"{title} ({id})
{eps} episodes.

{desc}
"#,
            title = self.title,
            id = self.id,
            desc = self.desc,
            eps = self.eps.len()
        )
    }
}

#[derive(Debug)]
pub struct Episode {
    pub title: String,
    pub series: Identifier<String>,
    pub n: String,
}

impl fmt::Display for Episode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"[{n}] {title} ({series})"#,
            n = self.n,
            title = self.title,
            series = self.series,
        )
    }
}

impl PartialEq for Episode {
    fn eq(&self, other: &Self) -> bool {
        self.series == other.series && self.n == other.n
    }
}

impl PartialOrd for Episode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.series == other.series {
            return Some(self.n.cmp(&other.n));
        }
        None
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Identifier<T>(T);

impl<T: Clone> Identifier<T> {
    pub fn new(v: &T) -> Self {
        Self(v.clone())
    }
}

impl<T> Deref for Identifier<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

impl FromStr for Identifier<String> {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

impl<T: Display> Display for Identifier<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
