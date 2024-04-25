use std::sync::Arc;

use chashmap::CHashMap;
use tokio::sync::Semaphore;

use crate::models::utils::auto_hashmap::auto_chashmap::AutoCHashmap;

/// Give and hold semaphores for API requests
pub struct SemaphoreMap {
    semaphores: AutoCHashmap<String, Arc<Semaphore>>
}

impl SemaphoreMap {
    pub fn new(name: String) -> Self {
        Self { semaphores: CHashMap::new()}
    }

    pub fn get_semaphore(&self, key: &K) -> Arc<Semaphore> {
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
}