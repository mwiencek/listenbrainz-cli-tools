pub mod caching;
pub mod fetching;

use core::fmt;
use std::fmt::{Display, Formatter};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::models::api::HasCacheAndFetchApi;
use crate::models::data::listenbrainz::mapping_data::MappingData;
use crate::models::data::musicbrainz::recording::Recording;

use super::messybrainz::MessyBrainzData;

pub mod collection;
pub mod convertion;

/// The id of a listen. It's a composite of (Username, listened_at, msid)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct ListenId(String, DateTime<Utc>, String);

impl Display for ListenId {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.0, self.1, self.2)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Listen {
    /// The username of the user who listened to it
    pub user: String,

    /// Time of when the listen happened
    pub listened_at: DateTime<Utc>,

    /// Data that have been sent to listenbrainz durring listen submition
    pub messybrainz_data: MessyBrainzData,

    /// Data of the mapping
    pub mapping_data: Option<MappingData>,
}

impl Listen {
    pub fn is_mapped(&self) -> bool {
        self.mapping_data.is_some()
    }

    pub fn get_mapping_data(&self) -> &Option<MappingData> {
        &self.mapping_data
    }

    pub fn get_listened_at(&self) -> &DateTime<Utc> {
        &self.listened_at
    }

    /// If mapped, return the recording MBID
    pub fn get_recording_mbid(&self) -> Option<&String> {
        self.mapping_data
            .as_ref()
            .map(|mapping| &mapping.recording_mbid)
    }

    /// Return true if the listen is mapped to this recording MBID
    pub fn is_mapped_to_recording(&self, mbid: &str) -> bool {
        self.mapping_data
            .as_ref()
            .is_some_and(|mapping| mapping.recording_mbid == mbid)
    }

    /// Return the recording's data from Musicbrainz from its mapping
    pub async fn get_recording_data(&self) -> color_eyre::Result<Option<Recording>> {
        match &self.mapping_data {
            Some(mapping) => Ok(Some(
                Recording::get_cached_or_fetch(mapping.get_recording_id()).await?,
            )),
            None => Ok(None),
        }
    }
}
