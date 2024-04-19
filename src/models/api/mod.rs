use serde::{de::DeserializeOwned, Serialize};
use std::hash::Hash;
use std::{fmt::Display, future::Future};

use super::cache::traits::has_cache::HasCache;
use super::cache::traits::merge::UpdateCachedEntity;

pub trait HasFetchApi<K> {
    /// Fetch an item an put it into the cache
    ///
    /// This operation isn't deduplicated! Refer to the Diskcache for safe call
    fn fetch_and_insert(key: &K) -> impl Future<Output = color_eyre::Result<Self>>;
}

pub trait HasCacheAndFetchApi<K>: HasCache<K> + HasFetchApi<K>
where
    K: Eq + Hash + Clone + Display,
    Self: DeserializeOwned + Serialize + UpdateCachedEntity + HasFetchApi<K>,
{
    /// Get the data from the cache, or call the API. Any request is deduplicated
    fn get_cached_or_fetch(key: &K) -> impl Future<Output = color_eyre::Result<Self>> {
        async {
            match Self::get_cache().get(key)? {
                Some(val) => Ok(val),
                None => Self::get_cache().get_or_fetch(key).await,
            }
        }
    }
}

impl<K, T> HasCacheAndFetchApi<K> for T
where
    K: Eq + Hash + Clone + Display,
    T: DeserializeOwned + Serialize + UpdateCachedEntity + HasFetchApi<K> + HasCache<K>,
{
}
