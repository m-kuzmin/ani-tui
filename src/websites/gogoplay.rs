use std::{
    io::Write,
    process::{Command, Stdio},
    thread::spawn,
};

use crate::anime_repo::{self, AnimeRepository, AnimeRepositoryError};
use easy_scraper::Pattern;
use regex::{escape, Regex};
use reqwest::{
    header::{HeaderMap, USER_AGENT},
    Client,
};

use QueryError::{ConnectionError, InvalidLink, ParsingError};

/// A link to the website
pub const BASE_URL: &'static str = "https://goload.pro";

/// <https://goload.pro> API.
pub struct Gogoplay {
    web_client: Client,
}

impl Gogoplay {
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self {
            web_client: Client::builder().default_headers(with!{
                mut HeaderMap::new() =>
                    .insert(USER_AGENT, "Mozilla/5.0 (X11; Linux x86_64; rv:101.0) Gecko/20100101 Firefox/101.0".parse().expect("Could not set User Agent for web client"))
            }).build().expect("Could not build a web client"),
        }
    }
}

#[async_trait]
impl AnimeRepository for Gogoplay {
    type SearchResult = SearchPage;
    type Identifier = Identifier;
    type Episode = EpisodeLink;
    type Link = String;
    type Detail = Detail;

    async fn search(&self, query: &str) -> anime_repo::Result<Self::SearchResult> {
        self.search(query)
            .await
            .ok_or(AnimeRepositoryError::DatasourceError)
    }

    async fn list_eps(&self, url: Self::Identifier) -> anime_repo::Result<Vec<Self::Episode>> {
        Ok(self.episode_page(url).await?.ep_list)
    }

    async fn detail(&self, ep: Self::Episode) -> anime_repo::Result<Self::Detail> {
        let content = self.episode_page(ep.link).await?;
        Ok(Detail {
            anime_title: content.anime_title,
            description: content.description,
        })
    }

