use easy_scraper::Pattern;

use lazy_static::__Deref;
use regex::Regex;

use crate::{
    core::Model,
    features::watch_anime::domain::entities::{AnimeDetails, AnimeSearchItem, Episode},
};

/// An item in anime search results
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnimeSearchItemModel {
    /// Anime title
    pub title: String,
    /// Part of a URL that identifies this anime
    pub ident: String,
}

impl Model for Vec<AnimeSearchItemModel> {
    fn from_html(html: &str) -> Option<Self> {
        let pattern = Pattern::new(
            r#"
<div class="video_player followed  default">
    <ul class="listing items">
        <li class="video-block ">
            <a href="/videos/{{ident}}-episode-{{_}}">
                <div class="name">
                  {{title}}
                </div>
            </a>
        </li>
    </ul>
</div>
"#,
        )
        .unwrap();

        let remove_ep_number_from_title = Regex::new(r"(?m)(.+?)(?: Episode \d+)?$").unwrap();

        let anime_list = pattern
            .matches(html)
            .into_iter()
            .map(|m| {
                let title = if let Some(cap) = remove_ep_number_from_title.captures(&m["title"]) {
                    if let Some(m) = cap.get(1) {
                        m.as_str()
                    } else {
                        &m["title"]
                    }
                } else {
                    &m["title"]
                };
                let ident: &str = &m["ident"];
                AnimeSearchItemModel::new(title, ident)
            })
            .collect();
        Some(anime_list)
    }
}

impl AnimeSearchItemModel {
    /// Creates a new Anime Search item
    pub fn new(title: &str, ident: &str) -> Self {
        Self {
            title: title.to_string(),
            ident: ident.to_string(),
        }
    }
}
impl From<&AnimeSearchItem> for AnimeSearchItemModel {
    fn from(source: &AnimeSearchItem) -> Self {
        Self {
            title: source.title.clone(),
            ident: source.ident().to_string(),
        }
    }
}

impl From<AnimeSearchItemModel> for AnimeSearchItem {
    fn from(source: AnimeSearchItemModel) -> Self {
        Self::new(&source.title, &source.ident)
    }
}

/// Models data on anime page
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnimeDetailsModel {
    /// Anime title
    pub title: String,
    /// Anime description
    pub desc: String,
    /// Anime identifier
    pub ident: String,
    /// List of anime episodes
    pub eps: Vec<EpisodeModel>,
}

impl AnimeDetailsModel {
    /// Creates a new anime details model
    pub fn new(title: &str, desc: &str, eps: Vec<EpisodeModel>, ident: &str) -> Self {
        Self {
            title: title.to_string(),
            desc: desc.to_string(),
            eps,
            ident: ident.to_string(),
        }
    }
}

impl Model for AnimeDetailsModel {
    fn from_html(html: &str) -> Option<Self> {
        let eps = Vec::<EpisodeModel>::from_html(html)?;

        let pattern = Pattern::new(
            r#"
<div class="video-info">
  <div class="video-info-left">
    <div class="watch_play">
      <div class="play-video">
        <div class="video-details">
          <span class="date">{{anime_title}}</span>
          <div class="post-entry">
            <div class="content-more-js" id="rmjs-1">{{desc}}</div>
          </div>
        </div>
      </div>
    </div>
  </div>
</div>"#,
        )
        .unwrap();

        let matches = pattern.matches(html);
        let capture = matches.get(0)?;

        let ident = eps.get(0)?.ident.clone();
        Some(AnimeDetailsModel::new(
            &capture["anime_title"],
            &capture["desc"],
            eps,
            &ident,
        ))
    }
}

impl From<&AnimeDetails> for AnimeDetailsModel {
    fn from(source: &AnimeDetails) -> Self {
        Self::new(
            &source.title,
            &source.desc,
            source
                .eps
                .clone()
                .into_iter()
                .map(EpisodeModel::from)
                .collect(),
            source.ident(),
        )
    }
}

