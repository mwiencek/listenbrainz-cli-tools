use std::sync::Arc;

use musicbrainz_rs::entity::release_group::ReleaseGroup;

use crate::models::cache::{disk_cache::DiskCacheWrapper, global_cache::GlobalCache, traits::{has_cache::HasCache, merge::UpdateCachedEntity}};

//impl HasCache<String, ReleaseGroup> for ReleaseGroup {
//    fn get_cache() -> Arc<DiskCacheWrapper<String, ReleaseGroup>> {
//        GlobalCache::new().get_release_group_cache()
//    }
//}
//
impl UpdateCachedEntity for ReleaseGroup {
    fn update_entity(self, new: Self) -> Self {
        Self {
            aliases: new.aliases.or(new.aliases),
            annotation: new.annotation.or(new.annotation),
            artist_credit: new.artist_credit.or(new.artist_credit),
            disambiguation: new.disambiguation.or(new.disambiguation),
            first_release_date: new.first_release_date.or(new.first_release_date),
            genres: new.genres.or(new.genres),
            id: new.id.or(new.id),
            primary_type: new.primary_type.or(new.primary_type),
            primary_type_id: new.primary_type_id.or(new.primary_type_id),
            rating: new.rating.or(new.rating),
            relations: new.relations.or(new.relations),
            releases: new.releases.or(new.releases),
            secondary_type_ids: new.secondary_type_ids.or(new.secondary_type_ids),
            secondary_types: new.secondary_types.or(new.secondary_types),
            tags: new.tags.or(new.tags),
            title: new.title.or(new.title)
        }
    }
}
