use std::future::Future;

/// For all the entities that are fetchable through their ID
pub trait IsFetchable<K>
where
    Self: Sized,
{
    /// Fetch the entity
    fn fetch(id: K) -> impl Future<Output = color_eyre::Result<Self>>;
}

/// For all the entities that are fetchable through their ID, but returns multiple self
pub trait IsFetchableLike<K>
where
    Self: Sized,
{
    /// Fetch all the entities that have an ID like this one. This is used when the API is forced to be fuzzy
    fn fetch_like(id: K) -> impl Future<Output = color_eyre::Result<Vec<Self>>>;
}