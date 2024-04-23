/// For all the values that are insertable into the cache
pub trait IsInsertable<K> {
    // Insert a value into the cache
    fn insert_into_cache(key: K, value: Self) -> color_eyre::Result<()>;
}