    /// Returns a link to the iframe
    async fn watch_link(&self, ep: Self::Episode) -> anime_repo::Result<Self::Link> {
        let iframe_link = self.episode_page(ep.link).await?.iframe;
        let iframe = self.iframe_page(&iframe_link).await?;

        enum Mode {
            Enc,
            Dec,
        }

        fn openssl(mode: Mode, data: Vec<u8>, key: &str, iv: &str) -> Option<Vec<u8>> {
            let mut openssl = Command::new("openssl")
                .args(&[
                    "enc",
                    match mode {
                        Mode::Enc => "-e",
                        Mode::Dec => "-d",
                    },
                    "-aes256",
                    "-K",
                    key,
                    "-iv",
                    iv,
                ])
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                //.stderr(Stdio::piped())
                .spawn()
                .ok()?;
            let mut stdin = openssl.stdin.take()?;
            let writer = spawn(move || {
                stdin.write_all(&data).unwrap();
            });
            let result = openssl.wait_with_output().ok()?.stdout;
            writer.join().ok()?;
            Some(result)
        }

        let token = String::from_utf8(
            openssl(Mode::Dec, iframe.token, &iframe.secret_key, &iframe.iv).ok_or(ParsingError)?,
        )
        .map_err(|_| ParsingError)?;

        let ajax_id = base64::encode(
            openssl(
                Mode::Enc,
                iframe.id.as_bytes().to_vec(),
                &iframe.secret_key,
                &iframe.iv,
            )
            .ok_or(ParsingError)?,
        );

        let json = self
            .web_client
            .get(&format!(
                "https://goload.pro/encrypt-ajax.php?id={ajax_id}&alias={id}&{token}",
                ajax_id = ajax_id,
                id = iframe.id,
                token = &token.split_at(token.find("token").ok_or(ParsingError)?).1,
            ))
            .header("X-Requested-With", "XMLHttpRequest")
            .send()
            .await
            .map_err(|_| ConnectionError)?
            .text()
            .await
            .map_err(|_| ConnectionError)?;

        let regex = regex::Regex::new(r#""data":"(.*?)""#).unwrap();
        let enc_link = base64::decode(
            regex
                .captures(&json)
                .ok_or(ParsingError)?
                .get(1)
                .ok_or(ParsingError)?
                .as_str()
                .replace(r"\", ""),
        )
        .map_err(|_| ParsingError)?;

        let json = String::from_utf8(
            openssl(Mode::Dec, enc_link, &iframe.second_key, &iframe.iv).ok_or(ParsingError)?,
        )
        .map_err(|_| ParsingError)?;

        let regex = regex::Regex::new(r#""file":"(.*?)""#).unwrap();
        let link = regex
            .captures(&json)
            .ok_or(ParsingError)?
            .get(1)
            .ok_or(ParsingError)?
            .as_str()
            .replace(r"\", "");

        Ok(link)
    }
}

impl Gogoplay {
    /// Returns content on a search page, given a title to search for
    ///
    /// # Return value
    ///
    /// Returns None in case of connection errors with the server
    pub async fn search(&self, title: &str) -> Option<SearchPage> {
        let html = self
            .web_client
            .get("https://goload.pro/search.html")
            .query(&[("keyword", title)])
            .send()
            .await
            .ok()?
            .text()
            .await
            .ok()?;

        let pattern = Pattern::new(
            r#"
<div class="video_player followed default">
    <ul class="listing items">
        <li class="video-block ">
            <a href="{{link}}">
                <div class="name">
                  {{title}}
                </div>
            </a>
        </li>
    </ul>
</div>"#,
        )
        .unwrap();

        Some({
            let mut eps = Vec::new();
            for ep in pattern.matches(&html) {
                eps.push(EpisodeLink {
                    title: ep.get("title").unwrap().to_string(),
                    link: Identifier::from_link(&with! {
                        mut BASE_URL.to_string() =>
                            .push_str(ep.get("link").unwrap())
                    })?,
                })
            }
            eps
        })
    }

    /// Get all the relevant info on a page
    pub async fn episode_page(&self, url: Identifier) -> Result<EpisodePage, QueryError> {
        let html = self
            .web_client
            .get(url.as_link())
            .send()
            .await
            .or(Err(ConnectionError))?
            .text()
            .await
            .or(Err(ConnectionError))?;

        let info_pattern = Pattern::new(
            r#"
<div class="video-info">
  <div class="video-info-left">
    <h1>{{ep_title}}</h1>
    ...
    <div class="video-details">
      <span class="date">{{anime_title}}</span>
      <div class="post-entry">
        <div class="content-more-js" id="rmjs-1">{{description}}</div>
      </div>
    </div>
  </div>
</div>"#,
        )
        .unwrap();

        let episode_pattern = Pattern::new(
            r#"
<div class="video-info">
  <div class="video-info-left">
    <ul class="listing items lists">
      <li class="video-block ">
        <a href="{{ep_link}}">
          <div class="name">
            {{ep_title}}
          </div>
        </a>
      </li>
    </ul>
  </div>
</div>"#,
        )
        .unwrap();

        let iframe_pattern = Pattern::new(r#"<iframe src="{{link}}" allowfullscreen="true" frameborder="0" marginwidth="0" marginheight="0" scrolling="no" />"#).unwrap();

        let m = info_pattern.matches(&html);
        let info = m.get(0).ok_or(ParsingError)?;
        let episodes = episode_pattern.matches(&html);

        Ok(EpisodePage {
            link: url,
            ep_title: info["ep_title"].to_string(),
            anime_title: info["anime_title"].to_string(),
            description: info["description"].to_string(),
            ep_list: {
                let mut eps = Vec::new();
                for ep in episodes {
                    eps.push(EpisodeLink {
                        title: ep["ep_title"].to_string(),
                        link: Identifier::from_link(&with! {
                            mut BASE_URL.to_string() =>
                                .push_str(&ep["ep_link"])
                        })
                        .ok_or(InvalidLink)?,
                    })
                }
                eps
            },
            iframe: with! {
                mut String::from("https:") =>
                    .push_str(
                        &iframe_pattern
                            .matches(&html)
                            .get(0)
                            .ok_or(InvalidLink)
                            ?["link"])
            },
        })
    }

    /// Returns content on player iframe page
    pub async fn iframe_page(&self, link: &str) -> Result<IframePage, QueryError> {
        if !link.starts_with(&format!("{}/streaming.php", BASE_URL)) {
            return Err(QueryError::InvalidLink);
        }

        let html = self
            .web_client
            .get(link)
            .send()
            .await
            .or(Err(QueryError::ConnectionError))?
            .text()
            .await
            .or(Err(QueryError::ConnectionError))?;

        let pattern = Pattern::new(r#"
<head>
   <script type="text/javascript" src="https://goload.pro/js/crypto-js/crypto-js.js?v=9.988" data-name="episode" data-value="{{token}}"></script>
</head>
<body class="container-{{secret_key}}">
    <input type="hidden" id="id" value="{{id}}">
    ...
    <div class="wrapper container-{{iv}}">
        <div class="videocontent videocontent-{{second_key}}">
        </div>
    </div>
</body>
"#).unwrap();

        let matches = pattern.matches(&html);
        let matches = matches.get(0).ok_or(QueryError::ParsingError)?;

        let get =
            |name: &str| -> Result<&str, QueryError> { Ok(matches.get(name).ok_or(ParsingError)?) };

        Ok(IframePage {
            token: base64::decode(get("token")?).map_err(|_| ParsingError)?,
            secret_key: hex::encode(get("secret_key")?.as_bytes()),
            second_key: hex::encode(get("second_key")?.as_bytes()),
            iv: hex::encode(get("iv")?.as_bytes()),
            id: get("id")?.to_string(),
        })
    }
}

/// An identifier for an episode
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identifier {
    /// ID of the anime
    pub id: String,
    /// Episode number of the anime
    pub ep: usize,
}

/// A source prefix in the string representation of Identifier
pub const REPR_PREFIX: &'static str = "GLP-1";

impl Identifier {
    /// Takes a URL link `"https://goload.pro/..."` and returns a parsed object
    pub fn from_link(url: &str) -> Option<Self> {
        let cap = Regex::new(&format!(
            "^{}/videos/(?P<id>.*?)-episode-(?P<ep>.*?)$",
            escape(BASE_URL)
        ))
        .unwrap()
        .captures(url)?;
        Some(Self {
            id: cap.name("id")?.as_str().to_string(),
            ep: cap.name("ep")?.as_str().parse().ok()?,
        })
    }

    /// Takes a user friendly identifier and returns a parsed object
    pub fn from_repr(url: &str) -> Option<Self> {
        let cap = Regex::new(&format!(r"^<{}:(?P<id>.*?)#(?P<ep>.*?)>$", REPR_PREFIX))
            .unwrap()
            .captures(url)?;
        Some(Self {
            id: cap.name("id")?.as_str().to_string(),
            ep: cap.name("ep")?.as_str().parse().ok()?,
        })
    }

    /// Makes a new URL from self
    pub fn as_link(&self) -> String {
        format!(
            "{base}/videos/{id}-episode-{ep}",
            base = BASE_URL,
            id = self.id,
            ep = self.ep
        )
    }

    /// Makes a new user friendly formatted identifier from self
    pub fn as_repr(&self) -> String {
        format!(
            "<{prefix}:{id}#{ep}>",
            prefix = REPR_PREFIX,
            id = self.id,
            ep = self.ep
        )
    }
}

/// A search page type
pub type SearchPage = Vec<EpisodeLink>;

/// An element of a result list on a search page
#[derive(Debug, Clone)]
pub struct EpisodeLink {
    /// Title of the element
    pub title: String,
    /// Link to the content
    pub link: Identifier,
}

/// Content on the page for an anime episode
#[derive(Debug)]
pub struct EpisodePage {
    /// A link to this page
    pub link: Identifier,
    /// Title of the episode
    pub ep_title: String,
    /// Title of the anime
    pub anime_title: String,
    /// Description on this page. Could be shared between all episodes of an anime,
    /// or it could be diferent for every episode.
    pub description: String,
    /// List of other episodes on this page
    pub ep_list: Vec<EpisodeLink>,
    /// Link to the player in the iframe
    pub iframe: String,
}

/// Anime details
pub struct Detail {
    /// Anime title
    pub anime_title: String,
    /// Anime description
    pub description: String,
}

/// An error that could occur as a result of a query
#[derive(Debug, PartialEq, Eq)]
pub enum QueryError {
    /// Connection to the server could not be established
    ConnectionError,
    /// Returned if the url supplied doesnt pass the validation checks
    InvalidLink,
    /// Occurs when a page could not be parsed into a struct
    ParsingError,
}

impl From<QueryError> for AnimeRepositoryError {
    fn from(source: QueryError) -> Self {
        match source {
            QueryError::ConnectionError => AnimeRepositoryError::DatasourceError,
            QueryError::InvalidLink => AnimeRepositoryError::Unsupported,
            QueryError::ParsingError => AnimeRepositoryError::Unsupported,
        }
    }
}

/// Parsed content from the player iframe
#[derive(Debug)]
pub struct IframePage {
    /// Contains encrypted data used for fetching download URL
    pub token: Vec<u8>,
    /// 1st encryption key
    pub secret_key: String,
    /// 2nd encryption key
    pub second_key: String,
    /// Encryption IV (initialization vector)
    pub iv: String,
    /// Anime ID
    pub id: String,
}
