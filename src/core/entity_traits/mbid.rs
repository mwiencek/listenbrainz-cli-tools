use extend::ext;
use crate::models::data::entity_database::{ENTITY_DATABASE, EntityDatabase};

pub trait MBID<T> {
    fn get_or_fetch_entity(
        &self,
    ) -> impl std::future::Future<Output = color_eyre::Result<T>> + Send;
    
    fn get_main_alias(&self) -> impl std::future::Future<Output = color_eyre::Result<Self>> + Send {
        async {
            ENTITY_DATABASE.mbid_aliases().clone().
        }
    }
    
}

#[ext]
pub impl<T, I: MBID<T>> Vec<I> {
    #[allow(async_fn_in_trait)]
    async fn get_or_fetch_entities(&self) -> color_eyre::Result<Vec<T>> {
        let mut result = Vec::new();

        for item in self {
            result.push(item.get_or_fetch_entity().await?);
        }

        Ok(result)
    }
}
