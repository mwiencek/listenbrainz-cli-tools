use crate::core::entity_traits::cached::Cached;
use crate::core::entity_traits::has_id::HasID;
use crate::core::entity_traits::insertable::{Insertable, IsAutoInsertable};
use crate::core::entity_traits::insertable_children::InsertableWithChildren;
use crate::core::entity_traits::into_ms_entities::IntoMSEntities;
use crate::models::data::musicbrainz::entity_enum::MSEntity;
use musicbrainz_rs::entity::release_group::ReleaseGroup;

impl HasID for ReleaseGroup {
    fn get_id(&self) -> String {
        self.id.to_string()
    }
}

impl Insertable for ReleaseGroup {
    async fn insert_into_cache_as(&self, key: String) -> color_eyre::Result<()> {
        crate::models::data::musicbrainz::release_group::ReleaseGroup::get_cache()
            .update(&key, self.clone().into())
            .await
    }
}

impl InsertableWithChildren for ReleaseGroup {
    async fn insert_with_children(&self, key: String) -> color_eyre::Result<()> {
        self.insert_into_cache_as(key).await?;

        if let Some(releases) = self.releases.clone() {
            for release in releases {
                release.insert_into_cache().await?;
            }
        }

        Ok(())
    }
}

impl IntoMSEntities for ReleaseGroup {
    fn into_ms_entities(self) -> Vec<MSEntity> {
        let mut results = vec![MSEntity::ReleaseGroup(self.clone().into())];

        results.extend(self.artist_credit.into_ms_entities());
        results.extend(self.releases.into_ms_entities());

        results
    }
}
