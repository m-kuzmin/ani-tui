use easy_scraper::Pattern;
use error::{BrowserError, BrowserResult, LinkParseError, ParserError, ParserResult};
use model::{Anime, Episode, Identifier};
use reqwest::Client;

pub mod error;
pub mod model;

pub struct Browser {
    pub client: Client,
    parser: Parser,
}

impl Browser {
    pub fn new() -> Result<Self, reqwest::Error> {
        Ok(Self {
            client: Client::builder()
                .user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:99.0) Gecko/20100101 Firefox/99.0")
                .build()?,
            parser: Parser::new(),
        })
    }

    async fn get(
        &self,
        url: String,
        param: Option<Vec<(String, String)>>,
    ) -> Result<String, reqwest::Error> {
        if let Some(param) = param {
            Ok(self
                .client
                .get(url)
                .query(&param)
                .send()
                .await?
                .text()
                .await?)
        } else {
            Ok(self.client.get(url).send().await?.text().await?)
        }
    }

    pub async fn get_anime(&mut self, identifier: &Identifier<String>) -> BrowserResult<Anime> {
        let response = self
            .get(
                format!("https://goload.pro/videos/{}-episode-1", *identifier),
                None,
            )
            .await?;

        if response == "404\n" {
            return Err(BrowserError::NotFound404);
        }

        self.parser.html = response;

        Ok(Anime {
            title: self.parser.title()?,
            desc: self.parser.description()?,
            id: identifier.clone(),
            eps: self.parser.episode_list()?,
        })
    }

    pub async fn search(
        &mut self,
        query: &String,
    ) -> BrowserResult<Vec<(Identifier<String>, String)>> {
        let response = self
            .get(
                "https://goload.pro/search.html".to_string(),
                Some(vec![("keyword".to_string(), query.clone())]),
            )
            .await?;

        self.parser.html = response;

        Ok(self.parser.search()?)
    }
}

struct Parser {
    pub html: String,
    title_pattern: Pattern,
    description_pattern: Pattern,
    episodes_pattern: Pattern,
    search_page_pattern: Pattern,
}
impl Parser {
    fn new() -> Self {
        Self {
            html: String::new(),
            title_pattern: Pattern::new(
                r#"<div class="video-details"><span class="date">{{title}}</span></div>"#,
            )
            .unwrap(),
            description_pattern: Pattern::new(
                r#"
<div class="video-details">
  <div class="post-entry">
    <div class="content-more-js" id="rmjs-1">
      {{description}}
    </div>
  </div>
</div>"#,
            )
            .unwrap(),
            episodes_pattern: Pattern::new(
                r#"
<div class="video-info">
  <div class="video-info-left">
    <ul class="listing items lists">
      <li class="video-block ">
        <a href="/videos/{{identifier}}-episode-{{number}}">
          <div class="name">{{title}}</div>
        </a>
      </li>
    </ul>
  </div>
</div>"#,
            )
            .unwrap(),
            search_page_pattern: Pattern::new(
                r#"
<div class="video_player followed  default">
<ul class="listing items">
<li class="video-block ">
  <a href="/videos/{{identifier}}-episode-{{ignore}}">
  <div class="name">
    {{title}}
  </div>
  </a>
</li>
</ul>
</div>"#,
            )
            .unwrap(),
        }
    }

    fn title(&mut self) -> ParserResult<String> {
        let matches = self.title_pattern.matches(&self.html);
        if matches.len() > 0 {
            return Ok(matches[0]
                .get("title")
                .ok_or(ParserError::Title)?
                .to_string());
        }
        Err(ParserError::Title)
    }

    fn description(&mut self) -> ParserResult<String> {
        let matches = self.description_pattern.matches(&self.html);

        if matches.len() > 0 {
            return Ok(matches[0]
                .get("description")
                .ok_or(ParserError::Description)?
                .to_string());
        }
        Err(ParserError::Description)
    }

    fn episode_list(&mut self) -> ParserResult<Vec<Episode>> {
        let matches = self.episodes_pattern.matches(&self.html);
        if matches.len() > 0 {
            let mut eps = Vec::<Episode>::with_capacity(matches.len());
            for ep in matches {
                eps.push(Episode {
                    title: ep
                        .get("title")
                        .ok_or(ParserError::Episode(LinkParseError::Title))?
                        .clone(),
                    series: ep
                        .get("identifier")
                        .ok_or(ParserError::Episode(LinkParseError::Identifier))?
                        .parse()
                        .unwrap(),
                    n: ep
                        .get("number")
                        .ok_or(ParserError::Episode(LinkParseError::Identifier))?
                        .clone(),
                })
            }

            return Ok(eps);
        }
        Err(ParserError::Episode(LinkParseError::None))
    }

    fn search(&mut self) -> ParserResult<Vec<(Identifier<String>, String)>> {
        let matches = self.search_page_pattern.matches(&self.html);
        if matches.len() > 0 {
            let mut series = Vec::<(Identifier<String>, String)>::with_capacity(matches.len());
            for anime in matches {
                series.push((
                    Identifier::new(
                        anime
                            .get("identifier")
                            .ok_or(ParserError::Search(LinkParseError::Identifier))?,
                    ),
                    anime
                        .get("title")
                        .ok_or(ParserError::Search(LinkParseError::Title))?
                        .to_string(),
                ));
            }
            return Ok(series);
        }
        Err(ParserError::Search(LinkParseError::None))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{fs::File, io::Read};

    #[test]
    fn parse_title() {
        let mut html = File::open("tests/samples/kemono-jihen-episode-1.html").unwrap();
        let mut parser = Parser::new();
        html.read_to_string(&mut parser.html).unwrap();

        assert_eq!("Kemono Jihen".to_string(), parser.title().unwrap());
    }

    #[test]
    fn parse_description() {
        let desc = String::from(
            r#"When a series of animal bodies that rot away after a single night begin appearing in a remote mountain village, Inugami, a detective from Tokyo who specializes in the occult, is called to investigate.

While working the case, he befriends a strange boy who works in the field every day instead of going to school. Shunned by his peers and nicknamed "Dorota-bou" after a yokai that lives in the mud, he helps Inugami uncover the truth behind the killings—but supernatural forces are at work, and while Dorota-bou is just a nickname, it might not be the only thing about the boy that isn't human."#,
        );

        let mut html = File::open("tests/samples/kemono-jihen-episode-1.html").unwrap();
        let mut parser = Parser::new();

        html.read_to_string(&mut parser.html).unwrap();
        assert_eq!(desc, parser.description().unwrap());
    }

    #[test]
    fn parse_eps() -> std::io::Result<()> {
        let mut html = File::open("tests/samples/kemono-jihen-episode-1.html")?;
        let mut parser = Parser::new();

        html.read_to_string(&mut parser.html)?;

        let eps = parser.episode_list().unwrap();
        for n in 12..1 {
            let ep = Episode {
                title: format!("Kemono Jihen Episode {}", n),
                series: Identifier::new(&String::from("kemono-jihen")),
                n: n.to_string(),
            };

            assert_eq!(ep, eps[n]);
        }

        Ok(())
    }
}
