use std::error::Error;

use easy_scraper::Pattern;

use crate::{core::Model, features::watch_anime::domain::entities::Anime};

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

#[derive(Debug, PartialEq, Eq)]
pub struct EpisodeModel {
    pub title: String,
}

impl Model for EpisodeModel {
    fn from_html(html: &str) -> Option<Self> {
        let pattern = Pattern::new(
            r#"
<div class="video-info">
  <div class="video-info-left">
    <h1>{{ep_title}}</h1>
  </div>
</div>"#,
        )
        .unwrap();

        if let Some(m) = pattern.matches(html).get(0) {
            Some(Self {
                title: m["ep_title"].clone(),
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{fmt::format, fs::File, io::Read};

    use super::*;

    fn fixture(file: &str) -> String {
        let mut content = String::new();
        File::open(format!("tests/fixtures/{}", file))
            .unwrap()
            .read_to_string(&mut content);
        content
    }

    #[test]
    fn should_parse_html_to_anime_model() {
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

    #[test]
    fn should_parse_html_to_episode_model() {
        let html = fixture("some-anime-episode-1.html");

        let result = EpisodeModel::from_html(&html).unwrap();

        assert_eq!(
            result,
            EpisodeModel {
                title: String::from("Episode title"),
            }
        );
    }
}
