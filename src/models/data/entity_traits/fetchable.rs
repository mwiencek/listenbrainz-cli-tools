use crate::models::api::api_return::ApiReturn;

use super::has_cache::HasCache;

/// Trait for all the entities that can be fetched
pub trait Fetchable<K> {
    /// Fetch the entity
    async fn fetch(key: K) -> color_eyre::Result<impl ApiReturn>;
}

// --------

pub trait FetchableAndCachable<K>: Fetchable<K> + HasCache<K> {
    /// Get the data from the cache, or call the API. Any request is deduplicated
    async fn get_cached_or_fetch(key: K) -> color_eyre::Result<K> {
        match Self::get_cached(key).await? {
            Some(val) => Ok(val),
            None => Self::get_cache().get_or_fetch(key).await,
        }
    }
}