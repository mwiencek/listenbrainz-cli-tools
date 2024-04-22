/// Trait for all the entities that have IDs
pub trait HasID<K> {
    fn get_id(&self) -> K;
}