impl From<AnimeDetailsModel> for AnimeDetails {
    fn from(source: AnimeDetailsModel) -> Self {
        Self::new(
            &source.title,
            &source.desc,
            source.eps.into_iter().map(Episode::from).collect(),
            &source.ident,
        )
    }
}

/// An anime episode model
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EpisodeModel {
    /// Episode title
    pub title: String,
    /// Anime identiifer. See [`AnimeSearchItemModel::ident`]
    pub ident: String,
    /// Episode number
    pub ep_number: usize,
}

impl EpisodeModel {
    /// Creates a new Episode model
    pub fn new(title: &str, ident: &str, ep_number: usize) -> Self {
        Self {
            title: title.to_string(),
            ident: ident.to_string(),
            ep_number,
        }
    }
}

impl Model for Vec<EpisodeModel> {
    /// Creates a list of all episodes located on anime page
    fn from_html(html: &str) -> Option<Self> {
        let pattern = Pattern::new(
            r#"
<div class="video-info">
  <div class="video-info-left">
    <ul class="listing items lists">
      <li class="video-block ">
        <a href="/videos/{{ident}}-episode-{{ep_number}}">
          <div class="name">
            {{title}}
          </div>
        </a>
      </li>
    </ul>
  </div>
</div>"#,
        )
        .unwrap();

        let matches = pattern.matches(html);

        Some(
            matches
                .into_iter()
                .map(|cap| -> EpisodeModel {
                    EpisodeModel::new(
                        &cap["title"],
                        &cap["ident"],
                        cap["ep_number"].parse().unwrap_or_default(),
                    )
                })
                .collect(),
        )
    }
}

impl From<EpisodeModel> for Episode {
    fn from(source: EpisodeModel) -> Self {
        Self::new(&source.title, &source.ident, source.ep_number)
    }
}

impl From<&Episode> for EpisodeModel {
    fn from(source: &Episode) -> Self {
        Self::new(&source.title, source.ident(), source.ep_number)
    }
}

impl From<Episode> for EpisodeModel {
    fn from(source: Episode) -> Self {
        Self::new(&source.title, source.ident(), source.ep_number)
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read};

    use super::*;

    fn fixture(file: &str) -> String {
        let mut content = String::new();
        File::open(format!("tests/fixtures/{}", file))
            .unwrap()
            .read_to_string(&mut content)
            .unwrap();
        content
    }

    #[test]
    fn should_parse_gogoplay_search_page_to_search_result_model() {
        let html = fixture("search.html");
        let result = Vec::<AnimeSearchItemModel>::from_html(&html).unwrap();

        assert_eq!(
            result,
            vec![
                AnimeSearchItemModel::new("Some Anime", "some-anime"),
                AnimeSearchItemModel::new("Some Other Anime", "some-unmatching-link"),
                AnimeSearchItemModel::new(
                    "This dark Episode: Doesnt end with ep number",
                    "break-follow-ep"
                ),
            ]
        )
    }

    #[test]
    fn should_parse_ep_html_to_episode_list() {
        let html = fixture("some-anime-episode-1.html");
        let result = Vec::<EpisodeModel>::from_html(&html).unwrap();

        assert_eq!(
            result,
            vec![
                EpisodeModel::new("Episode 2 title", "some-ident", 2),
                EpisodeModel::new("Episode 1 title", "some-ident", 1)
            ]
        );
    }

    #[test]
    fn should_parse_anime_page_into_anime_details_model() {
        let html = fixture("some-anime-episode-1.html");
        let result = AnimeDetailsModel::from_html(&html).unwrap();

        assert_eq!(
            result,
            AnimeDetailsModel::new(
                "Anime title",
                "Multiline\n\ndescription",
                vec![
                    EpisodeModel::new("Episode 2 title", "some-ident", 2),
                    EpisodeModel::new("Episode 1 title", "some-ident", 1),
                ],
                "some-ident",
            )
        )
    }
}
