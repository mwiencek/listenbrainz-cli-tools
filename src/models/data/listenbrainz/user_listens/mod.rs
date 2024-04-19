pub mod caching;
use std::sync::Arc;

use chrono::{DateTime, Utc};
use listenbrainz::raw::response::{UserListensListen, UserListensPayload};
use serde::{Deserialize, Serialize};

use crate::utils::extensions::UserListensPayloadExt;

use super::listen::collection::ListenCollection;
use super::listen::Listen;
use super::listen_request::FetchAnchor;

pub mod fetching;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserListens {
    username: String,
    listens: ListenCollection,
}

impl UserListens {
    pub fn new(user: &str) -> Self {
        Self {
            username: user.to_lowercase(),
            listens: ListenCollection::new(),
        }
    }

    pub fn get_user(&self) -> &str {
        &self.username
    }

    pub fn get_latest_listen(&self) -> Option<Arc<Listen>> {
        self.listens.get_latest_listen()
    }

    /// Insert a listen into the struct.
    ///
    /// ⚠️ This doesn't affect the cache ⚠️
    pub fn insert_listen(&mut self, listen: Listen) {
        self.listens.push(Arc::new(listen));
    }

    

    /// Remove all the listens in a specific timerange. This is a dangerous function as it can mess with data integrity
    ///
    /// ⚠️ This doesn't affect the cache ⚠️
    pub fn remove_timerange(
        &mut self,
        start: &DateTime<Utc>,
        end: &DateTime<Utc>,
        inclusive: bool,
    ) {
        self.listens.remove_timerange(start, end, inclusive)
    }

    /// Uncached and unchecked insert of a listenbrain listen into the struct
    ///
    /// ⚠️ This doesn't affect the cache ⚠️
    pub fn insert_lb_listen(&mut self, data: UserListensListen) {
        self.listens.push(Arc::new(data.into()))
    }

    /// Returns all the unmapped listens
    pub fn get_unmapped_listens(&self) -> ListenCollection {
        self.listens.get_unmapped_listens()
    }

    /// Returns all the mapped listens
    pub fn get_mapped_listens(&self) -> ListenCollection {
        self.listens.get_mapped_listens()
    }

    /// Returns the number of listens
    pub fn len(&self) -> usize {
        self.listens.len()
    }

    /// Returns true if there is no listens
    pub fn is_empty(&self) -> bool {
        self.listens.is_empty()
    }
}
