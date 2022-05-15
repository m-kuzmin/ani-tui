#[async_trait]
pub trait Ploc<Event, State> {
    async fn initial_state(&self) -> State;
    async fn dispatch(&self, e: Event) -> State;
}

pub type EventlessPloc = ();
