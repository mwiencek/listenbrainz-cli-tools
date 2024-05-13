use crate::core::entity_traits::insertable::Insertable;

use super::artist::Artist;
use super::recording::Recording;
use super::release::Release;
use super::release_group::ReleaseGroup;
use derive_more::*;

#[derive(IsVariant, Unwrap)]
pub enum MSEntity {
    Artist(Artist),
    Recording(Recording),
    Release(Release),
    ReleaseGroup(ReleaseGroup),
}

impl Insertable for MSEntity {
    async fn insert_into_cache_as(&self, key: String) -> color_eyre::Result<()> {
        match self {
            Self::Artist(val) => val.insert_into_cache_as(key).await,
            Self::Recording(val) => val.insert_into_cache_as(key).await,
            Self::Release(val) => val.insert_into_cache_as(key).await,
            Self::ReleaseGroup(val) => val.insert_into_cache_as(key).await,
        }
    }
}
