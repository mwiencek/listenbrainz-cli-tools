use cached::DiskCacheError;
use serde::{de::DeserializeOwned, Serialize};

use super::{has_cache::HasCache, has_id::HasID, merge::UpdateCachedEntity};
use std::{fmt::Display, hash::Hash};

pub trait HasIDAndCache<K>
where
    K: Eq + Hash + Clone + Display,
    Self: DeserializeOwned + Serialize + UpdateCachedEntity + HasID<K> + HasCache<K>,
{
    fn set_with_id(value: Self) -> Result<Option<Self>, DiskCacheError> {
        Self::set(value.get_id(), value)
    }
}
