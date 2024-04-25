use color_eyre::eyre::{Context, Ok};
use musicbrainz_rs::entity::artist::Artist as ArtistMS;
use musicbrainz_rs::Fetch;

use crate::models::api::api_return::ApiReturn;
use crate::models::data::entity_traits::fetchable::Fetchable;
use crate::models::data::entity_traits::has_cache::HasCache;
use crate::models::data::musicbrainz::artist::Artist;
use crate::utils::println_mus;

impl ApiReturn for ArtistMS {
    fn to_entities(self) -> Vec<Box<impl crate::models::data::entity_traits::has_cache::HasCache<dyn std::fmt::Display>>> {
        let new_artist = Artist::from(self.clone());
        let out: Vec<Box<dyn HasCache>> = vec![Box::new(new_artist)];

        if let Some(recordings) = self.recordings {
            let new_recordings = recordings.into_iter().flat_map(|recording| recording.to_insertables()).collect_vec();
            out.extend(new_recordings)
        }

        out
    }
}

impl Fetchable<String> for Artist {
    async fn fetch(key: String) -> color_eyre::Result<ArtistMS> {
        println_mus(format!("Getting data for artist MBID: {}", &key));
        let msreturn = ArtistMS::fetch()
            .id(&key)
            .with_recordings()
            .execute()
            .await
            .context("Failed to fetch artist from MusicBrainz")?;

            msreturn;
    }
}