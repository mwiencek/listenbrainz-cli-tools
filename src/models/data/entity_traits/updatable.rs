use super::insertable::Insertable;

/// Trait for updatable entities 
pub trait Updatable {
    /// Update the current entity with new info
    fn update(&self, new: Self) -> Self;
}

// --------
pub trait InsertableAndUpdatable<K>: Insertable<K> + Updatable {
    /// Insert or update the entity
    async fn insert_or_update() -> color_eyre::Result<()> {

    }
}

// --------

/// Trait for the Overwritable entities
pub trait Overwritable {}

impl<T: Overwritable> Updatable for T {
    fn update(&self, new: Self) -> Self {
        new
    }
}

