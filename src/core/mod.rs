use std::{collections::HashMap, hash::Hash};

/// CLI arg parsing
pub mod cli_args;
/// Data layer utils
pub mod delivery_mechanisms;
/// Resolves interfaces in constructors to their implementors
pub mod dependency_resolution;
/// Presentation layer utils
pub mod presentation;

/// Exposes a `call()` method on a usecase struct.
#[async_trait]
pub trait Usecase {
    /// [`call()`][c] arguments
    ///
    /// [c]: Self::call()
    type Params;
    /// [`call()`][c] return type
    ///
    /// [c]: Self::call()

    type Return: Send + Sync;
    /// Execute a usecase
    async fn call(&self, _: &Self::Params) -> Self::Return;
}

/// Implemented by models to create objects from raw data representations
pub trait Model {
    /// Tries to Convert an HTML document to a Model
    fn from_html(s: &str) -> Option<Self>
    where
        Self: Sized;
}

/// A key-value cache
pub struct Cache<K, V>(pub HashMap<K, V>);

#[cfg_attr(test, automock)]
impl<K, V> Cache<K, V>
where
    K: 'static + Eq + Hash,
    V: 'static,
{
    /// Try get a value from a cache. [`None`] means that the value hasn't been cached yet.
    pub fn get<'a>(&'a self, key: &K) -> Option<&'a V> {
        self.0.get(key)
    }

    /// Store a value in a cache
    pub fn put(&mut self, key: K, val: V) {
        self.0.insert(key, val);
    }
}

impl<K, V> Cache<K, V> {
    /// Creates a new cache
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}

impl<K, V> Default for Cache<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_give_none_for_uncached_values() {
        let cache = Cache::<i32, i32>::new();

        assert_eq!(cache.get(&42), None);
        assert_eq!(
            cache.get(&42),
            None,
            "Should not put into cache in prev. get()"
        );
    }

    #[test]
    fn should_cache_value() {
        let mut cache = Cache::<i32, i32>::new();

        cache.put(42, 13);

        assert_eq!(cache.get(&42), Some(&13));
        assert_eq!(
            cache.get(&42),
            Some(&13),
            "Should not delete value after get"
        );
        assert_eq!(cache.get(&13), None);
    }

    #[test]
    fn should_replace_value() {
        let mut cache = Cache::<i32, i32>::new();

        cache.put(42, 1);
        assert_eq!(cache.get(&42), Some(&1));

        cache.put(42, 2);
        assert_eq!(cache.get(&42), Some(&2));
    }
}
