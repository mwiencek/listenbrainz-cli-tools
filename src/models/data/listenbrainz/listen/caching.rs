use chrono::{DateTime, Utc};

use crate::models::cache::{
    global_cache::GlobalCache,
    traits::{has_cache::HasCache, has_id::HasID, merge::UpdateCachedEntity},
};

use super::{Listen, ListenId};

impl HasID<ListenId> for Listen {
    fn get_id(&self) -> ListenId {
        (
            self.user.to_owned(),
            *self.get_listened_at(),
            self.messybrainz_data.msid.clone(),
        )
    }
}

impl UpdateCachedEntity for Listen {
    fn update_entity(self, new: Self) -> Self {
        new
    }
}

impl HasCache<ListenId> for Listen {
    fn get_cache(
    ) -> std::sync::Arc<crate::models::cache::disk_cache::DiskCacheWrapper<ListenId, Self>> {
        GlobalCache::new().get_listen_cache()
    }
}
