
use easy_scraper::Pattern;
use std::io::Write;
use std::{
    process::{Command, Stdio},
    sync::Arc,
    thread::spawn,
};


use super::models::{AnimeDetailsModel, AnimeSearchItemModel, EpisodeModel};
use crate::core::{delivery_mechanisms::WebClient, Model};

/// Implements [`GoGoPlayInterface`]
pub struct GoGoPlayDataSource {
    /// A webclient
    client: Arc<dyn WebClient + Send + Sync>,
}

impl GoGoPlayDataSource {
    /// Creates a new GoGoPlay datasource
    pub fn new(client: Arc<dyn WebClient + Send + Sync>) -> Self {
        Self { client }
    }
}

/// <https://goload.pro>-specific interface
#[cfg_attr(test, automock)]
#[async_trait]
pub trait GoGoPlayInterface {
    /// Searches for anime
    async fn search_anime(&self, title: &str) -> Option<Vec<AnimeSearchItemModel>>;

    /// Provides episode list for anime
    async fn get_anime_episode_list(
        &self,
        anime: AnimeSearchItemModel,
    ) -> Option<Vec<EpisodeModel>>;

    /// Provides a streaming link for anime episode
    async fn get_streaming_link(&self, ep: &EpisodeModel) -> Option<String>;

    /// Provides detailed information about an anime
    async fn get_anime_details(&self, anime: &AnimeSearchItemModel) -> Option<AnimeDetailsModel>;
}

#[async_trait]
impl GoGoPlayInterface for GoGoPlayDataSource {
    /// Makes a get request to <https://goload.pro/search.html?keyword={TITLE}> and returns a parsed list of anime.
    async fn search_anime(&self, title: &str) -> Option<Vec<AnimeSearchItemModel>> {
        let html = self
            .client
            .get(
                "https://goload.pro/search.html",
                Some(vec![("keyword".to_string(), title.to_string())]),
            )
            .await?;
        Vec::<AnimeSearchItemModel>::from_html(&html)
    }

    /// Makes a get request to episode 1 of an anime and returns all episodes on page (<https://goload.pro/videos/{ANIME_IDENTIFIER}-episode-1>)
    async fn get_anime_episode_list(
        &self,
        anime: AnimeSearchItemModel,
    ) -> Option<Vec<EpisodeModel>> {
        let html = self
            .client
            .get(
                &format!("https://goload.pro/videos/{}-episode-1", anime.ident),
                None,
            )
            .await?;
        if &html == "404\n" {
            return None;
        }

        Vec::<EpisodeModel>::from_html(&html)
    }

