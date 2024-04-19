pub mod has_cache_and_fetch_api;
pub mod has_id_and_cache;
pub mod has_id;
pub mod has_cache;
use color_eyre::Result;

pub mod merge;

pub trait InsertExternalEntityIntoCache<T, E: Into<T>> {
    fn insert_ext_into_cache(value: E) -> Result<()>;

    fn insert_ext_iter_into_cache<I: IntoIterator<Item = E>>(values: I) -> Result<()> {
        values
            .into_iter()
            .try_for_each(|value| Self::insert_ext_into_cache(value))
    }

    fn insert_opt_ext_into_cache(value: Option<E>) -> Result<()> {
        match value {
            Some(value) => Self::insert_ext_into_cache(value),
            None => Ok(()),
        }
    }

    fn insert_opt_ext_iter_into_cache<I: IntoIterator<Item = E>>(values: Option<I>) -> Result<()> {
        match values {
            Some(values) => Self::insert_ext_iter_into_cache(values),
            None => Ok(()),
        }
    }
}
