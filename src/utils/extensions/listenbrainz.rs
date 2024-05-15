use extend::ext;
use listenbrainz::raw::response::UserListensResponse;
use listenbrainz::raw::response::UserListensTrackMetadata;
use listenbrainz::raw::response::UserListensListen;
use listenbrainz::raw::Client;


#[ext]
pub impl UserListensTrackMetadata {
    fn get_additional_string_metadata(&self, key: &str) -> Option<&String> {
        let data = self.additional_info.get(key)?;
        match data {
            serde_json::Value::String(data) => Some(data),
            _ => None,
        }
    }
}

#[ext]
pub impl UserListensListen {
    fn get_id(&self) -> ListenID {
        (self.username, self.recording_msid. Utc
            .timestamp_opt(self.listened_at, 0)
            .single()
            .expect("Cannot convert listened_at timestamp. This shouldn't happen since all the dates are UTC!");)
    }
}

#[ext]
pub impl Client {
    fn fetch_listens(
        &self,
        username: &str,
        fetch_date: FetchDateDirection,
        count: Option<u64>,
    ) -> Result<UserListensResponse, listenbrainz::Error> {
        match fetch_date.is_max_ts() {
            true => self.user_listens(
                username,
                None,
                Some(fetch_date.get_exclusive_timestamp().try_into().unwrap()),
                count,
            ),
            false => self.user_listens(
                username,
                Some(fetch_date.get_exclusive_timestamp().try_into().unwrap()),
                None,
                count,
            ),
        }
    }
}

pub enum FetchDateDirection {
    Before(i64),
    BeforeInclusive(i64),

    After(i64),
    AfterInclusive(i64),
}

impl FetchDateDirection {
    pub fn get_exclusive_timestamp(&self) -> i64 {
        match self {
            Self::After(val) | Self::Before(val) => val.clone(),
            Self::AfterInclusive(val) => val - 1,
            Self::BeforeInclusive(val) => val + 1,
        }
    }

    pub fn is_max_ts(&self) -> bool {
        match self {
            Self::After(_) | Self::AfterInclusive(_) => false,
            Self::Before(_) | Self::BeforeInclusive(_) => true,
        }
    }
}
