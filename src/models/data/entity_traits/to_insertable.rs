use std::fmt::Display;

use super::{has_cache::HasCache, insertable::Insertable};

/// Symbolise an entity that can be made into an insertable one
pub trait ToInsertable<T: HasCache<dyn Display>> {
    // Convert the entity to an insertable one
    fn to_insertable(self) -> T;
}

// -------

/// Symbolise an entity that can be made into multiple insertable one
pub trait ToInsertables<T: HasCache<dyn Display>> {
    // Convert the entity to an insertables one
    fn to_insertables(self) -> Vec<Box<impl HasCache<dyn Display>>>;
}

impl<T: Insertable<dyn Display>, V: ToInsertable<T>> ToInsertables<T> for V {
    fn to_insertables(self) -> Vec<Box<impl HasCache<dyn Display>>> {
        vec![self.to_insertable()]
    }
}