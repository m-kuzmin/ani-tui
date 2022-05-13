#[cfg(not(test))]
use reqwest::Client;

use std::collections::HashMap;

type Link = (String, QueryParams);
type QueryParams = Option<Vec<(String, String)>>;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait WebClient {
    async fn get(&self, url: &str, query_param: QueryParams) -> Option<String>;
}

pub struct CachedWebClient {
    client: Client,
    cache: WebCache<Link, String>,
}

impl CachedWebClient {
    pub fn new(client: Client, cache: WebCache<Link, String>) -> Self {
        Self { client, cache }
    }
}

#[async_trait]
impl WebClient for CachedWebClient {
    async fn get(&self, url: &str, query_params: QueryParams) -> Option<String> {
        if let Some(cached) = self.cache.get(&(url.to_string(), query_params.clone())) {
            return Some(cached);
        }

        let mut rq_builder = self.client.get(url);
        if let Some(ref query_params) = query_params {
            rq_builder = rq_builder.query(query_params);
        }

        let responce = rq_builder.send().await.ok()?.text().await.ok()?;

        self.cache.put(&(url.to_string(), query_params), &responce);
        Some(responce.to_owned())
    }
}

pub struct Cache<K, V>(pub HashMap<K, V>);

#[cfg_attr(test, double)]
use Cache as WebCache;

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

#[cfg(test)]
use MockRequestBuilder as RequestBuilder;

#[cfg_attr(test, automock)]
impl<K, V> Cache<K, V>
where
    K: 'static,
    V: 'static,
{
    pub fn get(&self, key: &K) -> Option<V> {
        unimplemented!()
    }

    pub fn put(&self, key: &K, val: &V) {
        unimplemented!()
    }
}
impl<K, V> Cache<K, V> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}
#[cfg(test)]
mod tests {
    use mockall::{predicate::eq, Sequence};

    use super::*;

    #[tokio::test]
    async fn should_cache_responce_when_request_not_found_in_cache_and_give_page_as_string() {
        let mut mock_cache = WebCache::<Link, String>::default();
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
