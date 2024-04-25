use std::ops::Deref;

use chashmap::CHashMap;

use super::AutoMapItem;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct AutoCHashmap<K, V> {
    data: CHashMap<K, V>
}

impl<K, V> Deref for AutoCHashmap<K, V> {
    type Target = CHashMap<K, V>;

    fn deref(&self) -> &CHashMap<K, V> {
        &self.data
    }
}

impl<K, V> AutoCHashmap<K, V> where V: AutoMapItem<K> {
    pub fn get_or_new(&self, key: &K) -> V {
        match self.data.get(key) {
            Some(val) => val,
            None => {
                let new_item = V::create_from_key(key);
                self.data.insert(new_item);
                self.data.get(key).expect("Couldn't retrieve item just inserted")
            }
        }
    }
}