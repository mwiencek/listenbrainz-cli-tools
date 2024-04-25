use std::{future::Future, sync::Arc};

use serde::{de::DeserializeOwned, Serialize};

use crate::models::cache::disk_cache::DiskCacheWrapper;

/// For all the entities that have a cache
pub trait HasCache<K>: Serialize + DeserializeOwned {
    /// Get the cache correponding to the entity
    fn get_cache() -> Arc<DiskCacheWrapper<K, Self>>;

    async fn get_cached(key: K) -> color_eyre::Result<Option<Self>> {
        Self::get_cache().get(key)
    }

    fn set(key: K, value: Self) -> impl Future<Output = color_eyre::Result<()>> {
        Self::get_cache().set(key, value)
    }

    fn set_or_update(key: K, value: Self) -> impl Future<Output = color_eyre::Result<()>> {
        Self::get_cache().set_or_update(key, value)
    }
}