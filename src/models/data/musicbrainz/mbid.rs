use derive_more::{IsVariant, Unwrap};
use serde::{Deserialize, Serialize};
use crate::models::data::musicbrainz::artist::mbid::ArtistMBID;
use crate::models::data::musicbrainz::recording::mbid::RecordingMBID;
use crate::models::data::musicbrainz::release::mbid::ReleaseMBID;
use crate::models::data::musicbrainz::release_group::mbid::ReleaseGroupMBID;

#[derive(Eq, PartialEq, Serialize, Deserialize, Debug, IsVariant, Unwrap)]
pub enum MBID {
    Artist(ArtistMBID),
    ReleaseGroup(ReleaseGroupMBID),
    Release(ReleaseMBID),
    Recording(RecordingMBID),
}