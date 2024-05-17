use reqwest::Client;

use crate::models::data::musicbrainz::artist::mbid::ArtistMBID;

pub fn browse(entity_type: BrowseEntity, id: String) {
    Client::new().get("https://musicbrainz.org/ws/2/release?artist=47e718e1-7ee4-460c-b1cc-1192a841c6e5")
}

pub enum BrowseEntity {

}
