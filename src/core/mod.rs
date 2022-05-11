#[async_trait]
pub trait Usecase {
    type Params;
    type Return: Send + Sync;
    async fn call(&self, _: &Self::Params) -> Self::Return;
}
