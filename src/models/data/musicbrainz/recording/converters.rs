use itertools::Itertools;

use super::Recording;

impl From<musicbrainz_rs::entity::recording::Recording> for Recording {
    fn from(recording: musicbrainz_rs::entity::recording::Recording) -> Self {
        Self {
            id: recording.id,
            title: recording.title,
            artist_credit: recording.artist_credit.map(|coll| coll.into()),
            releases: recording.releases.map(|releases| releases.into_iter().map(|release| release.id).collect_vec())
        }
    }
}
