pub mod auto_chashmap;

pub trait AutoMapItem<K> {
    fn create_from_key(key: K) -> Self;
}