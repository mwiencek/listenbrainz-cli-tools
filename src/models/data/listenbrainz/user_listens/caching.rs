use chrono::Utc;
use listenbrainz::raw::response::UserListensPayload;

use crate::models::data::listenbrainz::listen_request::FetchAnchor;

use super::UserListens;

impl UserListens {
    /// Remove all the listens in a specific timerange and replace them with payload data.
    ///
    /// ⚠️ This doesn't affect the cache ⚠️
    pub fn refresh_timerange(&mut self, data: UserListensPayload, anchor: FetchAnchor) {
        self.remove_timerange(
            &data
                .get_date_of_oldest_listen_of_payload()
                .unwrap_or(Utc::now()),
            &data
                .get_date_of_latest_listen_of_payload()
                .unwrap_or(Utc::now()),
            true,
        );

        for lb_listen in data.listens {
            self.insert_lb_listen(lb_listen)
        }
    }
}