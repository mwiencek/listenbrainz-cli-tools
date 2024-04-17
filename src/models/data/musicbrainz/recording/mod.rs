pub mod converters;
pub mod getters;
use crate::models::api::FetchAPI;

use clap::builder::Str;
use color_eyre::eyre::{eyre, Context, OptionExt};
use color_eyre::Result;
use serde::{Deserialize, Serialize};

use super::artist_credit::collection::ArtistCredits;
use super::HasMbid;

pub mod caching;
pub mod fetching;

impl HasMbid for Recording {
    fn get_mbid(&self) -> &str {
        &self.id
    }
}

impl Recording {
    pub fn get_artist_credits(&self) -> Option<ArtistCredits> {
        self.artist_credit.clone()
    }

    pub async fn get_or_fetch_artist_credits(&self) -> Result<ArtistCredits> {
        Ok(match &self.get_artist_credits() {
            Some(credits) => credits.clone(),
            None => {
                Self::fetch_and_insert(&self.get_mbid().to_string())
                .await
                .context("Couldn't fetch data from the API")?
                .get_artist_credits()
                .ok_or_eyre(eyre!(format!("Artist credit is null after fetching from the API. Something wrong happened, as it should return a empty vec. \n Is there an include missing somewhere in the API call? Or is the credit not saved? Faulty requested recording ID is: {}", &self.id)))?
            }
        })
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct Recording {
    pub id: String,
    pub title: String,
    pub artist_credit: Option<ArtistCredits>,
    releases: Option<Vec<String>>
}

