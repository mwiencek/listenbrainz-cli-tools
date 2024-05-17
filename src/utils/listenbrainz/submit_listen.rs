use std::collections::HashMap;

use chrono::prelude::DateTime;
use chrono::prelude::Utc;
use clap::builder::Str;
use derive_builder::Builder;
use listenbrainz::raw::request::ListenType;
use listenbrainz::raw::request::Payload;
use listenbrainz::raw::request::SubmitListens;
use listenbrainz::raw::request::TrackMetadata;
use listenbrainz::raw::Client;

#[derive(Debug, Builder)]
pub struct UnsubmittedListen {
    track_name: String,
    artist_name: String,
    listened_at: DateTime<Utc>,
    action: Option<String>,
}

impl UnsubmittedListen {
    pub fn submit(&self) {
        //Client::new().submit_listens(token, data)
    }
}

impl From<UnsubmittedListen> for SubmitListens {
    fn from(value: UnsubmittedListen) -> Self {
        let mut additional_info = HashMap::new();

        if let Some(action) = value.action {
            additional_info.insert("tags", vec![action]);
        }

        Self {
            listen_type: ListenType::Single,
            payload: [
                Payload {
                    listened_at: Some(value.listened_at.timestamp()),
                    track_metadata: TrackMetadata {
                        track_name: value.track_name,
                        artist_name: value.artist_namen
                        release_name: None,
                        additional_info
                    }
                }
            ]
        }
    }
}