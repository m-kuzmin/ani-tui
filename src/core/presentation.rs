/// **P**resentation **Lo**gic **C**ompoent
#[async_trait]
pub trait Ploc<Event, State> {
    /// Get the initial state
    async fn initial_state(&self) -> State;
    /// Send an event to PLoC and get a new state back
    async fn dispatch(&self, e: Event) -> State;
}

/// An event type for PLoCs that cant receive events.
/// Implementation of [`Ploc::dispatch(EventlessPloc)`][dsp] is not limited in any way.
///
/// [dsp]: Ploc::dispatch()
pub type EventlessPloc = ();
