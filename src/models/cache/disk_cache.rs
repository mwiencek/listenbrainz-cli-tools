use std::{fmt::Display, sync::Arc};

use chashmap::CHashMap;
use color_eyre::eyre::Context;

use serde::{de::DeserializeOwned, Serialize};
use tokio::sync::{Semaphore, SemaphorePermit};

use crate::{models::data::entity_traits::{fetchable::Fetchable, insertable::Insertable}, utils::cacache::disk_cacache::SerdeCacache};

use super::{traits::merge::UpdateCachedEntity, CACHE_LOCATION};
use std::hash::Hash;

pub struct DiskCacheWrapper<K, V> {
    name: String,
    cache: SerdeCacache<K, V>,
    watch_cache: CHashMap<K, Arc<Semaphore>>,
}

impl<K, V> DiskCacheWrapper<K, V> { 
    pub fn new(name: &str) -> Self {
        let mut location = CACHE_LOCATION.clone();
        location.push(name);
        Self {
            name: name.to_string(),
            cache: SerdeCacache::new(location),
            watch_cache: CHashMap::new(),
        }
    }

    pub fn get_name(&self) -> &str {&self.name}

    pub async fn set(&self, key: K, value: V) -> color_eyre::Result<()> {
        self.cache.set(&key, &value).await?;
        Ok(())
    }

    
    pub async fn get(&self, key: K) -> color_eyre::Result<Option<V>> {
        self.cache.get(&key).await
    }
}



impl<K, V> DiskCacheWrapper<K, V>
where
    K: Display + Eq + Hash + Clone,
    V: UpdateCachedEntity + Fetchable<K>,
{
    pub async fn set_or_update(&self, key: K, value: V) -> color_eyre::Result<()> {
        let cached = self.cache.get(&key).await?;

        let new;
        if let Some(cached) = cached {
            new = cached.update_entity(value);
        } else {
            new = value;
        }

        self.cache.set(&key, &new).await?;
        Ok(())
    }


    fn get_semaphore(&self, key: &K) -> Arc<Semaphore> {
        if let Some(semaphore) = self.watch_cache.get(key) {
            return (*semaphore).clone();
        }

        self.watch_cache
            .insert(key.clone(), Arc::new(Semaphore::new(1)));
        return (*self
            .watch_cache
            .get(key)
            .expect("Couldn't get a new semaphore"))
        .clone();
    }

    pub async fn get_or_fetch(&self, key: &K) -> color_eyre::Result<V> {
        let semaphore = self.get_semaphore(key);
        let permit = semaphore.acquire().await.context("Couldn't get permit")?;

        let maybe_data = self.get(key.clone()).await?;
        if let Some(data) = maybe_data {
            return Ok(data);
        }

        self.fetch_and_save_with_permit(key, &permit)
    }

    /// Fetch an item, bypassing the cache. This also save the request. Only one request is allowed at a time, so a Semaphore permit is required. If none is provided, it will get assigned automatically.
    /// ⚠️ Waiting for a permit doesn't cancel the request. It only delays it. If the intention is to only fetch once, see [Self::get_or_fetch]
    pub async fn fetch_and_save(&self, key: K) -> color_eyre::Result<()>  {
        let semaphore = self.get_semaphore(&key);
        let permit = semaphore.acquire().await.context("Couldn't get permit")?;

        self.fetch_and_save_with_permit(key, &permit)
    }

    async fn fetch_and_save_with_permit<'a>(&self, key: K, _permit: &SemaphorePermit<'a>) -> color_eyre::Result<()> {
        let res = V::fetch(key)?;
        
        for entity in res.to_entities() {
            entity.
        }
    }
}