use chrono::{DateTime, Utc};

use crate::models::cache::traits::{has_id::{HasID}, merge::UpdateCachedEntity};

use super::Listen;

impl HasID<(String, DateTime<Utc>, String)> for Listen {
    fn get_id(&self) -> (String, DateTime<Utc>, String) {
        (self.user.to_owned(), self.get_listened_at().clone(), self.messybrainz_data.msid.clone())
    }
}

impl UpdateCachedEntity for Listen {
    fn update_entity(self, new: Self) -> Self {
        new
    }
}