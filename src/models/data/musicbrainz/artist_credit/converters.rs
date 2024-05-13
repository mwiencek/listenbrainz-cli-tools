use musicbrainz_rs::entity::artist_credit::ArtistCredit as ArtistCreditMS;

use crate::core::entity_traits::into_ms_entities::IntoMSEntities;
use crate::models::data::musicbrainz::entity_enum::MSEntity;

use super::ArtistCredit;

impl From<ArtistCreditMS> for ArtistCredit {
    fn from(value: ArtistCreditMS) -> Self {
        Self {
            artist: value.artist.id.into(),
            joinphrase: value.joinphrase,
            name: value.name,
        }
    }
}

impl IntoMSEntities for ArtistCreditMS {
    fn into_ms_entities(self) -> Vec<MSEntity> {
        let results = vec![MSEntity::Artist(self.artist.into())];

        results
    }
}
