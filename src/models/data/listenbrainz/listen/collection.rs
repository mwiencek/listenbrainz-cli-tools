use std::ops::Deref;
use std::sync::Arc;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::Listen;

/// Wrapper for a vector of listens
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ListenCollection {
    data: Vec<Arc<Listen>>,
}

impl ListenCollection {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn get_mapped_listens(&self) -> ListenCollection {
        self.data
            .iter()
            .filter(|element| element.is_mapped())
            .cloned()
            .collect()
    }

    /// Returns the latest listen in the collection.
    pub fn get_latest_listen(&self) -> Option<Arc<Listen>> {
        self.data
            .iter()
            .max_by_key(|listen| listen.listened_at)
            .cloned()
    }

    /// Returns all the unmapped listens
    pub fn get_unmapped_listens(&self) -> ListenCollection {
        self.data
            .iter()
            .filter(|listen| !listen.is_mapped())
            .cloned()
            .collect()
    }

    /// Remove all the listens in between two dates. Start must be earlier than end
    pub fn remove_timerange(
        &mut self,
        start: &DateTime<Utc>,
        end: &DateTime<Utc>,
        inclusive_start: bool,
        inclusive_end: bool,
    ) {
        self.data.retain(|listen| {
            let start_filter = match inclusive_start {
                true => listen.get_listened_at() <= start,
                false => listen.get_listened_at() < start,
            };
            let end_filter = match inclusive_end {
                true => listen.get_listened_at() >= start,
                false => listen.get_listened_at() > start,
            };

            start_filter || end_filter
        })
    }

    /// Remove everything after a date
    pub fn remove_after(&mut self, date: &DateTime<Utc>, inclusive: bool) {
        self.data.retain(|listen| {
            if inclusive {
                listen.get_listened_at() >= date
            } else {
                listen.get_listened_at() > date
            }
        })
    }

    /// Remove everything before a date
    pub fn remove_before(&mut self, date: &DateTime<Utc>, inclusive: bool) {
        self.data.retain(|listen| {
            if inclusive {
                listen.get_listened_at() <= date
            } else {
                listen.get_listened_at() < date
            }
        })
    }

    /// Add a listen to the collection.
    pub fn push(&mut self, listen: Arc<Listen>) {
        self.data.push(listen);
    }
}

impl Deref for ListenCollection {
    type Target = Vec<Arc<Listen>>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl FromIterator<Arc<Listen>> for ListenCollection {
    fn from_iter<T: IntoIterator<Item = Arc<Listen>>>(iter: T) -> Self {
        Self {
            data: iter.into_iter().collect(),
        }
    }
}

impl IntoIterator for ListenCollection {
    type Item = Arc<Listen>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl Default for ListenCollection {
    fn default() -> Self {
        Self::new()
    }
}