    async fn get_streaming_link(&self, ep: &EpisodeModel) -> Option<String> {
        //! Isn't tested because this implementaion is prone to frequent updates

        let html = self
            .client
            .get(
                &format!(
                    "https://goload.pro/videos/{title}-episode-{ep_number}",
                    title = ep.ident,
                    ep_number = ep.ep_number
                ),
                None,
            )
            .await?;

        let iframe_link = || -> Option<String> {
            let pattern = Pattern::new(r#"<iframe src="{{link}}" allowfullscreen="true" frameborder="0" marginwidth="0" marginheight="0" scrolling="no" />"#).unwrap();
            Some("https:".to_string() + &pattern.matches(&html).get(0)?["link"])
        };
        let html = self.client.get(&iframe_link()?, None).await?;

        let (token, secret_key, second_key, iv, id) =
            || -> Option<(String, String, String, String, String)> {
                let pattern = Pattern::new(r#"
<head>
   <script type="text/javascript" src="https://goload.pro/js/crypto-js/crypto-js.js?v=9.988" data-name="episode" data-value="{{token}}"></script>
</head>
<body class="container-{{secret_key}}">
    <input type="hidden" id="id" value="{{id}}">
    <!--
        Must have all inputs, otherwise easy_scraper wont match
    -->
    <input type="hidden" id="title" value="{{_}}">
    <input type="hidden" id="typesub" value="{{_}}">
    <div class="wrapper container-{{iv}}">
        <div class="videocontent videocontent-{{second_key}}">
        </div>
    </div>
</body>
"#).unwrap();

                let matches = pattern.matches(&html);
                let matches = matches.get(0)?;
                Some((
                    matches["token"].clone(),
                    matches["secret_key"].clone(),
                    matches["second_key"].clone(),
                    matches["iv"].clone(),
                    matches["id"].clone(),
                ))
            }()?;

        let token = base64::decode(token).ok()?;
        let secret_key = hex::encode(&secret_key.into_bytes());
        let id_vecu8 = id.as_bytes().to_vec();
        let second_key = hex::encode(&second_key.into_bytes());
        let iv = hex::encode(&iv.into_bytes());

        let mut openssl = Command::new("openssl")
            .args(&["enc", "-d", "-aes256", "-K", &secret_key, "-iv", &iv])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .ok()?;
        let mut stdin = openssl.stdin.take()?;
        spawn(move || {
            stdin.write_all(&token).ok().unwrap();
        });
        let token = String::from_utf8(openssl.wait_with_output().ok()?.stdout).ok()?;

        let mut openssl = Command::new("openssl")
            .args(&["enc", "-e", "-aes256", "-K", &secret_key, "-iv", &iv])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .ok()?;
        let mut stdin = openssl.stdin.take()?;
        spawn(move || {
            stdin.write_all(&id_vecu8).unwrap();
        });
        let ajax_id = base64::encode(openssl.wait_with_output().ok()?.stdout);

        let json = self
            .client
            .get_with_headers(
                &format!(
                    "https://goload.pro/encrypt-ajax.php?id={ajax_id}&alias={id}&{token}",
                    ajax_id = ajax_id,
                    id = id,
                    token = &token.split_at(token.find("token")?).1,
                ),
                None,
                Some(
                    [(
                        String::from("X-Requested-With"),
                        String::from("XMLHttpRequest"),
                    )]
                    .to_vec(),
                ),
            )
            .await?;
        let regex = regex::Regex::new(r#""data":"(.*?)""#).unwrap();
        let json = regex.captures(&json)?.get(1)?.as_str().replace("\\", "");

        let json = base64::decode(json).ok()?;

        let mut openssl = Command::new("openssl")
            .args(&["enc", "-d", "-aes256", "-K", &second_key, "-iv", &iv])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .ok()?;
        let mut stdin = openssl.stdin.take()?;
        spawn(move || {
            stdin.write_all(&json).ok().unwrap();
        });

        let json = String::from_utf8(openssl.wait_with_output().ok()?.stdout).ok()?;
        let regex = regex::Regex::new(r#""file":"(.*?)""#).unwrap();
        let link = regex.captures(&json)?.get(1)?.as_str().replace("\\", "");

        Some(link)
    }

    async fn get_anime_details(&self, anime: &AnimeSearchItemModel) -> Option<AnimeDetailsModel> {
        let html = self
            .client
            .get(
                &format!("https://goload.pro/videos/{}-episode-1", anime.ident),
                None,
            )
            .await?;

        if &html == "404\n" {
            return None;
        }
        AnimeDetailsModel::from_html(&html)
    }
}

#[cfg(test)]
mod tests {
    use crate::core::delivery_mechanisms::MockWebClient as WebClient;
    use mockall::predicate::*;
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

    #[tokio::test]
    async fn should_give_anime_list_from_search_query_on_gogoplay() {
        let mut mock_client = WebClient::new();
        mock_client
            .expect_get()
            .times(1)
            .with(
                eq("https://goload.pro/search.html"),
                eq(Some(vec![(
                    "keyword".to_string(),
                    "some anime".to_string(),
                )])),
            )
            .returning(|_, _| Some(fixture("search.html")));

        let datasource = GoGoPlayDataSource::new(Arc::new(mock_client));
        let result = datasource.search_anime("some anime").await.unwrap();

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
        );
    }

    #[tokio::test]
    async fn should_give_list_of_eps_for_anime_from_gogoplay() {
        let mut mock_client = WebClient::new();

        mock_client
            .expect_get()
            .times(1)
            .with(
                eq("https://goload.pro/videos/some-ident-episode-1"),
                eq(None),
            )
            .returning(|_, _| Some(fixture("some-anime-episode-1.html")));

        let datasource = GoGoPlayDataSource::new(Arc::new(mock_client));

        let result = datasource
            .get_anime_episode_list(AnimeSearchItemModel::new("", "some-ident"))
            .await
            .unwrap();

        assert_eq!(
            result,
            vec![
                EpisodeModel::new("Episode 2 title", "some-ident", 2),
                EpisodeModel::new("Episode 1 title", "some-ident", 1)
            ]
        );
    }

    #[tokio::test]
    async fn should_detect_404_when_getting_ep_list() {
        let mut mock_client = WebClient::new();

        mock_client
            .expect_get()
            .times(1)
            .with(
                eq("https://goload.pro/videos/some-ident-episode-1"),
                eq(None),
            )
            .returning(|_, _| Some(String::from("404\n")));

        let datasource = GoGoPlayDataSource::new(Arc::new(mock_client));

        let result = datasource
            .get_anime_episode_list(AnimeSearchItemModel::new("", "some-ident"))
            .await;

        assert_eq!(result, None);
    }

    #[tokio::test]
    async fn should_get_anime_details() {
        let mut mock_client = WebClient::new();

        mock_client
            .expect_get()
            .times(1)
            .with(
                eq("https://goload.pro/videos/some-ident-episode-1"),
                eq(None),
            )
            .returning(|_, _| Some(fixture("some-anime-episode-1.html")));

        let datasource = GoGoPlayDataSource::new(Arc::new(mock_client));

        let result = datasource
            .get_anime_details(&AnimeSearchItemModel::new("some title", "some-ident"))
            .await
            .unwrap();

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
        );
    }
}
