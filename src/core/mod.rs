use std::{collections::HashMap, hash::Hash};

pub mod cli_args;
pub mod delivery_mechanisms;

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

pub struct Cache<K, V>(pub HashMap<K, V>);

#[cfg_attr(test, automock)]
impl<K, V> Cache<K, V>
where
    K: 'static + Eq + Hash,
    V: 'static,
{
    pub fn get<'a>(&'a self, key: &K) -> Option<&'a V> {
        self.0.get(key)
    }

    pub fn put(&mut self, key: K, val: V) {
        self.0.insert(key, val);
    }
}
impl<K, V> Cache<K, V> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_cache_value() {
        let mut cache = Cache::<i32, i32>::new();

        assert_eq!(cache.get(&42), None);
        assert_eq!(
            cache.get(&42),
            None,
            "Should not put into cache in prev. get()"
        );

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
