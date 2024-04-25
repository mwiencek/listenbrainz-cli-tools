use std::fmt::Display;

use crate::models::data::entity_traits::has_cache::HasCache;

pub trait ApiReturn<K, T: HasCache<K>> {
    /// Convert the API return to cache entities for insertion
    fn to_entities(self) -> (T, Vec<Box<impl HasCache<dyn Display>>>);
}