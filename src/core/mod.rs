#[async_trait]
pub trait Usecase {
    type Params;
    type Return: Send + Sync;
    async fn call(&self, _: &Self::Params) -> Self::Return;
}

pub trait Model {
    fn from_html(s: &str) -> Option<Self>
    where
        Self: Sized;
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait WebClient {
    async fn get(&self, url: &str, query_param: Option<Vec<(String, String)>>) -> Option<String>;
}
