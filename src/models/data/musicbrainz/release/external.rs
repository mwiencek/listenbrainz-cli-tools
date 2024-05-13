use musicbrainz_rs::entity::release::Release;

use crate::core::entity_traits::cached::Cached;
use crate::core::entity_traits::has_id::HasID;
use crate::core::entity_traits::insertable::{Insertable, IsAutoInsertable};
use crate::core::entity_traits::insertable_children::InsertableWithChildren;
use crate::core::entity_traits::into_ms_entities::IntoMSEntities;
use crate::models::data::musicbrainz::entity_enum::MSEntity;

impl HasID for Release {
    fn get_id(&self) -> String {
        self.id.to_string()
    }
}

impl Insertable for Release {
    async fn insert_into_cache_as(&self, key: String) -> color_eyre::Result<()> {
        crate::models::data::musicbrainz::release::Release::get_cache()
            .update(&key, self.clone().into())
            .await?;

        Ok(())
    }
}

impl InsertableWithChildren for Release {
    async fn insert_with_children(&self, key: String) -> color_eyre::Result<()> {
        self.insert_into_cache_as(key).await?;

        if let Some(data) = self.artist_credit.clone() {
            for item in &data {
                item.insert_into_cache().await?;
            }
        }

        if let Some(data) = self.media.clone() {
            for item in &data {
                item.insert_into_cache().await?;
            }
        }

        if let Some(data) = self.release_group.clone() {
            data.insert_into_cache().await?;
        }

        Ok(())
    }
}

impl IntoMSEntities for Release {
    fn into_ms_entities(self) -> Vec<MSEntity> {
        let mut results = vec![MSEntity::Release(self.clone().into())];

        results.extend(self.artist_credit.into_ms_entities());
        results.extend(self.release_group.into_ms_entities());

        results
    }
}
