use std::cell::RefCell;

#[cfg_attr(test, double)]
use super::Cache;
#[cfg(not(test))]
use reqwest::Client;

pub type Link = (String, QueryParams);
pub type QueryParams = Option<Vec<(String, String)>>;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait WebClient {
    async fn get(&self, url: &str, query_param: QueryParams) -> Option<String>;
}

pub struct CachedWebClient {
    client: Client,
    cache: Mutex<RefCell<Cache<Link, String>>>,
}

impl CachedWebClient {
    pub fn new(client: Client, cache: Cache<Link, String>) -> Self {
        Self {
            client,
            cache: Mutex::new(RefCell::new(cache)),
        }
    }
}

#[async_trait]
impl WebClient for CachedWebClient {
    async fn get(&self, url: &str, query_params: QueryParams) -> Option<String> {
        if let Some(cached) = self
            .cache
            .lock()
            .await
            .borrow()
            .get(&(url.to_string(), query_params.clone()))
        {
            return Some(cached.to_string());
        }

        let mut rq_builder = self.client.get(url);
        if let Some(ref query_params) = query_params {
            rq_builder = rq_builder.query(query_params);
        }

        let responce = rq_builder.send().await.ok()?.text().await.ok()?;

        self.cache
            .lock()
            .await
            .borrow_mut()
            .put((url.to_string(), query_params), responce.clone());
        Some(responce.to_string())
    }
}

#[cfg(test)]
mock!(pub Client {
    pub fn new() -> Self;
    pub fn get(&self, url: &str) -> RequestBuilder;

});

#[cfg(test)]
mock!(pub RequestBuilder {
    pub fn query(&self, query: &Vec<(String, String)>) -> Self;
    pub async fn send(&self) -> Result<MockResponce, ()>;
});

#[cfg(test)]
mock!(pub Responce {
    #[allow(private_in_public)]
    pub async fn text(&self) -> Result<String, ()>;
});

#[cfg(test)]
use MockClient as Client;

use tokio::sync::Mutex;
#[cfg(test)]
use MockRequestBuilder as RequestBuilder;

#[cfg(test)]
mod tests {
    use mockall::{predicate::eq, Sequence};

    use super::*;

    #[tokio::test]
    async fn should_cache_responce_when_request_not_found_in_cache_and_give_page_as_string() {
        let mut mock_cache = Cache::<Link, String>::default();
        let mut mock_client = Client::default();
        let mut seq = Sequence::new();

        mock_cache
            .expect_get()
            .times(1)
            .in_sequence(&mut seq)
            .with(eq((String::from("https://test.com"), None)))
            .returning(|_| None);

        mock_client
            .expect_get()
            .times(1)
            .in_sequence(&mut seq)
            .with(eq("https://test.com"))
            .returning(|_| {
                let mut mock_req_builder = RequestBuilder::new();
                mock_req_builder.expect_send().times(1).returning(|| {
                    Ok({
                        let mut mock_resp = MockResponce::new();
                        mock_resp
                            .expect_text()
                            .times(1)
                            .returning(|| Ok(String::from("<h1>[PASS] Test!</h1>")));
                        mock_resp
                    })
                });
                mock_req_builder
            });

        mock_cache
            .expect_put()
            .times(1)
            .in_sequence(&mut seq)
            .with(
                eq((String::from("https://test.com"), None)),
                eq(String::from("<h1>[PASS] Test!</h1>")),
            )
            .returning(|_, _| {});

        let web_client = CachedWebClient::new(mock_client, mock_cache);

        let result = web_client.get("https://test.com", None).await.unwrap();
        assert_eq!(result, "<h1>[PASS] Test!</h1>");
    }
}
