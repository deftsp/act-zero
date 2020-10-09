//! `async-std`-specific functionality

use std::time::Instant;

use futures::future::{BoxFuture, FutureExt};
use futures::task::{Spawn, SpawnError};

use crate::{timer, Actor, Addr};

/// Type representing the async-std runtime.
#[derive(Debug, Copy, Clone, Default)]
pub struct Runtime;

/// Alias for a timer based on async-std. This type can be default-constructed.
pub type Timer = timer::Timer<Runtime>;

/// Provides an infallible way to spawn an actor onto the async-std runtime,
/// equivalent to `Addr::new`.
pub fn spawn_actor<T: Actor>(actor: T) -> Addr<T> {
    Addr::new(&Runtime, actor).unwrap()
}

impl Spawn for Runtime {
    fn spawn_obj(&self, future: futures::future::FutureObj<'static, ()>) -> Result<(), SpawnError> {
        async_std::task::spawn(future);
        Ok(())
    }
}

impl timer::SupportsTimers for Runtime {
    type Delay = BoxFuture<'static, ()>;
    fn delay(&self, deadline: Instant) -> Self::Delay {
        let duration = deadline.saturating_duration_since(Instant::now());
        async_std::task::sleep(duration).boxed()
    }
}
