use musicbrainz_rs::entity::release::Release;
use serde::{Deserialize, Serialize};

use super::{artist::Artist, recording::Recording};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(tag = "type")]
pub enum EntityType {
    Artist(Box<Artist>),
    Recording(Box<Recording>),
    Release(Box<Release>),
}