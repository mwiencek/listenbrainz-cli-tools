use std::{fmt::Display, hash::Hash, sync::Arc};

use cached::DiskCacheError;
use serde::{de::DeserializeOwned, Serialize};

use crate::models::cache::disk_cache::DiskCacheWrapper;

use super::merge::UpdateCachedEntity;

/// Trait for all the entities that have a cache
pub trait HasCache<K>
where
    K: Eq + Hash + Clone + Display,
    Self: DeserializeOwned + Serialize + UpdateCachedEntity,
{
    fn get_cache() -> Arc<DiskCacheWrapper<K, Self>>;

    fn get_from_cache(key: &K) -> Result<Option<Self>, DiskCacheError> {
        Self::get_cache().get(key)
    }

    fn set(key: K, value: Self) -> Result<Option<Self>, DiskCacheError> {
        Self::get_cache().set(key, value)
    }

    fn set_or_update(key: K, value: Self) -> Result<Option<Self>, DiskCacheError> {
        Self::get_cache().set_or_update(key, value)
    }
}
