use std::error::Error;

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
