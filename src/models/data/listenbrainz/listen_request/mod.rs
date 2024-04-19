use chrono::{DateTime, TimeDelta, Utc};
use color_eyre::owo_colors::colors::xterm::DarkMintGreen;
use listenbrainz::raw::{response::UserListensResponse, Client};

pub struct ListenRequest {
    user: String,
    anchor: FetchAnchor
}

impl ListenRequest {
    pub fn fetch(&self) {
        let res =  Client::new().user_listens(
            &self.user, 
            self.anchor.get_after().map(|date| date.timestamp()), 
            self.anchor.get_before().map(|date| date.timestamp()), 
            Some(999)
        );

        
    }
}

pub enum FetchAnchor{
    Latest,
    Before(DateTime<Utc>),
    BeforeInclusive(DateTime<Utc>),
    After(DateTime<Utc>),
    AfterInclusive(DateTime<Utc>)
}

impl FetchAnchor {
    pub fn get_before(&self) -> Option<DateTime<Utc>> {
        match &self {
            FetchAnchor::Before(val) => Some(val.clone()),
            FetchAnchor::BeforeInclusive(val) => Some(val.clone() + TimeDelta::new(1, 0).unwrap()),
            FetchAnchor::Latest => Some(Utc::now()),
            _ => None
        }
    }

    pub fn get_after(&self) -> Option<DateTime<Utc>> {
        match &self {
            FetchAnchor::After(val) => Some(val.clone()),
            FetchAnchor::AfterInclusive(val) => Some(val.clone() - TimeDelta::new(1, 0).unwrap()),
            _ => None
        }
    }
}