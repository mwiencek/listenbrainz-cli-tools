use serde::{de::DeserializeOwned, Serialize};
use crate::models::data::entity_traits::has_cache::HasCache;

/// This trait is for all the entities that can be inserted in the cache
pub trait Insertable<K>: HasCache<K> {}