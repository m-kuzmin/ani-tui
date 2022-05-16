use std::sync::Arc;

use easy_scraper::Pattern;
use once_cell::unsync::OnceCell;
use regex::Regex;
use tokio::sync::Mutex;

use crate::{
    core::Model,
    features::watch_anime::domain::entities::{Anime, AnimeSearchItem, Episode},
};

// TODO replace with vec<anime search item model>
#[derive(Debug, PartialEq, Eq)]
pub struct SearchResultModel {
    pub anime_list: Vec<(String, String)>,
}

impl Model for SearchResultModel {
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
                        m.as_str().to_string()
                    } else {
                        m["title"].clone()
                    }
                } else {
                    m["title"].clone()
                };
                let link = m["ident"].clone();
                (title, link)
            })
            .collect();
        Some(Self { anime_list })
    }
}

/// An Anime search result
#[derive(Debug, PartialEq, Eq)]
pub struct AnimeSearchItemModel {
    /// Anime title
    pub title: String,
    /// Part of a URL that identifies this anime
    pub ident: String,
}

impl From<&AnimeSearchItem> for AnimeSearchItemModel {
    fn from(source: &AnimeSearchItem) -> AnimeSearchItemModel {
        AnimeSearchItemModel {
            title: source.title.clone(),
            ident: source.ident().to_string(),
        }
    }
}

// TODO remove or use
#[derive(Debug, PartialEq, Eq)]
pub struct AnimeModel {
    pub title: String,
    pub desc: String,
}

impl From<AnimeModel> for Anime {
    fn from(source: AnimeModel) -> Self {
        Self {
            title: source.title,
            desc: source.desc,
        }
    }
}

impl From<&Anime> for AnimeModel {
    fn from(source: &Anime) -> Self {
        Self {
            title: source.title.clone(),
            desc: source.desc.clone(),
        }
    }
}

impl Model for AnimeModel {
    fn from_html(html: &str) -> Option<Self> {
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

        if let Some(m) = pattern.matches(html).get(0) {
            Some(Self {
                title: m["anime_title"].clone(),
                desc: m["desc"].clone(),
            })
        } else {
            None
        }
    }
}

/// An anime episode model
#[derive(Debug, PartialEq, Eq)]
pub struct EpisodeModel {
    /// Episode title
    pub title: String,
    /// Anime identiifer. See [`AnimeSearchItemModel::ident`]
    pub ident: String,
    /// Episode number
    pub ep_number: usize,
}

impl EpisodeModel {
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
        Self::new(&source.title, &source.ident(), source.ep_number)
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
        let result = SearchResultModel::from_html(&html).unwrap();

        assert_eq!(
            result,
            SearchResultModel {
                anime_list: vec![
                    (String::from("Some Anime"), String::from("some-anime")),
                    (
                        String::from("Some Other Anime"),
                        String::from("some-unmatching-link")
                    ),
                    (
                        String::from("This dark Episode: Doesnt end with ep number"),
                        String::from("break-follow-ep")
                    )
                ]
            }
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
    fn should_parse_ep_html_to_anime_model() {
        let html = fixture("some-anime-episode-1.html");
        let result = AnimeModel::from_html(&html).unwrap();

        assert_eq!(
            result,
            AnimeModel {
                title: String::from("Anime title"),
                desc: String::from("Multiline\n\ndescription")
            }
        );
    }
}
