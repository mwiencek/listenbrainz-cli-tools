use core::hash::Hash;
use std::{fs::File, sync::Arc};
use moka::sync::Cache;
use serde::{Deserialize, Serialize};

pub struct DataCache<K, V> {
    name: String,
    cache: Cache<K, V>,
}

impl<K, V> DataCache<K, V>
where
    K: Hash + Eq + Send + Sync  + 'static,
    V: Clone + Send + Sync  + 'static
{
    pub fn new(name: String) -> Self {
        Self {
            name,
            cache: Cache::builder()
                .max_capacity(100 * 1024 * 1024) // This cache will hold up to 100MiB of values.
                .build(),
        }
    }
}

pub fn save_cache<K, V>(cache: Cache<K, V>, name: String) -> color_eyre::Result<()>
where
    K: Hash + Eq + Send + Sync + Serialize + 'static,
    V: Clone + Send + Sync + Serialize + 'static {
    let converted_cache = cache.iter().collect::<Vec<(Arc<K>, V)>>();

    let cache_file = File::create(format!("c:\\test\\{}.json",name))?;

    serde_json::to_writer(cache_file, &converted_cache)?;

    Ok(())
}

pub fn load_cache<'de, K, V>(cache: &mut Cache<K, V> ,name: String) -> color_eyre::Result<()>
where
    K: Hash + Eq + Send + Sync + Deserialize<'de>,
    V: Clone + Send + Sync + Deserialize<'de>  {
    let cache_file = File::open(format!("c:\\test\\{}.json",name))?;

    let cache_data: Vec<(K, V)> = serde_json::from_reader(cache_file)?;
    

    for (key, value) in cache_data.into_iter() {
        cache.insert(key, value);
    }

    Ok(())
}