use musicbrainz_rs::entity::release::Release;

use crate::core::entity_traits::{
    has_id::HasID,
    insertable::{Insertable, IsAutoInsertable},
    insertable_children::InsertableWithChildren,
};

impl InsertableWithChildren for Release {
    async fn insert_with_children(&self, key: String) -> color_eyre::Result<()> {
        self.insert_into_cache_as(key).await?;

        if let Some(data) = self.artist_credit.clone() {
            for item in data.iter() {
                item.insert_into_cache().await?;
            }
        }

        if let Some(data) = self.media.clone() {
            for item in data.iter() {
                item.insert_into_cache().await?;
            }
        }

        Ok(())
    }
}

impl HasID for Release {
    fn get_id(&self) -> String {
        self.id.to_string()
    }
}